use crate::types::{Predicate, Query, Rule, Term};

use super::NamedUniverse;

use crate::parser::Types;

pub struct Printer<'u> {
    universe: &'u NamedUniverse,
}

impl<'a> Printer<'a> {
    pub fn new(universe: &'a NamedUniverse) -> Self {
        Self { universe }
    }

    pub fn query_to_string(&self, query: &Query) -> String {
        let mut out = String::new();
        self.print_query(&mut out, query).unwrap();
        out
    }

    pub fn rule_to_string(&self, rule: &Rule) -> String {
        let mut out = String::new();
        self.print_rule(&mut out, rule).unwrap();
        out
    }

    pub fn term_to_string(&self, term: &Term) -> String {
        let mut out = String::new();
        self.print(&mut out, term).unwrap();
        out
    }

    pub fn print<W: std::fmt::Write>(&self, writer: &mut W, term: &Term) -> std::fmt::Result {
        match term {
            Term::Principal(p) => write!(writer, "${}", p.ord()),
            Term::Pred(pred) => self.print_pred(writer, pred),
        }
    }

    pub fn print_pred<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        term: &Predicate,
    ) -> std::fmt::Result {
        if let Some(Types::String(name)) = &self.universe.symbol_name(term.relation) {
            
            write!(writer, "{}", name)?;
        } else {
            write!(writer, "<unk:{}>", term.relation.ord())?;
        }

        if let Some((first, rest)) = term.args.split_first() {
            write!(writer, "(")?;

            self.print(writer, first)?;
            for arg in rest {
                write!(writer, ", ")?;
                self.print(writer, arg)?;
            }

            write!(writer, ")")?;
        }

        Ok(())
    }

    pub fn print_query<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        query: &Query,
    ) -> std::fmt::Result {
        self.print_conjunction(writer, &query.goals)
    }

    pub fn print_conjunction<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        goals: &[Predicate],
    ) -> std::fmt::Result {
        if let Some((first, rest)) = goals.split_first() {
            self.print_pred(writer, first)?;
            for arg in rest {
                write!(writer, ", ")?;
                self.print_pred(writer, arg)?;
            }
        }
        write!(writer, ".")?;

        Ok(())
    }

    pub fn print_rule<W: std::fmt::Write>(&self, writer: &mut W, rule: &Rule) -> std::fmt::Result {
        self.print_pred(writer, &rule.main_rule)?;
        if rule.sub_rules.is_empty() {
            write!(writer, ".")?;
        } else {
            write!(writer, " :- ")?;
            self.print_conjunction(writer, &rule.sub_rules)?;
        }
        Ok(())
    }
}