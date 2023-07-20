use super::*;
use std::collections::HashMap;
use std::io::prelude::*;
// use std::process::exit;
use serde_json::{Result, Value, from_value};
use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
// use regex::Regex;

use crate::{types::*, sym_naming::NamedUniverse, sym_naming::Types};


#[derive(Debug, Default)]
struct Rules{
    speaker: String,
    // condition: Vec<(String, String)>,
    relation: String,
    flag: bool,
    // facts: Vec<String>
}


#[test]
fn test_compiler(){
    let mut nu = NamedUniverse::new();
    build_facts_library("test/DemoData/Intoto/","Intoto", &mut nu);
    nu.print_names();
    nu.print_rules();
    query_test(&mut nu);
    // println!("The whole universe is {:?}.", nu.name_index());
}

fn recursive_write(speaker: &str, id:&str,map: HashMap<String,Value>, file_path: &str, rules:&Vec<Rules>, flag:bool, nu: & mut NamedUniverse) -> Result<()>{
    
    // let file_path = "Example_Output/".to_owned()+sourcetype+"/"+sourcetype+".pl";
    // let mut output_file = fs::OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(file_path)
    //     .unwrap();

    // let mut output_string: String = "".to_string().to_owned();
    
    let mut output = vec![nu.symbol(&Types::String(id.to_string()))];

    let mut relation = "sign";
    let mut flag: bool = false;
    for rule in rules{
        if speaker == rule.speaker{
            relation = &rule.relation;
            flag = rule.flag;
        }
    }
    for (key, value) in map{
        if value.is_object(){
            if flag{
                // output_string  = output_string+&key+",";
                output.push(nu.symbol(&Types::String(key.clone())));
                let newmap: HashMap<String, Value> = from_value(value.clone())?;
                let _ = recursive_write(&key, &key, newmap, file_path, rules, false, nu);
            }
            else{
                let hash: String = my_hash(value.to_string()).to_string();
                // output_string = output_string+&hash+",";
                output.push(nu.symbol(&Types::String(hash.clone())));
                let newmap: HashMap<String, Value> = from_value(value.clone())?;
                let _ = recursive_write(&key, &hash, newmap, file_path, rules, false, nu);
            }
            
            
        }
        else if value.is_array(){
            
            // println!("{},{},Array", speaker, hash);
            
            let array : Vec<Value> = from_value(value.clone())?;
            // let mut obj = false;
            // let mut output_string = "[".to_owned();
            let mut sub_array = vec![];
            for item in array{
                if item.is_object(){
                    // output_string = output_string+&hash+",";
                    let hash: String = my_hash(item.to_string()).to_string();
                    sub_array.push(Types::String(hash.clone()));
                    let newmap: HashMap<String, Value> = from_value(item)?;
                    let _ = recursive_write(&key, &hash, newmap, file_path, rules, false, nu);
                    // obj = true;
                }
                else if item.is_number(){
                    // output_string = output_string+"[";
                    // println!("{:?}", item);
                    // output_string = output_string+&item.to_string()+",";
                    let num = item.as_i64().unwrap();
                    sub_array.push(Types::Number(num));
                }
                else{
                    let sub_str = item.to_string();
                    sub_array.push(Types::String(sub_str));
                }
            }
            // if !obj{
                // output_string.pop();
                // output_string = output_string+"],";
            // }   
            output.push(nu.symbol(&Types::Array(sub_array)));
        }
        else if value.is_number() {
            // output_string = output_string+&value.to_string()+",";
            let num = value.as_i64().unwrap();
            output.push(nu.symbol(&Types::Number(num)));
        }
        else{
            let str = value.to_string().replace("\"", "");
            // println!("{:?}", str);
            output.push(nu.symbol(&Types::String(str)));
        }
    }
    // output_string.pop();
    // if let Err(e) = writeln!(output_file, "{}({},[{}]).",relation, id ,output_string) {
    //     eprintln!("Couldn't write to file: {}", e);
    // }
    // println!("The output is ({}).", {output_string});
    // println!("The output is ({:?}).", output);
    let args = output.into_iter().map(|s| Term::Pred(s.into())).collect();
    // println!("The args is ({:?}).", args);
    let rs = nu.symbol(&Types::String(relation.to_string()));
    let rule = Rule::fact(rs, args);
    // println!("rule:{:?}", rule);
    nu.inner_mut().add_rule(rule);
    Ok(())
}

