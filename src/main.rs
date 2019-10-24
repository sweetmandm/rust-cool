mod lexer;
mod token;

fn _sample_one() -> String {
    return r#"
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
    "#.to_string();
}

fn _sample_two() -> String {
    return r#"
        class Main {
            (* This is an unterminated comment.
    "#.to_string();
}

fn _sample_three() -> String {
    return r#"
        class Main {
            foo: String <- "foo\
            bar";
        };
    "#.to_string();
}

fn main() {
    let sample = _sample_three();
    let lexer = lexer::Lexer::new(&sample);
    for token_tup in lexer {
        println!("token: {:?}", token_tup);
    }
}
