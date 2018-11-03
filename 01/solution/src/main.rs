extern crate solution;
use solution::CodeIdentifier;

fn main() {
    // let code_identifier = CodeIdentifier::new("some_var").unwrap();
    // let code_identifier = CodeIdentifier::new("some_var    ").unwrap();
    // let code_identifier = CodeIdentifier::new("    some_var").unwrap();
    // let code_identifier = CodeIdentifier::new("    some_var    ").unwrap();
    let code_identifier = CodeIdentifier::new("    SoMe_Var    ").unwrap();
    // let code_identifier = CodeIdentifier::new("    SoMe_Var__het    ").unwrap();
    // let code_identifier = CodeIdentifier::new("    SoMe_Var__het__    ").unwrap();
    // let code_identifier = CodeIdentifier::new("    __SoMe_Var__het__    ").unwrap();
    // let code_identifier = CodeIdentifier::new("    1SoMe_Var    ").unwrap();
    // let code_identifier = CodeIdentifier::new("    SoMe_V@r__het    ").unwrap();
    //let code_identifier = CodeIdentifier::new("    So Me Var    ").unwrap();
    
    let result = code_identifier.screaming_snakecase();
    println!("{:?}", result);

    let result = code_identifier.underscore();
    println!("{:?}", result);

    let result = code_identifier.kebabcase();
    println!("{:?}", result);

    let result = code_identifier.titlecase();
    println!("{:?}", result);

    let result = code_identifier.camelcase();
    println!("{:?}", result);
}