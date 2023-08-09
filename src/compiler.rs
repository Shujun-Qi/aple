use super::*;
use std::collections::HashMap;
use std::io::prelude::*;
use std::os::unix::prelude::PermissionsExt;
// use std::process::exit;
use serde_json::{Result, Value, from_value, from_str};
use std::fs::{self, OpenOptions};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use indexmap::IndexMap;
// use regex::Regex;
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::DecodePublicKey, pkcs8::EncodePublicKey, pkcs8::DecodePrivateKey, pkcs8::EncodePrivateKey};
use rsa::pss::{BlindedSigningKey, VerifyingKey, Signature, SigningKey};
use rsa::signature::{Keypair,RandomizedSigner, SignatureEncoding, Verifier};
use rsa::sha2::{Digest, Sha256, Sha512, Sha384};
use rsa::pkcs8::LineEnding;
use rsa::rand_core::OsRng;
use std::str;
use chrono::prelude::*;


use crate::engine::SolutionIter;
use crate::{types::*, parser::NamedUniverse, parser::Types};
use crate::parser::{Parser, Printer, TextualUniverse};


#[cfg(test)]
mod test_compiler;

// struct for schema
#[derive(Debug)]
struct Schema{
    schema_type:SchemaTypes,
    field:String,
    fetch_type:FetchTypes,
    pedding: String
}

// struct for compiled trust metadata files
#[derive(Debug, Clone)]
struct File{
    filename: String,
    mt: MetadataType,
    dirname: String
}

// metadata types
#[derive(Debug, Clone)]
pub enum MetadataType{
    INTOTO,
    TUF
}

// name complement for fetching external files
#[derive(Debug)]
pub enum FetchTypes{
    PREFIX,
    SURFIX,
    REPLACE,
    KPREFIX,
    KSURFIX,
    EXTERNAL,
    NONE
}

// schema types
#[derive(Debug)]
pub enum SchemaTypes{
    RESAMPLE,
    HASH,
    DIRECT,
    ASSAMBLE,
    RENAME,
    FETCH,
    TIME,
    RULE,
    NONE
}

// struct for query result
#[derive(Debug, Default)]
pub struct QueryResult{
    results: Vec<String>,
    state: bool
}

impl QueryResult {
    pub fn new() -> Self{
        Self { results: vec![], state: false }
    }
}

// Compiler struct, containing the logic facts universe and the compiled file map
#[derive(Default)]
pub struct Compiler{
    tu: TextualUniverse,
    file_map: HashMap<String, File>
}

impl Compiler{
    pub fn new() -> Self{
        Self{
            tu : TextualUniverse::new(),
            file_map : HashMap::default()
        }
    }

    // compile a group of trust metadata files
    pub fn compile(& mut self, dirname: &str, mt: MetadataType){
        let root_filename = match mt{
            MetadataType::INTOTO => "root.layout",
            MetadataType::TUF => "root.json"
        };
        let rule_filename = match mt{
            MetadataType::INTOTO => "test/Rules/Intoto_test.rule",
            MetadataType::TUF => "test/Rules/Tuf_test.rule"
        };
        let file_s = File{
            filename: root_filename.to_string(),
            mt: mt.clone(), 
            dirname: dirname.to_string()
        };
        let map = self.get_schema(&mt);
        self.file_map.insert(root_filename.to_string(), file_s.clone());
        let rules = fs::read_to_string(rule_filename)
        .expect("Something went wrong reading the file");
        self.tu.load_str(&rules).expect("load rule string");
        let sp = match mt{
            MetadataType::INTOTO => "intotoroot",
            MetadataType::TUF => "tufroot"
        };
        let _ = self.fetch_external(sp, &file_s, &map);
    }

    // load policy file
    pub fn load_policy(& mut self, filename: &str){
        let rules = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
        self.tu.load_str(&rules).expect("load rule string");
    }

