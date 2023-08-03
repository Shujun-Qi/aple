use super::*;

#[test]
fn compiler(){
    let mut cp = Compiler::new();
    cp.initial();
    // cp.compile("test/DemoData/Intoto/debian/", MetadataType::INTOTO);
    // let result = cp.query("validate($f).");
    // if result.state{
    //     println!("Found solutions:");
    //     println!("{:?}", result.results);
    // }
    // else{
    //     println!("No compliant result");
    // }
    cp.print_nu("test/Example_Output/Intoto.log");
}