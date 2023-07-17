use crate::{
    ast::*,
    term_library::{self, TermLibrary},
};

#[derive(Debug)]
pub struct Universe {
    next_id: usize,
    rules: Vec<Rule>,
    compiled_rules: CompiledRuleDb,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            rules: vec![],
            compiled_rules: CompiledRuleDb::new(),
        }
    }

    pub fn alloc_symbol(&mut self) -> Sym {
        let sym = Sym::from_ord(self.next_id);
        self.next_id += 1;
        sym
    }

    pub fn alloc_symbols(&mut self, count: usize) -> impl Iterator<Item = Sym> {
        let fresh_start = self.next_id;
        self.next_id += count;
        (fresh_start..fresh_start + count).map(Sym::from_ord)
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.compiled_rules.insert(&rule);
        self.rules.push(rule);
    }

    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }

    pub fn num_symbols(&self) -> usize {
        self.next_id
    }

    pub fn compiled_rules(&self) -> &CompiledRuleDb {
        &self.compiled_rules
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CompiledRule {
    main_rule_library: TermLibrary,
    main_rule: term_library::TermId,
    sub_rule_library: TermLibrary,
    sub_rule: Vec<term_library::TermId>,
    size: usize,
}

impl CompiledRule {
    pub fn new(rule: &Rule) -> CompiledRule {
        let mut temp = Vec::new();
        let mut main_rule_library = TermLibrary::new();
        let mut sub_rule_library = TermLibrary::new();
        let main_rule = main_rule_library.insert_ast_predicate(&mut temp, &rule.main_rule);
        let sub_rule = rule
            .sub_rules
            .iter()
            .map(|sr| sub_rule_library.insert_ast_predicate(&mut temp, sr))
            .collect();
        CompiledRule {
            main_rule_library,
            main_rule,
            sub_rule_library,
            sub_rule,
            size: rule.main_rule.count_size().max(
                rule.sub_rules
                    .iter()
                    .map(|sr| sr.count_size())
                    .max()
                    .unwrap_or(0),
            ),
        }
    }

    #[inline(always)]
    pub fn main_rule(&self) -> (term_library::TermId, &TermLibrary) {
        (self.main_rule, &self.main_rule_library)
    }

    #[inline(always)]
    pub fn sub_rule(&self) -> (&[term_library::TermId], &TermLibrary) {
        (&self.sub_rule, &self.sub_rule_library)
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct CompiledRuleDb {
    rules_by_main: Vec<Vec<CompiledRule>>,
}

impl CompiledRuleDb {
    pub fn new() -> Self {
        Self {
            rules_by_main: Vec::new(),
        }
    }

    pub fn new_with_size(size: usize) -> Self {
        Self {
            rules_by_main: vec![Vec::new(); size],
        }
    }

    pub fn insert(&mut self, rule: &Rule) {
        self.resize(rule.main_rule.relation);
        let compiled = CompiledRule::new(rule);
        self.rules_by_main[rule.main_rule.relation.ord()].push(compiled);
    }

    #[inline(always)]
    pub fn rules_by_main(&self, main_rule: Sym) -> &[CompiledRule] {
        if main_rule.ord() < self.rules_by_main.len() {
            &self.rules_by_main[main_rule.ord()]
        } else {
            &[]
        }
    }

    fn resize(&mut self, sym: Sym) {
        if sym.ord() >= self.rules_by_main.len() {
            self.rules_by_main.resize(sym.ord() + 1, Vec::new());
        }
    }
}

impl Default for CompiledRuleDb {
    fn default() -> Self {
        Self::new()
    }
}