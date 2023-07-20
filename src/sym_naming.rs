use std::collections::HashMap;
use crate::{
    types::{Sym, Term},
    engine::{self, SolutionIter},
    universe::Universe,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Types{
    String(String),
    Number(i64),
    Array(Vec<Types>),
    None
}

impl From<std::string::String> for Types {
    fn from(str: std::string::String) -> Self {
        Types::String(str)
    }
}

impl From<&str> for Types {
    fn from(str: &str) -> Self {
        Types::String(str.to_string())
    }
}

impl From<Vec<Types>> for Types {
    fn from(v: Vec<Types>) -> Self {
        Types::Array(v)
    }
}

impl From<i64> for Types{
    fn from(i: i64) -> Self{
        Types::Number(i)
    }
}




pub struct NamedUniverse {
    names: HashMap<Types, Sym>,
    syms: HashMap<Sym, Types>,
    universe: Universe,
}

impl NamedUniverse {

    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
            syms: HashMap::new(),
            universe: Universe::new(),
        }
    }

    pub fn symbol(&mut self, name: &Types) -> Sym {
        if let Some(sym) = self.names.get(name) {
            *sym
        } else {
            let sym = self.universe.alloc_symbol();
            self.names.insert(name.to_owned(), sym);
            self.syms.insert(sym, name.to_owned());
            sym
        }
    }
    pub fn symbol_name(&self, sym: Sym) -> Option<&Types> {
        self.syms.get(&sym)
    }

    pub fn inner_mut(&mut self) -> &mut Universe {
        &mut self.universe
    }

    pub fn inner(&self) -> &Universe {
        &self.universe
    }

    pub fn name_index(&self) -> &HashMap<Types, Sym>{
        &self.names
    }

    pub fn print_rules(&self){
        let rules = *&self.inner().rules();
        for rule in rules{
            let main_name = self.symbol_name(rule.main_rule.relation).unwrap();
            let args = &rule.main_rule.args;
            let arg_name: Vec<&Types> = args.into_iter().map(|a| match a {
                Term::Pred(pred) => self.symbol_name(pred.relation).unwrap(),
                _ => &Types::None,
            }).collect();
            let sub_rule = &rule.sub_rules;
            let sub_name: Vec<&Types> = sub_rule.into_iter().map(|sr| self.symbol_name(sr.relation).unwrap()).collect();
            println!("relation:{:?}, args: {:?}, sub_rule:{:?}", main_name, arg_name, sub_name);
        }
    }

    pub fn print_names(&self){
        for (sym, value) in self.syms.clone().into_iter(){
            println!("Sym: {:?}, Value: {:?}", sym, value);
        }
    }
}

impl Default for NamedUniverse {
    fn default() -> Self {
        Self::new()
    }
}

