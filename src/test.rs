use aple::types::{FactLibrary, Rules, Predicate};
use aple::types::Types::{String, Number, Boolean, List, None};
use aple::goal::Goal;


fn main() {
    let mut fact_library = FactLibrary::new();
    let predicate1 = Predicate::new("".to_string(), "parent".to_string(), vec![String("Alice".to_string()), String("Bob".to_string())]);
    let predicate2 = Predicate::new("".to_string(), "parent".to_string(), vec![String("Alice".to_string()), String("Eve".to_string())]);
    fact_library.add_predicate(predicate1);
    fact_library.add_predicate(predicate2);
    let mut rule1 = Rules::new("".to_string(), "siblings".to_string(), vec!["X".to_string(), "Y".to_string()]);
    rule1.add_sub_rule("".to_string(), "parent".to_string(), vec!["Z".to_string(), "Y".to_string()]);
    rule1.add_sub_rule("".to_string(), "parent".to_string(), vec!["Z".to_string(), "X".to_string()]);
    fact_library.add_rule(rule1);
    let goal1 = Goal::new("".to_string(), "siblings".to_string(), vec![String("Eve".to_string()), String("Bob".to_string())]);
    // rule1.match_rules([String("Bob".to_string()), String("Eve".to_string())],&fact_library);
    let result = goal1.query(&fact_library);
    if result {
        println!("true");
    }
    else{
        println!("false");
    }
}
