use dash_lang::run;

#[test]
fn test_run_simple_program() {
    let source = r#"
        let x = 2 + 3
        print(x)
    "#;

    // You can redirect stdout to capture output if needed
    run(source);
}
