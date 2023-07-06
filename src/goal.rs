use crate::types::Types;
use crate::types::FactLibrary;
pub struct Goal{
    speaker: String,
    relation: String,
    variables: Vec<Types>,
    sub_goals: Vec<Goal>,
}

// fn vec_compare(va: &[String], vb: &[String]) -> bool {
//     (va.len() == vb.len()) &&  // zip stops at the shortest
//      va.iter()
//        .zip(vb)
//        .all(|(a,b)| eq_with_nan_eq(*a,*b))
// }


impl Goal{
    pub fn new(speaker: String, relation: String, variables: Vec<Types>) -> Goal{
        Goal{
            speaker: speaker,
            relation: relation,
            variables: variables,
            sub_goals: vec![]
        }
    }

    pub fn add_sub_goal(&mut self, speaker: String, relation: String, variables: Vec<Types>){
        let sub_goal = Goal::new(speaker, relation, variables);
        self.sub_goals.push(sub_goal);
    }

    pub fn get_speaker(&self) -> &String{
        &self.speaker
    }

    pub fn get_relation(&self) -> &String{
        &self.relation
    }

    pub fn get_variables(&self) -> &Vec<Types>{
        &self.variables
    }

    pub fn get_sub_goals(&self) -> &Vec<Goal>{
        &self.sub_goals
    }

    pub fn query(& self, library: &FactLibrary) -> bool{
        let mut result = false;
        if self.speaker != ""{
            let lib_sp_index = library.search_by_speaker(self.speaker.clone());
        }
        else{
            let lib_rel_index = library.search_by_relation(self.relation.clone());
            // println!("lib_rel_index: {:?}", lib_rel_index);
            for p in lib_rel_index{
                if self.variables == *p.get_variable(){
                    result = true;
                    return result;
                }
            }
            let lib_rules = library.search_rule(self.relation.clone());
            // println!("lib_rules: {:?}", lib_rules);
            for r in lib_rules{
                // println!("r: {:?}", r);
                if r.get_relation().to_owned() == self.relation{
                    let res_vec = r.match_rules(&self.variables, &library);
                    println!("res_vec: {:?}", res_vec);
                    if res_vec.len() > 0{
                        result = true;
                        return result;
                    }
                }
            }
        }

        result
    }

}