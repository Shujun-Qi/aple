
#[cfg(test)]
mod test;
// mod compiler;
mod signature;

use crate::{
    types::{self, Query, Sym, Principal},
    term_library::{self, TermLibrary},
    universe::{CompiledRule, CompiledRuleDb, Universe},
};

pub fn query_dfs<'a>(universe: &'a Universe, query: &Query) -> SolutionIter<'a> {
    
    let max_pcpl = query.count_size();
    let mut solution = SolutionState::new(max_pcpl);
    let mut temp = Vec::new();

    SolutionIter {
        rules: universe.compiled_rules(),
        unresolved_goals: query
            .goals
            .iter()
            .rev() 
            .map(|pred| solution.lib.insert_ast_predicate(&mut temp, pred))
            .collect(),
        checkpoints: vec![],
        solution,
    }
}
#[derive(Debug)]
pub struct SolutionIter<'s> {
    rules: &'s CompiledRuleDb,
    unresolved_goals: Vec<term_library::TermId>,
    pub checkpoints: Vec<Checkpoint>,
    solution: SolutionState,
}
#[derive(Debug)]
pub struct Checkpoint {
    goal: term_library::TermId,
    main_rule: Sym,
    alternative: usize,
    goals_checkpoint: usize,
    solution_checkpoint: SolutionCheckpoint,
}

impl Checkpoint {
    pub fn get_solution_checkpoint(&self) -> &SolutionCheckpoint {
        &self.solution_checkpoint
    }
}

pub enum Step {
    Yield,
    Precompiled,
    Continue,
    Done,
}

impl<'s> SolutionIter<'s> {
  
    pub fn step(&mut self) -> Step {
        if let Some(goal) = self.unresolved_goals.pop() {
            let main_rule = match self.solution.lib.get_term(goal) {
                term_library::Term::Principal(_) => unreachable!(),
                term_library::Term::Pred(relation, _) => relation,
            };

            self.checkpoints.push(Checkpoint {
                goal,
                main_rule,
                alternative: 0,
                solution_checkpoint: self.solution.checkpoint(),
                goals_checkpoint: self.unresolved_goals.len(),
            });
        }
        // println!("checkpoint: {:?}", self.checkpoints);
        return self.backtrack_resume();
    }

    pub fn get_solution(&self) -> Vec<Option<types::Term>> {
        self.solution.get_solution()
    }

    pub fn get_checkpoints(&self) -> & Vec<Checkpoint> {
        &self.checkpoints
    }

    pub fn solution_state(&self) -> bool{
        if self.unresolved_goals.len() > 0{
            false
        }
        else{
            true
        }
    }

    pub fn get_unresolved(& self) -> &Vec<term_library::TermId>{
        &self.unresolved_goals
    }

    fn resume_checkpoint(&mut self) -> bool {
        let checkpoint = self
            .checkpoints
            .last_mut()
            .expect("invariant: there is always a checkpoint when this is called");
        let rules = self.rules.rules_by_main(checkpoint.main_rule);
        while let Some(current) = rules.get(checkpoint.alternative) {
            checkpoint.alternative += 1;
            let result = self.solution.unify_rule(checkpoint.goal, current);
            if let Some(goals) = result {
                self.unresolved_goals.extend(goals);
                return true;
            } else {
                drop(result);
                self.solution.restore(&checkpoint.solution_checkpoint);
            }
        }
        
        let discarded = self.checkpoints.pop().expect("we know there is one here");
        self.unresolved_goals.push(discarded.goal);
        false
    }

    fn backtrack_resume(&mut self) -> Step {
        while let Some(checkpoint) = self.checkpoints.last() {
            self.solution.restore(&checkpoint.solution_checkpoint);
            self.unresolved_goals.truncate(checkpoint.goals_checkpoint);
            // match checkpoint.main_rule.ord(){
            //     4 => {
            //         return Step::Precompiled;
            //     }
            //     _ => {}
            // }
            if self.resume_checkpoint() {
                if self.unresolved_goals.is_empty() {
                    return Step::Yield;
                } else {
                    return Step::Continue;
                }
            }
        }
        return Step::Done;
    }
}

impl<'s> Iterator for SolutionIter<'s> {
    type Item = Vec<Option<types::Term>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.step() {
                Step::Yield => break Some(self.get_solution()),
                Step::Precompiled => continue,
                Step::Continue => continue,
                Step::Done => break None,
            }
        }
    }
}

#[derive(Debug)]
struct SolutionState {
    map: Vec<Option<term_library::TermId>>,
    assignments: Vec<Principal>,
    goal_nums: usize,
    lib: TermLibrary,
    occurs_stack: Vec<term_library::TermId>,
}

#[derive(Debug)]
pub struct SolutionCheckpoint {
    assign_checkpoint: usize,
    map_checkpoint: usize,
    lib_checkpoint: term_library::Checkpoint,
}

impl SolutionState {

    fn new(goal_nums: usize) -> Self {
        Self {
            assignments: vec![],
            map: vec![None; goal_nums],
            goal_nums,
            lib: TermLibrary::new(),
            occurs_stack: Vec::new(),
        }
    }

    fn allocate_map(&mut self, nums: usize) {
        self.map.resize(self.map.len() + nums, None);
    }

