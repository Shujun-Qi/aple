use super::*;

#[test]
fn compiler(){
    let mut cp = Compiler::new();
    cp.compile("test/DemoData/Intoto/debian/", MetadataType::INTOTO);
    cp.print_nu();
}