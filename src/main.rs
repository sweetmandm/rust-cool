mod lexer;
mod token;
mod ast;
mod parser;

fn _sample_one() -> String {
    return r#"
        class Main inherits Foo {
            (*
            test comment (*
            *)
            -- another comment
            foo: String <- "foo";
            bar: Bool <- true;
            baz: Bool <- false;
            main(): Object {};
            (* another comment *)
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

fn _sample_four() -> String {
    return r#"class Main {};"#.to_string();
}

fn main() {
    let all_test_progs = vec![
        _sample_one(),
        _sample_two(),
        _sample_three(),
        _sample_four(),
    ];

    for prog in all_test_progs {
        let lexer = lexer::Lexer::new(&prog);
        println!("---");
        println!("{:?}", prog);
        let program = parser::parse(lexer);
        match program {
            Ok(v) => println!("Res: {:?}", v),
            Err(e) => {
                println!("Err: {:?}", e);
                for token_tup in lexer::Lexer::new(&prog) {
                    println!("{:?}", token_tup);
                }
            },
        }
    }
}