    // query the logic facts universe to get the result
    pub fn query(& mut self, q: &str, writelog: bool) -> QueryResult{
        
        let query = self.tu.prepare_query(q).expect("translate query");
        let mut solutions = query_dfs(self.tu.inner(), &query);
        let mut result = QueryResult::new();
        loop {
            // step by step iteration of the matching process of the query against the logic facts universe
            match solutions.step() {
                // Found a matching solution
                engine::Step::Yield => {

                    let solution = solutions.get_solution();
                    result.state = true;
                    for (index, var) in solution.into_iter().enumerate() {
                        if let Some(term) = var {
                            let st = self.tu.printer().term_to_string(&term);
                            if result.results.contains(&st){
                                continue;
                            }
                            result.results.push(self.tu.printer().term_to_string(&term));
                        } 
                    }
                    break;
                }
                engine::Step::Precompiled => {
                    // println!("Precompiled");
                    // let checkpoint = solutions.get_checkpoints().last().unwrap();
                    // solutions.checkpoints.pop();
                    continue;
                },
                engine::Step::Continue => {
                    if writelog{
                        // let id = solutions.unresolved_goals.last().unwrap();
                        // let sym = match solutions.solution.lib.get_term(*id) {
                        //     term_library::Term::Pred(pred, _) => pred,
                        //     _ => unreachable!(),
                        // };
                        // let rule = self.tu.universe().symbol_name(sym).unwrap();
                        // let csym = solutions.checkpoints.last().unwrap().get_main_rule();
                        // let crule = self.tu.universe().symbol_name(csym).unwrap();
                        let id = solutions.unresolved_goals.last().unwrap();
                            // let sym = match solutions.solution.lib.get_term(*id) {
                            //     term_library::Term::Pred(pred, _) => pred,
                            //     _ => unreachable!(),
                            // };

                            let term = solutions.solution.get_solution_term(*id);
                            let st = self.tu.printer().term_to_string(&term);
                            // let rule = self.tu.universe().symbol_name(sym).unwrap();
                            // println!("unresolved goal: {:?}", rule);
                        let mut w = OpenOptions::new()
                                .append(true)
                                .create(true)
                                .open("test/Example_Output/console.log")
                                .unwrap();
                        writeln!(&mut w, "goal name: {:?}", st).unwrap();
                        // writeln!(&mut w, "checkpoint: {:?}", solutions.checkpoints.last().unwrap()).unwrap();
                    }
                    
                    continue;
                },
                // No more solutions
                engine::Step::Done => {
                    if writelog{
                        if result.state == false{
                            let id = solutions.unresolved_goals.last().unwrap();
                            // let sym = match solutions.solution.lib.get_term(*id) {
                            //     term_library::Term::Pred(pred, _) => pred,
                            //     _ => unreachable!(),
                            // };

                            let term = solutions.solution.get_solution_term(*id);
                            let st = self.tu.printer().term_to_string(&term);
                            // let rule = self.tu.universe().symbol_name(sym).unwrap();
                            // println!("unresolved goal: {:?}", rule);
                            let mut w = OpenOptions::new()
                                .append(true)
                                .create(true)
                                .open("test/Example_Output/console.log")
                                .unwrap();
                            writeln!(&mut w, "unresolved goal: {:?}", st).unwrap();
                        }
                    }
                    break;
                }
            }
        }
        result
    }

    pub fn print_files(& self){
        println!("{:?}", self.file_map);
    }

    pub fn print_nu(& mut self, filename: &str){
        fs::remove_file(filename).expect("remove file");
        self.tu.universe().write_names(filename);
        self.tu.universe().write_rules(filename);
    }

    pub fn initial(& mut self){
        self.tu.universe_mut().symbol(&Types::String("add".to_string()));
        self.tu.universe_mut().symbol(&Types::String("sub".to_string()));
        self.tu.universe_mut().symbol(&Types::String("eq".to_string()));
        self.tu.universe_mut().symbol(&Types::String("lt".to_string()));
        self.tu.universe_mut().symbol(&Types::String("gt".to_string()));
        self.tu.universe_mut().symbol(&Types::String("geq".to_string()));
        self.tu.universe_mut().symbol(&Types::String("leq".to_string()));
        //test
        // let rule_n = self.tu.universe().symbol(&Types::String("lst".to_string()));
        // let num1 = self.tu.universe().symbol(&Types::Number(1));
        // let num2 = self.tu.universe().symbol(&Types::Number(2));
        // let args = vec![Term::Pred(num1.into()), Term::Pred(num2.into())];
        // let rule_t = self.tu.universe().symbol(&Types::String("test".to_string()));
        // let rule_filename = "test/Rules/test.rule";
        // let rules = fs::read_to_string(rule_filename)
        // .expect("Something went wrong reading the file");
        // self.tu.load_str(&rules).expect("load rule string");
        // let solutions = self.query("test(1,2).", false);
        // println!("{:?}", solutions);
    }

