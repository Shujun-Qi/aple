use super::*;
// use pem::Pem;
// use ring::agreement::PublicKey;
// use ring::signature;

// use rsa::pkcs1::DecodeRsaPublicKey;
use serde_json::{Value, from_value, Error};
// use std::fmt::format;
// use ::signature::Result;
use std::fs;
use std::hash::{Hash, Hasher};
// use std::collections::hash_map::DefaultHasher;
use indexmap::IndexMap;
// use hex_literal::hex;
// use canonical_json;
// use spki::SubjectPublicKeyInfo;
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::DecodePublicKey, pkcs8::EncodePublicKey, pkcs8::DecodePrivateKey, pkcs8::EncodePrivateKey};
use rsa::pss::{BlindedSigningKey, VerifyingKey, Signature, SigningKey};
use rsa::signature::{Keypair,RandomizedSigner, SignatureEncoding, Verifier};
use rsa::sha2::{Digest, Sha256, Sha512, Sha384};
use rsa::pkcs8::LineEnding;
use rsa::rand_core::OsRng;

use std::str;
use chrono::prelude::*;


#[test]
fn test_signature(){
    
    let dirname = "test/DemoData/Intoto/debian/";
    // let _ = parse_files(filename);
    // test_pkg(filename);
    // generate_keys();
    // test_sig();
    // generate_sigs(dirname);
    // validate_sigs(dirname);
    test_time();
}

fn parse_files(filename: &str) -> Result<(),rsa::pkcs8::spki::Error>{
    
    let result = get_msg(filename).unwrap();
    let msg = result.as_bytes();
    let key = "-----BEGIN PUBLIC KEY-----\nMIIBojANBgkqhkiG9w0BAQEFAAOCAY8AMIIBigKCAYEAtirZR6ujYl2qQAo0O5aB\ndTCPYXrBgNGqp2+IqDRRCV3vH+SPR217TPKCUGvHYbT8UYPA/gmdlE+lQGPRH76i\n7BTCpdT25OpXnXgGoORcUeH7X8QZ0iOxRZz1KN+wQ2l65DWRkcmgD1UtKmbPBAD4\nprHtuwOZ1s3SZFPzGIQXWoMDa3sbb+fisTyviqiGsRiZ2T1mnvCC2HEX7ekUuJCU\n2jqiA5UXHR5AznEpupQeyPOQUEi7QbeMuRnzc9rrElfZHsn9HoNO6l4ltAD2uYbS\n0tWacf/1+lfXN97KiBzfXMPaSzLy6L+kh+olyHaa7fciq1KFnsXEq7I0boLZLBnp\nJ2yv4q+BwF9YdotSlkXv4DF+FysOpllwq5lehZhgVyGrQD6jAoHDth4CAjTCCjp0\nCHoz8+e+RsedQY1xtB402SA07+j2wJXHHE9XkE9vMjJTw3t+fuY8IptvC7MVJMvk\nFc8pV35FdqTulN2yM6fMOpsvQdg7pAquJh/CHtD/R9LtAgMBAAE=\n-----END PUBLIC KEY-----";
    
    // let binding = parse(key).unwrap();
    // let pem = binding.contents();
    // println!("{:?}", pem);

    let sig = "9c3a5609e79ea68f9d7aa4b72e5ab1f00b7773f06912b789add45da026f46621b9ed06739bec692e7ea33ffc4a6dbd1e54c1e37cd01de233eeed924da836b0ab7c3893752e011b0d95827eff5644d9f139ed5873ed065834328efb837032299a55e27aa4679ecf6f6ae9b4754112a951cdad9f0a54607c19bab67c9f7460b1e29caa8d31ff8f97f38268c4facc53d6a364d00010bb619d891d36d01fa69607e3c230aa6d2d2f8ac3ae9f9a5568d2503e00d8080111b4563594f2fecaef0630882d45c82b818b822ab0fac216eae5cc034e66547b9f6d80364924a8c5fd2011d9766a932fd5805d947aa1a931032dceca4fcb5d89714248c01ca5b87b8bdd603695221ca65410e8c93a7a1a394f59f798307c5f0ab3c56c4ead18f920c7836643c0519e8d33cb6141a6e776fa17d1a05a6009198d5fe4c3839471007a40345a12645f9dd0e274e588dfbc3a2171bcbf822509f0b2b4d8ea85c06be9a4570619807e40bfc1988467d2e9390a40cae186dcfc523bccf0f93e08fb65f6fdc8416e2a";
    
    // let pub_key = fs::read(pub_key_path).unwrap();
    // let pub_key:SubjectPublicKeyInfo<_, _> = SubjectPublicKeyInfo::try_from(key.as_bytes()).unwrap();
    // let pubkey = signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256, pub_key.subject_public_key.to_owned());
    // let pkey = signature::RsaSubjectPublicKey::try_from(key.as_bytes());
    // let public_key = RsaPublicKey::from_public_key_pem(key)?;
    // // println!("{:?}",public_key);
    // // let msg = contents.as_bytes();
    // // let verifying_key = VerifyingKey::from_public_key_pem(key)?
    // let sig_bytes = Signature::try_from(sig.as_bytes()).unwrap();
    // let verifying_key: VerifyingKey<Sha256> = VerifyingKey::from(public_key);  
    // println!("{:?}", sig_bytes);  
    // println!("{:?}", verifying_key);
    // verifying_key.verify(msg, &sig_bytes).expect("failed to verify");
    // let pubkey = signature::UnparsedPublicKey::<Vec<u8>>::new(
    //     &signature::RSA_PSS_2048_8192_SHA512,
    //     pub_key.subject_public_key.to_owned(),
    // );
    // Sign
    // let data = b"hello world";
    // let signature = signing_key.sign_with_rng(&mut rng, data);
    // assert_ne!(signature.to_bytes().as_ref(), data);

    // // Verify
    // verifying_key.verify(data, &signature).expect("failed to verify");
    Ok(())
}

