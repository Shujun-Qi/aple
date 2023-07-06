use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Types{
    String(String),
    Number(i32),
    Boolean(bool),
    List(Vec<Types>),
    None
}


#[derive(Debug)]
pub struct FactLibrary{
    predicates:Vec<Rc<Predicate>>,
    speakers: HashMap<String, Vec<Rc<Predicate>>>,
    relations: HashMap<String, Vec<Rc<Predicate>>>,
    rules: HashMap<String, Vec<Rc<Rules>>>
}

impl FactLibrary{
    pub fn new() -> Self{
        Self{
            predicates: vec![],
            speakers: HashMap::new(),
            relations: HashMap::new(),
            rules: HashMap::new()
        }
    }

    pub fn add_predicate(&mut self, predicate: Predicate){
        let pointer = Rc::new(predicate);
        self.predicates.push(Rc::clone(&pointer));
        
        if self.speakers.contains_key(&pointer.speaker) {
            self.speakers.get_mut(&pointer.speaker).unwrap().push(Rc::clone(&pointer));
        }else{
            self.speakers.insert(pointer.speaker.clone(), vec![Rc::clone(&pointer)]);
        }
        if self.relations.contains_key(&pointer.relation){
            self.relations.get_mut(&pointer.relation).unwrap().push(Rc::clone(&pointer));
        }else{
            self.relations.insert(pointer.relation.clone(), vec![Rc::clone(&pointer)]);
        }


        // self.speakers.entry(speaker).and_modify(|v| v.push(&self.predicates.last().unwrap())).or_insert(vec![&self.predicates.last().unwrap()]);
        // self.relations.entry(relation).and_modify(|v| v.push(&self.predicates.last().unwrap())).or_insert(vec![&self.predicates.last().unwrap()]);
    }

    pub fn add_rule(&mut self, rules: Rules){
        let pointer = Rc::new(rules);
        self.rules.entry(pointer.relation.clone()).and_modify(|v| v.push(Rc::clone(&pointer))).or_insert(vec![Rc::clone(&pointer)]);
    }

    pub fn search_rule(&self, relation: String) -> Vec<&Rc<Rules>>{
        let mut result = vec![];
        // println!("searching rule: {}", relation);
        // println!("rules: {:?}", self.rules);
        if self.rules.contains_key(&relation) {
            for rule in self.rules.get(&relation).unwrap() {
                result.push(rule);
                println!("{:?}", **rule);
            }
        }
        result
    }

    pub fn search_by_speaker(&self, speaker: String) -> Vec<&Rc<Predicate>>{
        let mut result = vec![];
        if self.speakers.contains_key(&speaker) {
            for predicate in self.speakers.get(&speaker).unwrap() {
                result.push(predicate);
                // println!("{:?}", **predicate);
            }
        }
        result
    }

    pub fn search_by_relation(&self, relation: String) -> Vec<&Rc<Predicate>>{
        let mut result = vec![];
        if self.relations.contains_key(&relation) {
            for predicate in self.relations.get(&relation).unwrap() {
                result.push(predicate);
                // println!("{:?}", **predicate);
            }
        }
        result
    }

}



#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Predicate{
    speaker: String,
    relation: String,
    variable: Vec<Types>
}

impl Predicate {
    pub fn new(speaker: String, relation: String, variable: Vec<Types>) -> Self{
        Self{
            speaker: speaker,
            relation: relation,
            variable: variable
        }
    }

    pub fn get_speaker(&self) -> &String{
        &self.speaker
    }

    pub fn get_relation(&self) -> &String{
        &self.relation
    }

    pub fn get_variable(&self) -> &Vec<Types>{
        &self.variable
    }

}


#[derive(Debug)]
pub struct Rules{
    speaker: String,
    relation: String,
    variable: Vec<String>,
    sub_rules: Vec<Rules>
}

impl Rules {
    pub fn new(speaker: String, relation: String, variable: Vec<String>) -> Self{
        Self{
            speaker: speaker,
            relation: relation,
            variable: variable,
            sub_rules: vec![]
        }
    }

    pub fn add_sub_rule(&mut self, speaker: String, relation: String, variable: Vec<String>){
        let sub_rule = Rules::new(speaker, relation, variable);
        self.sub_rules.push(sub_rule);
    }

    pub fn match_rules(&self, variables:&Vec<Types>, lib:&FactLibrary) -> Vec<Rc<Predicate>>{
        
        let mut result:Vec<Rc<Predicate>> = vec![];
        let variable = self.get_variable();
        if variable.len() != variables.len() {
            panic!("The number of variables does not match");
        }
        let mut index = vec![];
        let mut tuples = vec![];
        for (pos, key) in variable.iter().enumerate(){
            index.push(key.clone());
        }
        tuples.push(variables.clone());
        println!("index: {:?}", index);
        println!("tuples: {:?}", tuples);

        for sub_r in &self.sub_rules {
            let predicates = lib.search_by_relation(sub_r.relation.clone());
            let sub_v = sub_r.get_variable();
            // let mut valid_predicates = vec![];
            let mut need_update = false;
            let mut valid_pred = vec![vec![]; predicates.len()];
            for (pos, predicate) in predicates.iter().enumerate() {
                let pred_v = predicate.get_variable();
                let mut flag = true;
                for (t_pos, key) in tuples.iter().enumerate(){
                    for (p, k) in sub_v.iter().enumerate(){
                        if index.contains(k){
                            let i = index.iter().position(|x| x == k).unwrap();
                            if key[i] != pred_v[p]{
                                flag = false;
                                break;
                            }
                        }
                        else{
                            need_update = true;
                        }
                    }
                    if flag {
                        valid_pred[pos].push(t_pos);
                    }
                }
            }
            for (p_ind, valid) in valid_pred.iter().enumerate(){
                if !valid.is_empty() {
                    let pred_v = predicates[p_ind].get_variable();
                    for v in valid {
                        if need_update{
                            let mut new_tuple = tuples[*v].clone();
                            for (p, k) in sub_v.iter().enumerate(){
                                if !index.contains(k){
                                    new_tuple.push(pred_v[p].clone());
                                }
                            }
                            tuples.push(new_tuple);
                        }
                    }        
                }
            }
            if need_update{
                for (p, k) in sub_v.iter().enumerate(){
                    if !index.contains(k){
                        index.push(k.clone());
                    }
                }
                tuples.retain(|x| x.len() == index.len()); 
                
            }
            println!("index: {:?}", index);
            println!("tuples: {:?}", tuples);
            // println!("{:?}", result);
            // println!("{:?}", index);
            // let matching = result.iter().zip(predicates).filter(|(a,b) | a.)
        }
        for tuple in tuples{
            let mut new_var = vec![];
            for (pos, key) in variable.iter().enumerate(){
                let i = index.iter().position(|x| x == key).unwrap();
                new_var.push(tuple[i].clone());
            }
            let new_pred = Rc::new(Predicate::new(self.speaker.clone(), self.relation.clone(), new_var));
            result.push(new_pred);
        }
        result
        
    }

    pub fn get_speaker(&self) -> &String{
        &self.speaker
    }

    pub fn get_relation(&self) -> &String{
        &self.relation
    }

    pub fn get_variable(&self) -> &Vec<String>{
        &self.variable
    }

    pub fn get_sub_rules(&self) -> &Vec<Rules>{
        &self.sub_rules
    }

}