fn my_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

fn parse_files(filename:&str, sourcetype: &str, types:&Vec<Rules>, nu: &mut NamedUniverse) -> Result<()>{
    let file_path = "test/Example_Output/".to_owned()+sourcetype+"/"+sourcetype+".pl";
    let output_name = file_path.clone();
    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    if let Err(e) = writeln!(output_file, "\n#Facts from {}.", filename) {
        eprintln!("Couldn't write to file: {}", e);
    }
    // println!{"\n#Facts from {}.", filename};

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let intoto_root: Value = serde_json::from_str(&contents)?;
    let speaker: String = my_hash(intoto_root.to_string()).to_string();
    
    if intoto_root.is_object(){
        let map: HashMap<String, Value> = from_value(intoto_root)?;
        let _ = recursive_write("root", &speaker, map, &output_name, types, false, nu);
    }
    else{

    } 

    Ok(())
}

fn parse_rules(rule_file:&str) -> Vec<Rules>{
    let mut types: Vec<Rules> = Default::default();
    let rule_contents = fs::read_to_string(rule_file)
        .expect("Something went wrong reading the file");
    let binding = rule_contents.trim().replace("\n", "").replace("\r", "");
    let rules: Vec<&str> = binding.split(".").filter(|s| !s.is_empty()).collect();
    for rule in rules{
        let parts: Vec<&str> = rule.split(":").filter(|s| !s.is_empty()).collect();
        let speaker = parts[0];
        let mut relation = "sign";
        if parts.len() > 1{
            relation = parts[1];
        } 
        let new_rule = Rules { speaker: speaker.replace("$", ""), relation: relation.to_owned(), flag: speaker.contains("$") };
        types.push(new_rule);
    }
    return types;
}

pub fn build_facts_library(dirname:&str, sourcetype: &str, nu: &mut NamedUniverse){
    
    // let re: Regex = Regex::new(r"\((.+)\)").unwrap();
    
    let rule_file = "test/Rules/".to_owned()+sourcetype+".schema";
    let rules = parse_rules(&rule_file);
    // println!("{:?}", rules);
    let paths = fs::read_dir(dirname).unwrap();
    let file_path = "test/Example_Output/".to_owned()+sourcetype+"/"+sourcetype+".pl";
    let file_name = file_path.clone();
    if let Err(_) = fs::remove_file(file_name){
        println!("No file to remove");
    }
    let mut output_file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    // let policy_file = "Policy/".to_owned()+sourcetype+".pl";
    // if let Err(e) = writeln!(output_file, ":-consult({}).", policy_file) {
    //     eprintln!("Couldn't write to file: {}", e);
    // }

    for path in paths {
        let _ =   parse_files(path.unwrap().path().as_os_str().to_str().unwrap(), sourcetype,  &rules, nu);
    }
    
}

fn query_test(nu: &mut NamedUniverse){
    let keyid = nu.symbol(&Types::String("12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2".to_owned()));
    let sig = nu.symbol(&Types::String("signatures".to_owned()));
    let validate = nu.symbol(&Types::String("validate".to_owned()));
    let key_info = nu.symbol(&Types::String("key_info".to_owned()));

    nu.inner_mut().add_rule(forall(|[k, sp1, sp2, s1, s2]| {
        Rule::fact(validate, vec![k.into(), sp1.into(), sp2.into(), s1.into(), s2.into()])
        .add_sub_rule(sig, vec![sp1.into(), k.into(), s1.into(), s2.into()])
        .add_sub_rule(key_info, vec![sp2.into(), k.into()])
    }));

    let query = exists(|[x1, x2, x3, x4]|{
        Query::new(validate, vec![keyid.into(), x1.into(), x2.into(), x3.into(), x4.into()])
    });

    let solutions = query_dfs(nu.inner(), &query).collect::<Vec<_>>();
    if solutions.len() == 0{
        println!("Query failed, no matching facts.");
    }
    else {
        for (count, solution) in solutions.into_iter().enumerate(){
            println!("matching record: {:?}", count);
            for pred in solution.into_iter().map(|t| t.unwrap()){
                let name = match pred{
                    Term::Pred(p) => nu.symbol_name(p.relation).unwrap(),
                    _ => None.unwrap(),
                };
                println!("terms: {:?}", name);
            }
        }
        println!("Query succeeded!");
    }
    
    
}