fn get_msg(filename: &str) -> serde_json::Result<String>{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let json_content: Value = serde_json::from_str(&contents)?;
    if json_content.is_object(){
        let map: IndexMap<String, Value> = from_value(json_content)?;
        let result = map.get("signed").unwrap().to_string();
        // println!("{:?}", result);
        // let cj_res = canonical_json::to_string(map.get("signed").unwrap()).unwrap();
        // println!("{:?}", cj_res);
        return Ok(result);
    }
    Ok("failed".to_string())
}

fn get_sig(filename: &str) -> serde_json::Result<String>{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let json_content: Value = serde_json::from_str(&contents)?;
    if json_content.is_object(){
        let map: IndexMap<String, Value> = from_value(json_content)?;
        let result = map.get("signatures").unwrap();
        if result.is_array(){
            let sig = result[0]["sig"].to_string();
            // println!("{}", sig);
            return Ok(sig);
        }
    }
    Ok("failed".to_string())
}

fn test_sig(){
    let pk_file = "test/keys/privatekey.pem";
    let vk_file = "test/keys/publickey.pem";

    let pub_key = fs::read_to_string(vk_file).unwrap();
    let public_key = RsaPublicKey::from_public_key_pem(pub_key.as_str()).unwrap();
    let verifying_key: VerifyingKey<Sha256> = VerifyingKey::from(public_key);

    let prv_key = fs::read_to_string(pk_file).unwrap();
    let private_key = RsaPrivateKey::from_pkcs8_pem(&prv_key).unwrap();
    let signing_key: SigningKey<Sha256> = SigningKey::from(private_key);
    let data = b"hello world";


    let signature = signing_key.sign_with_rng(&mut OsRng, data);
    let sig_bytes = signature.to_vec();
    // let sig_str = base64::encode(&sig_bytes);
    let sig_str = hex::encode(&sig_bytes);
    println!("{:?}", sig_str);
    verifying_key.verify(data, &signature).expect("verify");
}