    // parse schema file and get the schema for metadata type
    fn get_schema(& mut self, mt: &MetadataType) -> HashMap<String, Schema>{
        let mut map: HashMap<String, Schema> = HashMap::default();
        let sourcetype = match mt{
            MetadataType::INTOTO => "Intoto",
            MetadataType::TUF => "Tuf"
        };
        let schema_file = "test/Rules/".to_owned()+sourcetype+".schema";
        let rule_contents = fs::read_to_string(schema_file)
            .expect("Something went wrong reading the file");
        let binding = rule_contents.trim().replace("\n", "").replace("\r", "");
        let rules: Vec<&str> = binding.split(";").filter(|s| !s.is_empty()).collect();
        for rule in rules{
            let parts: Vec<&str> = rule.split(":").filter(|s| !s.is_empty()).collect();
            let speaker = parts[0].to_string();
            let s_type = parts[1];
            let schema = match s_type{
                "RESAMPLE" => Schema{
                    schema_type: SchemaTypes::RESAMPLE,
                    field: "".to_string(),
                    fetch_type: FetchTypes::NONE,
                    pedding: "".to_string()
                },
                "HASH" => Schema{
                    schema_type: SchemaTypes::HASH,
                    field: "".to_string(),
                    fetch_type: FetchTypes::NONE,
                    pedding: "".to_string()
                },
                "RENAME" => Schema{
                    schema_type: SchemaTypes::RENAME,
                    field: parts[2].to_string(),
                    fetch_type: FetchTypes::NONE,
                    pedding: "".to_string()
                },
                "FETCH" => Schema{
                    schema_type: SchemaTypes::FETCH,
                    field: "".to_string(),
                    fetch_type: match parts[2]{
                        "SURFIX" => FetchTypes::SURFIX,
                        "PREFIX" => FetchTypes::PREFIX,
                        "REPLACE" => FetchTypes::REPLACE,
                        "KPREFIX" => FetchTypes::KPREFIX,
                        "KSURFIX" => FetchTypes::KSURFIX,
                        "EXTERNAL" => FetchTypes::EXTERNAL,
                        _ => FetchTypes::NONE
                    },
                    pedding: parts[3].to_string()
                },
                "RULE" => Schema{
                    schema_type: SchemaTypes::RULE,
                    field: "".to_string(),
                    fetch_type: FetchTypes::NONE,
                    pedding: "".to_string()
                },
                "DIRECT" => Schema { 
                    schema_type: SchemaTypes::DIRECT, 
                    field: "".to_string(), 
                    fetch_type: FetchTypes::NONE, 
                    pedding: "".to_string() 
                },
                "ASSAMBLE" => Schema { 
                    schema_type: SchemaTypes::ASSAMBLE, 
                    field: "".to_string(), 
                    fetch_type: FetchTypes::NONE, 
                    pedding: "".to_string() 
                },
                "TIME" => Schema { 
                    schema_type: SchemaTypes::TIME, 
                    field: "".to_string(), 
                    fetch_type: FetchTypes::NONE, 
                    pedding: "".to_string() 
                },
                _ => Schema{
                    schema_type: SchemaTypes::NONE,
                    field: "".to_string(),
                    fetch_type: FetchTypes::NONE,
                    pedding: "".to_string()
                }
            };
            map.insert(speaker, schema);
        }
        return map;
    }


