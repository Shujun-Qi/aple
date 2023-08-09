// in aple, any facts/rules/predicates/principals are represented as a tree of terms
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sym(usize);

impl Sym {
    #[inline(always)]
    pub fn ord(self) -> usize {
        self.0
    }
    #[inline(always)]
    pub fn from_ord(ord: usize) -> Sym {
        Sym(ord)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Principal(usize);

impl Principal {
    #[inline(always)]
    pub fn ord(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn from_ord(ord: usize) -> Principal {
        Principal(ord)
    }

    pub fn offset(self, offset: usize) -> Principal {
        Principal(self.0 + offset)
    }
}

// a term is the basic unit of a logic expression in aple, it can be a principal or a predicate
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Principal(Principal),
    Pred(Predicate),
}

impl Term {
    pub fn count_size(&self) -> usize {
        match self {
            Term::Principal(pcpl) => pcpl.0 + 1,
            Term::Pred(pred) => pred.count_size(),
        }
    }
}

impl From<Principal> for Term {
    fn from(pcpl: Principal) -> Self {
        Term::Principal(pcpl)
    }
}

impl From<Sym> for Term {
    fn from(s: Sym) -> Self {
        Term::Pred(s.into())
    }
}

impl From<Predicate> for Term {
    fn from(at: Predicate) -> Self {
        Term::Pred(at)
    }
}

// a predicate is a relation with a list of arguments
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub relation: Sym,
    pub args: Vec<Term>,
}

impl From<Sym> for Predicate {
    fn from(s: Sym) -> Self {
        Self {
            relation: s,
            args: vec![],
        }
    }
}

impl Predicate {
    pub fn new(relation: Sym, args: Vec<Term>) -> Self {
        Self { relation, args }
    }

    pub fn count_size(&self) -> usize {
        self.args
            .iter()
            .map(|t| t.count_size())
            .max()
            .unwrap_or(0)
    }
}

pub fn pred(relation: Sym, args: Vec<Term>) -> Term {
    Term::Pred(Predicate::new(relation, args))
}

pub fn principal(pcpl: Principal) -> Term {
    Term::Principal(pcpl)
}

// a rule is a predicate with a list of sub-rules represented in predicates as well
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub main_rule: Predicate,
    pub sub_rules: Vec<Predicate>,
}

impl Rule {
    pub fn fact(relation: Sym, args: Vec<Term>) -> Self {
        let main_rule = Predicate {
            relation,
            args,
        };
        Self { main_rule, sub_rules: vec![] }
    }
    pub fn add_sub_rule(mut self, relation: Sym, args: Vec<Term>) -> Self {
        let pred = Predicate {
            relation,
            args,
        };
        self.sub_rules.push(pred);
        self
    }
}

// a query is a list of predicates
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    pub goals: Vec<Predicate>,
}

impl Query {
    pub fn empty() -> Query {
        Query::with_goals(vec![])
    }

    pub fn with_goals(goals: Vec<Predicate>) -> Query {
        Query { goals }
    }

    pub fn new(relation: Sym, args: Vec<Term>) -> Query {
        Query::with_goals(vec![Predicate::new(relation, args)])
    }

    pub fn add_goals(mut self, relation: Sym, args: Vec<Term>) -> Self {
        let pred = Predicate {
            relation,
            args,
        };
        self.goals.push(pred);
        self
    }

    pub fn count_size(&self) -> usize {
        self.goals
            .iter()
            .map(|t| t.count_size())
            .max()
            .unwrap_or(0)
    }
}

// qunatify the number of principals in a given rule/query
fn quantify<R, const N: usize>(f: impl FnOnce([Principal; N]) -> R) -> R {
    let mut pcpls = [Principal::from_ord(0); N];
    pcpls.iter_mut()
        .enumerate()
        .for_each(|(i, pcpl)| *pcpl = Principal::from_ord(i));
    f(pcpls)
}

pub fn forall<const N: usize>(f: impl FnOnce([Principal; N]) -> Rule) -> Rule {
    quantify(f)
}

pub fn exists<const N: usize>(f: impl FnOnce([Principal; N]) -> Query) -> Query {
    quantify(f)
}