#[macro_use]
extern crate lalrpop_util;

mod lexer;
mod token;
mod ast;
//mod parser;

lalrpop_mod!(pub cool);

fn main() {
    let all_test_progs = vec![
        _sample_0(),
        _sample_1(),
        _sample_2(),
        _sample_3(),
    ];

    for (i, source) in all_test_progs.iter().enumerate() {
        println!("--- sample {}", i);
        let lexer = lexer::Lexer::new(&source);
        let program = cool::ProgramParser::new().parse(lexer);
        match program {
            Ok(v) => println!("Res: {:?}", v),
            Err(e) => {
                println!("Err: {:?}", e);
                for token_tup in lexer::Lexer::new(&source) {
                    println!("{:?}", token_tup);
                }
            },
        }
    }
}

fn _sample_0() -> String {
    return r#"
        class Main inherits Foo {
            (*
            test comment (*
            *)
            -- another comment
            foo: String <- "foo";
            bar: Bool <- true;
            baz: Bool <- false;
            math: Int <- 4 + 10 * 3;
            main(): Object {};
            (* another comment *)
        };
    "#.to_string();
}

fn _sample_1() -> String {
    return r#"
        class Main {
            (* This is an unterminated comment.
    "#.to_string();
}

fn _sample_2() -> String {
    return r#"
        class Main {
            foo: String <- "foo\
            bar";
        };
    "#.to_string();
}

fn _sample_3() -> String {
    return r#"class Main {};"#.to_string();
}
