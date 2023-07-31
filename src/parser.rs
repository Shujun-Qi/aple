mod lexer;
mod parser;
mod printer;

use std::{collections::HashMap, fs::{File, OpenOptions}};
use crate::{
    types::{Sym, Term, Query},
    engine::{self, SolutionIter},
    universe::Universe,
};

use std::io::Write;

pub use parser::{ParseError, ParseErrorKind};
pub use self::{parser::Parser, printer::Printer};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Types{
    String(String),
    Number(i64),
    Array(Vec<Types>),
    None
}
#[derive(Debug)]
pub enum Status{
    Fetch,
    Fail,
    Success
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

    pub fn write_rules(&self, filename: &str){
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
            let mut w = OpenOptions::new()
                .append(true)
                .create(true)
                .open(filename)
                .unwrap();
            writeln!(&mut w, "relation:{:?}, args: {:?}, sub_rule:{:?}", main_name, arg_name, sub_name).unwrap();
        }
    }

    pub fn write_names(&self, filename: &str){
        for (sym, value) in self.syms.clone().into_iter(){
            let mut w = OpenOptions::new()
                .append(true)
                .create(true)
                .open(filename)
                .unwrap();
            writeln!(&mut w, "Sym: {:?}, Value: {:?}", sym, value).unwrap();
        }
    }

    // pub fn query(&self, query: &Query, domain: &str) -> Result<SolutionIter, Status>{
        

    // }

}

impl Default for NamedUniverse {
    fn default() -> Self {
        Self::new()
    }
}


pub struct TextualUniverse {
    universe: NamedUniverse,
}

impl TextualUniverse {
    pub fn new() -> Self {
        Self {
            universe: NamedUniverse::new(),
        }
    }

    pub fn load_str(&mut self, rules: &str) -> Result<(), ParseError> {
        let rules = Parser::new(&mut self.universe).parse_rules_str(rules)?;
        for rule in rules {
            self.universe.inner_mut().add_rule(rule);
        }
        Ok(())
    }

    pub fn prepare_query(&mut self, query: &str) -> Result<Query, ParseError> {
        Parser::new(&mut self.universe).parse_query_str(query)
    }

    pub fn query_dfs(&mut self, query: &str) -> Result<SolutionIter, ParseError> {
        let query = self.prepare_query(query)?;
        Ok(engine::query_dfs(self.universe.inner(), &query))
    }

    pub fn printer(&self) -> Printer {
        Printer::new(&self.universe)
    }

    pub fn parse(&mut self) -> Parser {
        Parser::new(&mut self.universe)
    }

    pub fn inner_mut(&mut self) -> &mut Universe {
        self.universe.inner_mut()
    }

    pub fn inner(&self) -> &Universe {
        self.universe.inner()
    }
}

impl Default for TextualUniverse {
    fn default() -> Self {
        Self::new()
    }
}

