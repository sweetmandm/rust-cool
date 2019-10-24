mod lexer;
mod token;


fn main() {
    let sample = r#"
        class Main inherits Foo {
            (*
            test comment
            *)
            -- another comment
            foo: String <- "foo";
            bar: Bool <- true;
            baz: Bool <- false;
            main(): Object {};
        };
    "#;
    let lexer = lexer::Lexer::new(&sample);
    for token_tup in lexer {
        println!("token: {:?}", token_tup);
    }
}