fn generate_keys(){
    // let mut rng = rand::thread_rng();
    let bits = 4096;
    let private_key = RsaPrivateKey::new(&mut OsRng, bits).expect("failed to generate a key");
    let signing_key = BlindedSigningKey::<Sha256>::new(private_key);
    let verifying_key = signing_key.verifying_key();

    let pk_file = "test/keys/privatekey.pem";
    let vk_file = "test/keys/publickey.pem";
    let sk_str = signing_key.write_pkcs8_pem_file(pk_file,LineEnding::default()).unwrap();
    let vk_str = verifying_key.write_public_key_pem_file(vk_file, LineEnding::LF).unwrap();
    let vk_s = verifying_key.to_public_key_pem(LineEnding::LF).unwrap();
    println!("{:?}", vk_s);
    let mut hasher = Sha256::new();
    hasher.update(vk_s);
    let result = hasher.finalize();
    let hash = format!("{:x}", result);
    println!("{:?}", hash);
}

fn generate_sigs(dirname: &str){
    let pk_file = "test/keys/privatekey.pem";
    // let vk_file = "test/keys/publickey.pem";

    // let pub_key = fs::read_to_string(vk_file).unwrap();
    // let public_key = RsaPublicKey::from_public_key_pem(pub_key.as_str()).unwrap();
    // let verifying_key: VerifyingKey<Sha256> = VerifyingKey::from(public_key);

    let prv_key = fs::read_to_string(pk_file).unwrap();
    let private_key = RsaPrivateKey::from_pkcs8_pem(&prv_key).unwrap();
    let signing_key: SigningKey<Sha256> = SigningKey::from(private_key);

    let paths = fs::read_dir(dirname).unwrap();
    for path in paths {
        let path_name = path.unwrap().path();
        let filename = path_name.as_os_str().to_str().unwrap();
        let data_str = get_msg(filename).unwrap();
        let data_bytes = data_str.as_bytes();
        println!("file: {}", filename);
        // println!("Data str: {}", data_str);
        // println!("Data bytes: {:?}", data_bytes);
        let signature = signing_key.sign_with_rng(&mut OsRng, data_bytes);
        let sig_bytes = signature.to_vec();
        let sig_str = hex::encode(&sig_bytes);
        println!("sig: {:?}", sig_str);
    }
}

fn validate_sigs(dirname: &str){
    // let pk_file = "test/keys/privatekey.pem";
    let vk_file = "test/keys/publickey.pem";

    let pub_key = fs::read_to_string(vk_file).unwrap();
    let public_key = RsaPublicKey::from_public_key_pem(pub_key.as_str()).unwrap();
    let verifying_key: VerifyingKey<Sha256> = VerifyingKey::from(public_key);

    // let prv_key = fs::read_to_string(pk_file).unwrap();
    // let private_key = RsaPrivateKey::from_pkcs8_pem(&prv_key).unwrap();
    // let signing_key: SigningKey<Sha256> = SigningKey::from(private_key);

    let paths = fs::read_dir(dirname).unwrap();
    for path in paths {
        let path_name = path.unwrap().path();
        let filename = path_name.as_os_str().to_str().unwrap();
        let sig_str = get_sig(filename).unwrap().replace("\"", "");
        let sig_bytes = hex::decode(sig_str).unwrap();
        let signature = Signature::try_from(sig_bytes.as_ref()).unwrap();

        let data_str = get_msg(filename).unwrap();
        let data_bytes = data_str.as_bytes();
        verifying_key.verify(data_bytes, &signature).expect("verify");
    }
}

fn test_time(){
    let time = std::time::SystemTime::now();
    let now: DateTime<Utc> = time.into();
    let now = now.to_rfc3339_opts(SecondsFormat::Millis, true);

    println!("{}", now);
    let bn = DateTime::parse_from_rfc3339(&now).unwrap();
    let utc = DateTime::<Local>::from(bn).timestamp();
    println!("{}", bn);
    println!("{}", utc);
}