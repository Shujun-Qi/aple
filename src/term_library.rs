use crate::ast::{self, Sym, Principal};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct TermId(usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct ArgId(usize);

#[derive(Debug, Clone)]
pub struct TermLibrary {
    terms: Vec<Term>,
    args: Vec<TermId>,
}

impl TermLibrary {
    pub fn new() -> Self {
        Self {
            terms: vec![],
            args: vec![],
        }
    }

    pub fn principal(&mut self, pcpl: Principal) -> TermId {
        let term = TermId(self.terms.len());
        self.terms.push(Term::Principal(pcpl));
        term
    }

    pub fn predicate(&mut self, relation: Sym, args: &[TermId]) -> TermId {
        let term = TermId(self.terms.len());
        let arg_start = self.args.len();
        let arg_end = arg_start + args.len();
        self.args.extend_from_slice(args);
        self.terms.push(Term::Pred(
            relation,
            ArgRange {
                start: arg_start,
                end: arg_end,
            },
        ));
        term
    }

    pub fn extend_library(
        &mut self,
        lib: &TermLibrary,
        pcpl_offset: usize,
    ) -> impl Fn(TermId) -> TermId {
        let here = self.checkpoint();
        self.terms
            .extend(lib.terms.iter().map(|term| match term {
                Term::Principal(pcpl) => Term::Principal(pcpl.offset(pcpl_offset)),
                Term::Pred(relation, args) => Term::Pred(
                    *relation,
                    ArgRange {
                        start: args.start + here.args,
                        end: args.end + here.args,
                    },
                ),
            }));
        self.args.extend(
            lib.args.iter().map(|term_id| TermId(term_id.0 + here.terms)),
        );

        let term_offset = here.terms;
        move |TermId(old)| TermId(old + term_offset)
    }

    
    pub fn insert_ast_term(&mut self, ids: &mut Vec<TermId>, term: &ast::Term) -> TermId {
        match term {
            ast::Term::Principal(pcpl) => self.principal(*pcpl),
            ast::Term::Pred(pred) => self.insert_ast_predicate(ids, pred),
        }
    }

    pub fn insert_ast_predicate(&mut self, ids: &mut Vec<TermId>, pred: &ast::Predicate) -> TermId {
        let args_start = ids.len();
        for arg in &pred.args {
            let pred_id = self.insert_ast_term(ids, arg);
            ids.push(pred_id);
        }
        let out = self.predicate(pred.relation, &ids[args_start..]);
        ids.truncate(args_start);
        out
    }

    #[inline]
    pub fn get_arg(&self, arg_id: ArgId) -> TermId {
        self.args[arg_id.0]
    }

    #[inline]
    pub fn get_term(&self, term_id: TermId) -> Term {
        self.terms[term_id.0]
    }

    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            terms: self.terms.len(),
            args: self.args.len(),
        }
    }

    pub fn release(&mut self, checkpoint: &Checkpoint) {
        debug_assert!(checkpoint.args <= self.args.len() && checkpoint.terms <= self.terms.len());
        self.args.truncate(checkpoint.args);
        self.terms.truncate(checkpoint.terms);
    }
}

impl Default for TermLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Checkpoint {
    terms: usize,
    args: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ArgRange {
    start: usize,
    end: usize,
}

impl Iterator for ArgRange {
    type Item = ArgId;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.start;
        if start == self.end {
            None
        } else {
            self.start += 1;
            Some(ArgId(start))
        }
    }

    #[inline]
    fn any<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        (self.start..self.end).any(move |x| f(ArgId(x)))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ArgRange {
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Term {
    Principal(Principal),
    Pred(Sym, ArgRange),
}