    fn set_principal(&mut self, pcpl: Principal, value: term_library::TermId) -> bool {
        debug_assert!(self.map[pcpl.ord()].is_none());

        if self.occurs(pcpl, value) {
            return false;
        }

        self.map[pcpl.ord()] = Some(value);
        self.assignments.push(pcpl);

        true
    }

    fn occurs(&mut self, pcpl: Principal, mut term: term_library::TermId) -> bool {
        loop {
            match self.lib.get_term(term) {
                term_library::Term::Principal(p) => {
                    if p == pcpl {
                        self.occurs_stack.clear();
                        return true;
                    }
                }
                term_library::Term::Pred(_, args) => {
                    let lib = &self.lib;
                    self.occurs_stack
                        .extend(args.map(|arg_id| lib.get_arg(arg_id)))
                }
            }
            match self.occurs_stack.pop() {
                Some(next) => term = next,
                None => return false,
            }
        }
    }

    fn checkpoint(&self) -> SolutionCheckpoint {
        SolutionCheckpoint {
            assign_checkpoint: self.assignments.len(),
            map_checkpoint: self.map.len(),
            lib_checkpoint: self.lib.checkpoint(),
        }
    }

    fn restore(&mut self, checkpoint: &SolutionCheckpoint) {
        for pcpl in self.assignments.drain(checkpoint.assign_checkpoint..) {
            self.map[pcpl.ord()] = None;
        }
        self.map.truncate(checkpoint.map_checkpoint);
        self.lib.release(&checkpoint.lib_checkpoint);
    }

    fn get_solution_term(&self, term: term_library::TermId) -> types::Term {
        match self.lib.get_term(term) {
            term_library::Term::Principal(p) => {
                if let Some(value) = &self.map[p.ord()] {
                    self.get_solution_term(*value)
                } else {
                    types::Term::Principal(p)
                }
            }
            term_library::Term::Pred(relation, args) => types::Term::Pred(types::Predicate {
                relation,
                args: args
                    .map(|arg_id| self.get_solution_term(self.lib.get_arg(arg_id)))
                    .collect(),
            }),
        }
    }

    fn get_solution(&self) -> Vec<Option<types::Term>> {
        self.map
            .iter()
            .take(self.goal_nums)
            .map(|val| val.as_ref().map(|t| self.get_solution_term(*t)))
            .collect()
    }

    fn follow_pcpls(&self, mut term: term_library::TermId) -> (term_library::TermId, term_library::Term) {
        loop {
            match self.lib.get_term(term) {
                term_library::Term::Principal(pcpl) => {
                    if let Some(value) = &self.map[pcpl.ord()] {
                        term = *value;
                    } else {
                        return (term, term_library::Term::Principal(pcpl));
                    }
                }
                pred @ term_library::Term::Pred(_, _) => return (term, pred),
            }
        }
    }

    fn unify(&mut self, goal_term: term_library::TermId, rule_term: term_library::TermId) -> bool {
        let (goal_term_id, goal_term) = self.follow_pcpls(goal_term);
        let (rule_term_id, rule_term) = self.follow_pcpls(rule_term);
        println!("unify {:?} {:?}", goal_term, rule_term);
        match (goal_term, rule_term) {
            (term_library::Term::Principal(goal_pcpl), term_library::Term::Principal(rule_pcpl)) => {
                if goal_pcpl != rule_pcpl {
                    self.set_principal(rule_pcpl, goal_term_id)
                } else {
                    true
                }
            }
            (term_library::Term::Principal(goal_pcpl), term_library::Term::Pred(_, _)) => {
                self.set_principal(goal_pcpl, rule_term_id)
            }
            (term_library::Term::Pred(_, _), term_library::Term::Principal(rule_pcpl)) => {
                self.set_principal(rule_pcpl, goal_term_id)
            }
            (
                term_library::Term::Pred(goal_relation, goal_args),
                term_library::Term::Pred(rule_relation, rule_args),
            ) => {
                println!("unify {:?} {:?} {:?} {:?}", goal_relation, goal_args, rule_relation, rule_args);
                match goal_relation.ord(){
                    4 => {
                        println!("unify {:?} {:?} {:?} {:?}", goal_relation, goal_args, rule_relation, rule_args);
                        return true;
                    }
                    _ => {}
                }
                if goal_relation != rule_relation {
                    return false;
                }
                if goal_args.len() != rule_args.len() {
                    return false;
                }
                goal_args.zip(rule_args).all(|(goal_arg, rule_arg)| {
                    self.unify(self.lib.get_arg(goal_arg), self.lib.get_arg(rule_arg))
                })
            }
        }
    }
    
    fn unify_rule<'a>(
        &mut self,
        goal_term: term_library::TermId,
        rule: &'a CompiledRule,
    ) -> Option<impl Iterator<Item = term_library::TermId> + 'a> {
        let size = self.map.len();
        self.allocate_map(rule.size());

        let (main_rule, main_rule_library) = rule.main_rule();
        let conv_rule_main = self.lib.extend_library(main_rule_library, size);
        let new_main_rule = conv_rule_main(main_rule);
        // println!("unify {:?} {:?}", self.lib.get_term(goal_term), self.lib.get_term(new_main_rule));
        if self.unify(goal_term, new_main_rule) {
            let (sub_rule, sub_rule_library) = rule.sub_rule();
            let conv_sub_rule = self.lib.extend_library(sub_rule_library, size);
            Some(sub_rule.iter().rev().map(move |sr| conv_sub_rule(*sr)))
        } else {
            None
        }
    }
}