    // validate the signature of the file
    fn validate_sig(&mut self, values:Value, speaker: &str, file: &File, schema_map:& HashMap<String, Schema>){
        
        let chunk = values["signed"].to_string();
        let sigs = values["signatures"].as_array().unwrap();
        let mut map: HashMap<String, String> = HashMap::default();
        for sig in sigs{
            let keyid = sig["keyid"].to_string().replace("\"", "");
            let signature = sig["sig"].to_string().replace("\"", "");
            map.insert(keyid, signature);
        }
        let mut count = 0;
        for (keyid, sig) in map{
            // let sig = map.get(keyid).unwrap();
            let pub_key: String = match speaker{
                "intotoroot" | "tufroot"=> {
                    let key_file = "test/keys/".to_owned()+&keyid+"_public.pem";
                    fs::read_to_string(key_file).unwrap()
                }
                _ => {
                    let q_str= format!("validate_key({}, $pubkey).", keyid);
                    let result = self.query(&q_str, false);
                    if !result.state{
                        continue;
                    }
                    let rk = result.results[0].clone().replace("\\n", "\r\n");
                    rk
                }
            };
            // println!("{}", pub_key);
            let public_key = RsaPublicKey::from_public_key_pem(pub_key.as_str()).unwrap();
            let verifying_key: VerifyingKey<Sha256> = VerifyingKey::from(public_key);

            let sig_bytes = hex::decode(sig.clone()).unwrap();
            let signature = Signature::try_from(sig_bytes.as_ref()).unwrap();
            let data_bytes = chunk.as_bytes();
            let result = verifying_key.verify(data_bytes, &signature);
            match result{
                Ok(()) => {
                    let rs = self.tu.universe_mut().symbol(&Types::String("sign".to_string()));
                    let kid = Term::Pred(self.tu.universe_mut().symbol(&Types::String(keyid.to_string())).into());
                    let file = Term::Pred(self.tu.universe_mut().symbol(&Types::String(speaker.to_string())).into());
                    let pubk = Term::Pred(self.tu.universe_mut().symbol(&Types::String(pub_key.replace("\r\n", "\\n"))).into());
                    let sig = Term::Pred(self.tu.universe_mut().symbol(&Types::String(sig.clone())).into());
                    let args = vec![kid, file, pubk, sig];
                    let rule = Rule::fact(rs, args);
                    self.tu.universe_mut().inner_mut().add_rule(rule);
                    count += 1;
                }
                Err(_) => println!("no")
            }
        }
        let rule_n = self.tu.universe_mut().symbol(&Types::String("verified".to_string()));
        let sp = Term::Pred(self.tu.universe_mut().symbol(&Types::String(speaker.to_string())).into());
        let cn = Term::Pred(self.tu.universe_mut().symbol(&Types::Number(count)).into());
        let args = vec![sp, cn.clone()];
        let rule = Rule::fact(rule_n, args);
        self.tu.universe_mut().inner_mut().add_rule(rule);

        let crule = self.tu.universe_mut().symbol(&Types::String("geq".to_string()));
        let c1 = Term::Pred(self.tu.universe_mut().symbol(&Types::Number(1)).into());
        let cargs = vec![cn, c1];
        let crule = Rule::fact(crule, cargs);
        self.tu.universe_mut().inner_mut().add_rule(crule);


        let map: IndexMap<String, Value> = from_value(values).expect("fail to translate json to maps");
        self.parse_files_recursive(file, speaker, &map, schema_map);
    }

