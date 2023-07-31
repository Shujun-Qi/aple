use super::*;
use std::collections::HashMap;
use std::io::prelude::*;
use std::os::unix::prelude::PermissionsExt;
// use std::process::exit;
use serde_json::{Result, Value, from_value};
use std::fs;
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


use crate::{types::*, parser::NamedUniverse, parser::Types};
use crate::parser::{Parser, Printer};


#[cfg(test)]
mod test_compiler;


#[derive(Debug)]
struct Schema{
    schema_type:SchemaTypes,
    field:String,
    fetch_type:FetchTypes,
    pedding: String
}

#[derive(Debug)]
struct File{
    filename: String,
    mt: MetadataType,
    dirname: String
}

#[derive(Debug, Clone)]
pub enum MetadataType{
    INTOTO,
    TUF
}

#[derive(Debug)]
pub enum FetchTypes{
    PREFIX,
    SURFIX,
    REPLACE,
    NONE
}
#[derive(Debug)]
pub enum SchemaTypes{
    RESAMPLE,
    HASH,
    Custom,
    RENAME,
    FETCH,
    RULE,
    NONE
}



#[derive(Default)]
pub struct Compiler{
    nu: NamedUniverse,
    file_map: HashMap<String, File>
}

impl Compiler{
    pub fn new() -> Self{
        Self{
            nu : NamedUniverse::new(),
            file_map : HashMap::default()
        }
    }

    pub fn compile(& mut self, dirname: &str, mt: MetadataType){
        // self.get_schema(&mt);
        self.handle_root(dirname, &mt);
    }

    pub fn print_files(& self){
        println!("{:?}", self.file_map);
    }

