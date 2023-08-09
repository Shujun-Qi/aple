use std::result;
use std::time::SystemTime;
use super::*;

#[test]
fn test_intoto(){
    let mut cp = Compiler::new();
    cp.initial();
    cp.compile("test/DemoData/Intoto/debian/", MetadataType::INTOTO);
    // cp.compile("test/DemoData/Tuf/debian/", MetadataType::TUF);
    // let result = cp.query("validate_tuf($f).");
    let start_intoto = SystemTime::now();
    let result = cp.query("validate_intoto($f).",true);
    match start_intoto.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("Time: {} ms", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    if result.state{
        if result.results.len() > 0{
            println!("Found compliant image:");
            println!("{:?}", result.results);
        }
        else{
            println!("Policy Enforced!");
        }
    }
    else{
        println!("No compliant result");
    }
    cp.print_nu("test/Example_Output/Intoto.log");
    // cp.print_nu("test/Example_Output/Tuf.log");
}

#[test]
fn test_tuf(){
    let mut cp = Compiler::new();
    cp.initial();
    // cp.compile("test/DemoData/Intoto/debian/", MetadataType::INTOTO);
    cp.compile("test/DemoData/Tuf/debian/", MetadataType::TUF);
    let start_tuf = SystemTime::now();
    let result = cp.query("validate_tuf(debian).", false);
    match start_tuf.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("Time: {} ms", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    // let result = cp.query("validate_intoto($f).");
    if result.state{
        if result.results.len() > 0{
            println!("Found compliant image:");
            println!("{:?}", result.results);
        }
        else{
            println!("Policy Enforced!");
        }
    }
    else{
        println!("No compliant result");
    }
    // cp.print_nu("test/Example_Output/Intoto.log");
    cp.print_nu("test/Example_Output/Tuf.log");
}

#[test]
fn test_integration(){
    let mut cp = Compiler::new();
    cp.initial();
    cp.compile("test/DemoData/Intoto/debian/", MetadataType::INTOTO);
    cp.compile("test/DemoData/Tuf/debian/", MetadataType::TUF);
    cp.load_policy("test/Policy/tuf_intoto.policy");
    let start_tuf = SystemTime::now();
    let result = cp.query("validate($f).", false);
    match start_tuf.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("Time: {} ms", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    // let result = cp.query("validate_intoto($f).");
    if result.state{
        if result.results.len() > 0{
            println!("Found compliant image:");
            println!("{:?}", result.results);
        }
        else{
            println!("Policy Enforced!");
        }
    }
    else{
        println!("No compliant result");
    }
    // cp.print_nu("test/Example_Output/Intoto.log");
    cp.print_nu("test/Example_Output/Integration.log");
}