    // fetch an external metadata file and hash it
    fn fetch_external(& mut self, speaker: &str , file: &File, schema_map:& HashMap<String, Schema>) -> Result<()>{
        
        
        let dirname = &file.dirname;
        let filename = &file.filename;
        let rfname = dirname.to_string()+ filename;
        // println!("{}", filename);
        let contents = fs::read_to_string(rfname)
        .expect("Something went wrong reading the file");
        
        let values: Value = serde_json::from_str(&contents)?;
        match file.mt{
            MetadataType::INTOTO |MetadataType::TUF => {
                let mut hasher = Sha256::new();
                hasher.update(contents);
                let result = hasher.finalize();
                let hash = format!("{:x}", result);
                let rs = self.tu.universe_mut().symbol(&Types::String("hash".to_string()));
                let files = Term::Pred(self.tu.universe_mut().symbol(&Types::String(file.filename.to_string())).into());
                let hashs = Term::Pred(self.tu.universe_mut().symbol(&Types::String(hash)).into());
                let sp = Term::Pred(self.tu.universe_mut().symbol(&Types::String(speaker.to_string())).into());
                let args = vec![sp, files, hashs];
                let rule = Rule::fact(rs, args);
                self.tu.universe_mut().inner_mut().add_rule(rule);

                
                self.validate_sig(values, speaker, file, schema_map)
            },
            // MetadataType::TUF => {
                
            //     self.validate_sig(values, speaker, file, schema_map)
            // }
            _ => {
                // let map: IndexMap<String, Value> = from_value(values).expect("fail to translate json to maps");
                // self.parse_files_recursive(file, speaker, &map, schema_map);

            }
        }
        Ok(())
    }

    
    // compiler handles the object type in json file
    fn handle_object(& mut self,file: &File, speaker: &str, key: &String, value: &Value, schema_map:& HashMap<String, Schema>){
        let imap: IndexMap<String, Value> = from_value(value.clone()).expect("cannot translate to map");
        // println!("{:?}", schema_map);
        if schema_map.contains_key(key){
            let sc = schema_map.get(key).unwrap();
            // println!(" key: {}, vvec:{:?}", speaker, key, value);
            match &sc.schema_type {
                SchemaTypes::RESAMPLE => {
                    let vvec = imap.values().cloned().collect::<Vec<Value>>();
                    self.handle_array(file, speaker, key, &vvec, schema_map);
                }
                SchemaTypes::HASH => {
                    self.parse_files_recursive(file, speaker, &imap, schema_map)
                }
                SchemaTypes::RENAME => {
                    let name = &sc.field;
                    let nv = imap.get(name).expect("get value").to_string().replace("\"", "");
                    self.parse_files_recursive(file, &nv, &imap, schema_map)
                }
                SchemaTypes::DIRECT => {
                    for (k, v) in imap{
                        let mut output = vec![self.tu.universe_mut().symbol(&Types::String(speaker.to_string()))];
                        output.push(self.tu.universe_mut().symbol(&Types::String(k.clone())));
                        let map: IndexMap<String, Value> = from_value(v.clone()).expect("cannot translate to map");
                        let args = output.into_iter().map(|s| Term::Pred(s.into())).collect();
                        
                        let rs = self.tu.universe_mut().symbol(&Types::String(key.to_string()));
                        let rule = Rule::fact(rs, args);
                        self.tu.inner_mut().add_rule(rule);
                        // if schema_map.contains_key(&k){
                        //     let scs = schema_map.get(&k).unwrap();
                        //     match &scs.schema_type {
                        //         SchemaTypes::FETCH => {
                        //             let ftype = &scs.fetch_type;
                        //             let pedding = &scs.pedding;
                        //             let fname = match ftype{
                        //                 FetchTypes::KPREFIX => pedding.to_string()+&k,
                        //                 FetchTypes::KSURFIX => k.to_string()+&pedding,
                        //                 _ => "".to_string(),
                        //             };
                        //             println!("fname: {}", fname);
                        //             if !self.file_map.contains_key(&fname){
                        //                 let temp_mt = file.mt.clone();
                        //                 let file_s = File{
                        //                     filename: fname.to_string(),
                        //                     mt: temp_mt, 
                        //                     dirname: file.dirname.to_string()
                        //                 };
                        //                 self.file_map.insert(fname.to_string(), file_s.clone());
                        //                 let _ = self.fetch_external(&k, &file_s, schema_map);
                        //             }
                        //         }
                        //         _ => {
                        //         }
                        //     }
                        // }
                        self.parse_files_recursive(file, &k, &map, schema_map)

                    }
                    // let args = vec![Term::Pred(self.tu.universe().symbol(&Types::String(speaker.to_string())).into()), Term::Pred(self.tu.universe().symbol(&Types::Array(testvec)).into())];
                    // let rs = self.tu.universe().symbol(&Types::String(key.to_string()));
                    // let rule = Rule::fact(rs, args);
                    // self.tu.inner_mut().add_rule(rule);
                }
                SchemaTypes::ASSAMBLE => {
                    let mut output = vec![];
                    for (k, v) in imap{
                        output.push(Types::String(k.to_string()));
                        let map: IndexMap<String, Value> = from_value(v.clone()).expect("cannot translate to map");
                        self.parse_files_recursive(file, &k, &map, schema_map)
                    }
                    let args = vec![Term::Pred(self.tu.universe_mut().symbol(&Types::String(speaker.to_string())).into()), Term::Pred(self.tu.universe_mut().symbol(&Types::Array(output)).into())];
                    let rs = self.tu.universe_mut().symbol(&Types::String(key.to_string()));
                    let rule = Rule::fact(rs, args);
                    self.tu.inner_mut().add_rule(rule);
                }
                SchemaTypes::FETCH => {
                    for (k, v) in imap{
                        let mut output = vec![self.tu.universe_mut().symbol(&Types::String(speaker.to_string()))];
                        output.push(self.tu.universe_mut().symbol(&Types::String(k.clone())));
                        let map: IndexMap<String, Value> = from_value(v.clone()).expect("cannot translate to map");
                        let args = output.into_iter().map(|s| Term::Pred(s.into())).collect();
                        
                        let rs = self.tu.universe_mut().symbol(&Types::String(key.to_string()));
                        let rule = Rule::fact(rs, args);
                        self.tu.inner_mut().add_rule(rule);
                        
                        let ftype = &sc.fetch_type;
                        let pedding = &sc.pedding;
                        let fname = match ftype{
                                FetchTypes::KPREFIX => pedding.to_string()+&k,
                                FetchTypes::KSURFIX => k.to_string()+&pedding,
                                FetchTypes::EXTERNAL => k.to_string(),
                                _ => "".to_string(),
                        };
                        if !self.file_map.contains_key(&fname){
                            let temp_mt = file.mt.clone();
                            let file_s = match ftype{
                                FetchTypes::EXTERNAL => File{
                                    filename: fname.to_string(),
                                    mt: match sc.pedding.as_str() {
                                        "INTOTO" => MetadataType::INTOTO,
                                        _ => file.mt.clone()
                                    },
                                    dirname: match sc.pedding.as_str() {
                                        "INTOTO" => "test/DemoData/Intoto/debian/".to_string(),
                                        _ => file.dirname.to_string()
                                    }
                                },
                                _ => File{
                                    filename: fname.to_string(),
                                    mt: temp_mt, 
                                    dirname: file.dirname.to_string()
                                }
                            };
                            self.file_map.insert(fname.to_string(), file_s.clone());
                            match ftype{
                                FetchTypes::EXTERNAL => {
                                    let _ = self.fetch_external("root", &file_s, schema_map);
                                }
                                _ => {
                                    let _ = self.fetch_external(&k, &file_s, schema_map);
                                }
                            }
                        }
                        self.parse_files_recursive(file, &k, &map, schema_map)
                    }
                }
                SchemaTypes::RULE => {

                }
                _ => {}
            }
        }
    }