    pub fn print_nu(& self){
        let wu = std::fs::File::create("test/Example_Output/Intoto.log").unwrap();
        self.nu.write_names("test/Example_Output/Intoto.log");
        self.nu.write_rules("test/Example_Output/Intoto.log");
    }

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
                "CUSTOM" => Schema { 
                    schema_type: SchemaTypes::Custom, 
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

    fn handle_root(& mut self, dirname: &str, mt: &MetadataType){
        let root_filename = match mt{
            MetadataType::INTOTO => "root.layout",
            MetadataType::TUF => "root.json"
        };
        // let root_key = "0d6d097a467ebebdf03bed8e545312409afc9e17529ab4f10f97935d755d9059";
        // let key_vec = vec![root_key];
        // let meta = *mt.clone();
        let file_s = File{
            filename: root_filename.to_string(),
            mt: mt.clone(), 
            dirname: dirname.to_string()
        };
        let map = self.get_schema(mt);
        self.file_map.insert(root_filename.to_string(), file_s);
        let _ = self.fetch_external( dirname, &root_filename, mt, &map);
    }

    fn fetch_external(& mut self, dirname: &str,filename: &str, mt:& MetadataType, schema_map:& HashMap<String, Schema>) -> Result<()>{
        
        // println!("{}", filename);
        let rfname = dirname.to_string()+filename;
        let contents = fs::read_to_string(rfname)
        .expect("Something went wrong reading the file");

        let values: Value = serde_json::from_str(&contents)?;
        match mt{
            MetadataType::INTOTO => {
                let chunk = values["signed"].to_string();
                let sigs = values["signatures"].as_array().unwrap();
                let mut map: HashMap<String, String> = HashMap::default();
                for sig in sigs{
                    let keyid = sig["keyid"].to_string().replace("\"", "");
                    let signature = sig["sig"].to_string().replace("\"", "");
                    map.insert(keyid, signature);
                }
                for (keyid, sig) in map{
                    // let sig = map.get(keyid).unwrap();
                    let key_file = "test/keys/".to_owned()+&keyid+".pem";
                    let pub_key = fs::read_to_string(key_file).unwrap();
                    let public_key = RsaPublicKey::from_public_key_pem(pub_key.as_str()).unwrap();
                    let verifying_key: VerifyingKey<Sha256> = VerifyingKey::from(public_key);

                    let sig_bytes = hex::decode(sig.clone()).unwrap();
                    let signature = Signature::try_from(sig_bytes.as_ref()).unwrap();
                    let data_bytes = chunk.as_bytes();
                    let result = verifying_key.verify(data_bytes, &signature);
                    match result{
                        Ok(()) => {
                            let rs = self.nu.symbol(&Types::String("sign".to_string()));
                            let kid = Term::Pred(self.nu.symbol(&Types::String(keyid.to_string())).into());
                            let file = Term::Pred(self.nu.symbol(&Types::String(filename.to_string())).into());
                            let sig = Term::Pred(self.nu.symbol(&Types::String(sig.clone())).into());
                            let args = vec![kid, file, sig];
                            let rule = Rule::fact(rs, args);
                            // println!("rule:{:?}", rule);
                            self.nu.inner_mut().add_rule(rule);
                        }
                        Err(error) => println!("no")
                    }
                }
                let map: IndexMap<String, Value> = from_value(values).expect("fail to translate json to maps");
                self.parse_files_recursive(dirname, filename, &map, schema_map, mt);
            },
            MetadataType::TUF => {}
        }
        Ok(())
    }

    

    fn handle_object(& mut self,dirname: &str, speaker: &str,key: &String, value: &Value, schema_map:& HashMap<String, Schema>, mt: &MetadataType){
        let imap: IndexMap<String, Value> = from_value(value.clone()).expect("cannot translate to map");
        // println!("{:?}", schema_map);
        if schema_map.contains_key(key){
            let sc = schema_map.get(key).unwrap();
            // println!(" key: {}, vvec:{:?}", speaker, key, value);
            match &sc.schema_type {
                SchemaTypes::RESAMPLE => {
                    // println!("{:?}",value);
                    let mut vvec = imap.values().cloned().collect::<Vec<Value>>();


                    self.handle_array(dirname, speaker, key, &vvec, schema_map, mt);
                }
                SchemaTypes::HASH => {
                    self.parse_files_recursive(dirname, speaker, &imap, schema_map, mt)
                }
                SchemaTypes::RENAME => {
                    let name = &sc.field;
                    let nv = imap.get(name).expect("get value").to_string().replace("\"", "");
                    self.parse_files_recursive(dirname, &nv, &imap, schema_map, mt)
                }
                SchemaTypes::Custom => {
                    
                    // let keys = imap.keys().cloned().collect::<Vec<String>>();
                    // if keys.len()>0{
                    //     let skey = &keys[0];
                    //     vvec.insert(0, Value::from(skey.to_string()));
                    // }    
                }
                SchemaTypes::FETCH => {

                }
                SchemaTypes::RULE => {

                }
                SchemaTypes::NONE => {}
            }
        }
    }

    fn handle_array(& mut self,dirname: &str, speaker: &str, key: &String, vvec: &Vec<Value>, schema_map:& HashMap<String, Schema>, mt: &MetadataType){
        let mut output = vec![self.nu.symbol(&Types::String(speaker.to_string()))];
        // println!("speaker: {}, key: {}, vvec:{:?}", speaker, key, vvec);
        for value in vvec{
            if value.is_object(){
                // println!("speaker: {}, key: {}", speaker, key);
                self.handle_object(dirname, speaker, key, value, schema_map, mt)
            }
            else if value.is_array(){
                let value_vec = value.as_array().unwrap();
                // println!("{:?}", value_vec);
                self.handle_array(dirname, speaker, key, value_vec, schema_map, mt)
            }
            else if value.is_number(){
                let num = value.as_i64().unwrap();
                output.push(self.nu.symbol(&Types::Number(num)));
            }else{
                let str = value.to_string().replace("\"", "");
                output.push(self.nu.symbol(&Types::String(str)));
            }
        }
        if vvec.len() == 0{
            output.push(self.nu.symbol(&Types::String("".to_string())));
        }
        if output.len() > 1{
            let args = output.into_iter().map(|s| Term::Pred(s.into())).collect();
            let rs = self.nu.symbol(&Types::String(key.to_string()));
            let rule = Rule::fact(rs, args);
            self.nu.inner_mut().add_rule(rule);
        }
    }

    fn parse_files_recursive(& mut self,dirname: &str, speaker: &str, map: & IndexMap<String, Value>, schema_map:& HashMap<String, Schema>, mt: &MetadataType){
        let sp = self.nu.symbol(&Types::String(speaker.to_string()));
        for (key, value) in map{
            if value.is_object(){
                self.handle_object(dirname, speaker, key, value, schema_map, mt);
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
                self.handle_array(dirname, speaker, key, vvec, schema_map, mt);
            }
            else if value.is_number(){
                let num = value.as_i64().unwrap();
                let ns = self.nu.symbol(&Types::Number(num));
                let args =vec![Term::Pred(sp.into()), Term::Pred(ns.into())];
                let rs = self.nu.symbol(&Types::String(key.to_string()));
                let rule = Rule::fact(rs, args);
                self.nu.inner_mut().add_rule(rule);
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
                                FetchTypes::NONE => "".to_string(),
                            };
                            if !self.file_map.contains_key(&fname){
                                let file_s = File{
                                    filename: fname.to_string(),
                                    mt: mt.clone(), 
                                    dirname: dirname.to_string()
                                };
                                self.file_map.insert(fname.to_string(), file_s);
                                let _ = self.fetch_external(dirname, &fname, mt, schema_map);
                                continue;
                            }
                            
                        }
                        _ => {}
                    }
                }
                let ss = self.nu.symbol(&Types::String(str));
                let args =vec![Term::Pred(sp.into()), Term::Pred(ss.into())];
                let rs = self.nu.symbol(&Types::String(key.to_string()));
                let rule = Rule::fact(rs, args);
                self.nu.inner_mut().add_rule(rule);
            }
        }
        
    }
}