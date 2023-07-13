use aple::types::{FactLibrary, Rules, Predicate};
use aple::types::Types::{String, Number, Boolean, List, None};
use aple::goal::Goal;


fn main() {
    let mut fact_library = FactLibrary::new();
    let predicate1 = Predicate::new("".to_string(), "parent".to_string(), vec![String("Alice".to_string()), String("Bob".to_string())]);
    let predicate2 = Predicate::new("".to_string(), "parent".to_string(), vec![String("Alice".to_string()), String("Eve".to_string())]);
    let predicate3 = Predicate::new("".to_string(), "parent".to_string(), vec![String("Bob".to_string()), String("Charlie".to_string())]);
    let predicate4 = Predicate::new("".to_string(), "parent".to_string(), vec![String("Alice".to_string()), String("Dodge".to_string())]);
    let predicate5 = Predicate::new("".to_string(), "parent".to_string(), vec![String("Charlie".to_string()), String("Eve".to_string())]);
    
    fact_library.add_predicate(predicate1);
    fact_library.add_predicate(predicate2);
    fact_library.add_predicate(predicate3);
    fact_library.add_predicate(predicate4);
    fact_library.add_predicate(predicate5);

    let mut rule1 = Rules::new("".to_string(), "siblings".to_string(), vec!["X".to_string(), "Y".to_string()]);
    rule1.add_sub_rule("".to_string(), "parent".to_string(), vec!["Z".to_string(), "Y".to_string()]);
    rule1.add_sub_rule("".to_string(), "parent".to_string(), vec!["Z".to_string(), "X".to_string()]);
    fact_library.add_rule(rule1);

    let mut rule2 = Rules::new("".to_string(), "grandparent".to_string(), vec!["X".to_string(), "Y".to_string()]);
    rule2.add_sub_rule("".to_string(), "parent".to_string(), vec!["X".to_string(), "Z".to_string()]);
    rule2.add_sub_rule("".to_string(), "parent".to_string(), vec!["Z".to_string(), "Y".to_string()]);
    fact_library.add_rule(rule2);

    let mut rule3 = Rules::new("".to_string(), "family".to_string(), vec!["X".to_string(), "Y".to_string(), "Z".to_string()]);
    rule3.add_sub_rule("".to_string(), "parent".to_string(), vec!["X".to_string(), "Z".to_string()]);
    rule3.add_sub_rule("".to_string(), "parent".to_string(), vec!["Y".to_string(), "Z".to_string()]);
    rule3.add_sub_rule("".to_string(), "couple".to_string(), vec!["X".to_string(), "Y".to_string()]);
    fact_library.add_rule(rule3);

    let mut rule4 = Rules::new("".to_string(), "couple".to_string(), vec!["X".to_string(), "Y".to_string()]);
    rule4.add_sub_rule("".to_string(), "parent".to_string(), vec!["X".to_string(), "Z".to_string()]);
    rule4.add_sub_rule("".to_string(), "parent".to_string(), vec!["Y".to_string(), "Z".to_string()]);
    fact_library.add_rule(rule4);


    let goal1 = Goal::new("".to_string(), "siblings".to_string(), vec![String("Eve".to_string()), String("Bob".to_string())]);
    // rule1.match_rules([String("Bob".to_string()), String("Eve".to_string())],&fact_library);
    let result = goal1.query(&fact_library);
    if result {
        println!("true");
    }
    else{
        println!("false");
    }

    let goal2 = Goal::new("".to_string(), "grandparent".to_string(), vec![String("Alice".to_string()), String("Charlie".to_string())]);
    let result = goal2.query(&fact_library);
    if result {
        println!("true");
    }
    else{
        println!("false");
    }

    let goal3 = Goal::new("".to_string(), "family".to_string(), vec![String("Alice".to_string()), String("Charlie".to_string()), String("Eve".to_string())]);
    let result = goal3.query(&fact_library);
    if result {
        println!("true");
    }
    else{
        println!("false");
    }

    
}


/*
An example rule:
siblings(X,Y) :- parent(Z,Y), parent(Z,X).

what if?

siblings(X,Y):
    parent(Z,X)
    parent(Z,Y)
    couple(Z,T)
    T:parent(T,X)
    T:parent(T,Y)


 */