    // compiler handles the array type in json
    fn handle_array(& mut self,file: &File, speaker: &str, key: &String, vvec: &Vec<Value>, schema_map:& HashMap<String, Schema>){  
        let mut output = vec![self.tu.universe_mut().symbol(&Types::String(speaker.to_string()))];
        let mut testvec = vec![];
        for value in vvec{
            if value.is_object(){
                // println!("speaker: {}, key: {}", speaker, key);
                self.handle_object(file, speaker, key, &value, schema_map);
            }
            else if value.is_array(){
                let value_vec = value.as_array().unwrap();
                // println!("{:?}", value_vec);
                self.handle_array(file, speaker, key, value_vec, schema_map);
            }
            else if value.is_number(){
                let num = value.as_i64().unwrap();
                output.push(self.tu.universe_mut().symbol(&Types::Number(num)));
                testvec.push(Types::Number(num));
            }else{
                let str = value.to_string().replace("\"", "");
                output.push(self.tu.universe_mut().symbol(&Types::String(str.clone())));
                testvec.push(Types::String(str));
            }
        }
        if vvec.len() == 0{
            output.push(self.tu.universe_mut().symbol(&Types::String("".to_string())));
            testvec.push(Types::String("".to_string()));
        }
        if output.len() > 1{
            let args = output.into_iter().map(|s| Term::Pred(s.into())).collect();
            // let args = vec![Term::Pred(self.tu.universe().symbol(&Types::String(speaker.to_string())).into()),Term::Pred(self.tu.universe().symbol(&Types::Array(testvec)).into())];
            let rs = self.tu.universe_mut().symbol(&Types::String(key.to_string()));
            let rule = Rule::fact(rs, args);
            self.tu.inner_mut().add_rule(rule);
        }
    }

    // compiler recursively parses the json file
    fn parse_files_recursive(& mut self, file: &File, speaker: &str, map: & IndexMap<String, Value>, schema_map:& HashMap<String, Schema>){
        let sp = self.tu.universe_mut().symbol(&Types::String(speaker.to_string()));
        for (key, value) in map{
            if value.is_object(){
                self.handle_object(file, speaker, key, value, schema_map);
            }
            else if value.is_array(){
                if schema_map.contains_key(key){
                    let sc = schema_map.get(key).unwrap();
                    match &sc.schema_type {
                        SchemaTypes::RULE => {continue;}
                        _ => {}
                    }
                }
                let vvec = value.as_array().unwrap();
                self.handle_array(file, speaker, key, vvec, schema_map);
            }
            else if value.is_number(){
                let num = value.as_i64().unwrap();
                let ns = self.tu.universe_mut().symbol(&Types::Number(num));
                let args =vec![Term::Pred(sp.into()), Term::Pred(ns.into())];
                let rs = self.tu.universe_mut().symbol(&Types::String(key.to_string()));
                let rule = Rule::fact(rs, args);
                self.tu.inner_mut().add_rule(rule);
            }else{
                let str = value.to_string().replace("\"", "");
                if schema_map.contains_key(key){
                    let sc = schema_map.get(key).unwrap();
                    match &sc.schema_type {
                        SchemaTypes::FETCH => {
                            let ftype = &sc.fetch_type;
                            let pedding = &sc.pedding;
                            let fname = match ftype{
                                FetchTypes::PREFIX => pedding.to_string()+&str,
                                FetchTypes::SURFIX => str.to_string()+pedding,
                                FetchTypes::REPLACE => pedding.to_string(),
                                FetchTypes::KPREFIX => key.to_string()+&str,
                                FetchTypes::KSURFIX => str.to_string()+key,
                                _ => "".to_string(),
                            };
                            if !self.file_map.contains_key(&fname){
                                let temp_mt = file.mt.clone();
                                let file_s = File{
                                    filename: fname.to_string(),
                                    mt: temp_mt, 
                                    dirname: file.dirname.to_string()
                                };
                                self.file_map.insert(fname.to_string(), file_s.clone());
                                let _ = self.fetch_external(&str, &file_s, schema_map);
                                continue;
                            }
                            let ss = self.tu.universe_mut().symbol(&Types::String(str));
                            let args =vec![Term::Pred(sp.into()), Term::Pred(ss.into())];
                            let rs = self.tu.universe_mut().symbol(&Types::String(key.to_string()));
                            let rule = Rule::fact(rs, args);
                            self.tu.inner_mut().add_rule(rule);
                        }
                        SchemaTypes::TIME => {
                            let bn = DateTime::parse_from_rfc3339(&str).unwrap();
                            let utc = DateTime::<Local>::from(bn).timestamp();

                            let now = Utc::now().timestamp();
                            let ns = self.tu.universe_mut().symbol(&Types::Number(now));
                            let nargs =vec![Term::Pred(sp.into()), Term::Pred(ns.into())];
                            let nrs = self.tu.universe_mut().symbol(&Types::String("now".to_string()));
                            let nrule = Rule::fact(nrs, nargs);
                            self.tu.inner_mut().add_rule(nrule);

                            let ss = self.tu.universe_mut().symbol(&Types::Number(utc));
                            let args =vec![Term::Pred(sp.into()), Term::Pred(ss.into())];
                            let rs = self.tu.universe_mut().symbol(&Types::String(key.to_string()));
                            let rule = Rule::fact(rs, args);
                            self.tu.inner_mut().add_rule(rule);
                            if now <= utc{
                                let cargs =vec![Term::Pred(ns.into()), Term::Pred(ss.into())];
                                let crs = self.tu.universe_mut().symbol(&Types::String("leq".to_string()));
                                let crule = Rule::fact(crs, cargs);
                                self.tu.inner_mut().add_rule(crule);
                            }
                        }
                        _ => {}
                    }
                }
                else{
                    let ss = self.tu.universe_mut().symbol(&Types::String(str));
                    let args =vec![Term::Pred(sp.into()), Term::Pred(ss.into())];
                    let rs = self.tu.universe_mut().symbol(&Types::String(key.to_string()));
                    let rule = Rule::fact(rs, args);
                    self.tu.inner_mut().add_rule(rule);
                }
            }
        }
        
    }
}