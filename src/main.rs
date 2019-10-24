extern crate plex;

mod lexer {
    use plex::lexer;

    const MAX_STR_CONST: i64 = 1025;

    #[derive(Debug, Clone)]
    pub enum Token {
        Class,
        Else,
        Fi,
        If,
        In,
        Inherits,
        Let,
        Loop,
        Pool,
        Then,
        While,
        Case,
        Esac,
        Of,
        New,
        Isvoid,
        Not,
        StrConst(String),
        IntConst(String),
        BoolConst(bool),
        Typeid(String),
        Objectid(String),
        Darrow,
        Assign,
        Le,
        LetStmt,
        Whitespace,
        Comment,
        Lbrace,
        Rbrace,
        Lparen,
        Rparen,
        Colon,
        Semicolon,
        At,
        Plus,
        Minus,
        Divide,
        Mul,
        Neg,
        Equal,
        Lt,
        Period,
        Error(String),
    }

    lexer! {
        fn next_token(text: 'a) -> Token;
        "class" => Token::Class,
        "else" => Token::Else,
        "fi" => Token::Fi,
        "if" => Token::If,
        "in" => Token::In,
        "inherits" => Token::Inherits,
        "let" => Token::Let,
        "loop" => Token::Loop,
        "pool" => Token::Pool,
        "then" => Token::Then,
        "while" => Token::While,
        "case" => Token::Case,
        "esac" => Token::Esac,
        "of" => Token::Of,
        "new" => Token::New,
        "isvoid" => Token::Isvoid,
        "not" => Token::Not,

        r#""[^"]*""# => Token::StrConst((&text[1..text.len()-1]).to_string()),
        "[0-9]+" => Token::IntConst(text.to_owned()),
        "(true)|(false)" => Token::BoolConst(text == "true"),
        "[A-Z][a-zA-Z0-9_]*" => Token::Typeid(text.to_owned()),
        "[a-z][a-zA-Z0-9_]*" => Token::Objectid(text.to_owned()),

        r"[ \t\r\n]" => Token::Whitespace,
        r"--[^\n\r]*" => Token::Comment,
        r"\(\*.*\*\)" => Token::Comment,

        "=>" => Token::Darrow,
        "<-" => Token::Assign,
        "<=" => Token::Le,

        "{" => Token::Lbrace,
        "}" => Token::Rbrace,
        r"\(" => Token::Lparen,
        r"\)" => Token::Rparen,
        ":" => Token::Colon,
        ";" => Token::Semicolon,
        "@" => Token::At,
        r"\+" => Token::Plus,
        "-" => Token::Minus,
        "/" => Token::Divide,
        r"\*" => Token::Mul,
        r"\~" => Token::Neg,
        "=" => Token::Equal,
        "<" => Token::Lt,
        "." => Token::Period,
        "." => Token::Error(format!("Unexpected character: {}", text.to_owned())),
    }

    pub struct Lexer<'a> {
        original: &'a str,
        remaining: &'a str,
    }

    impl <'a> Lexer<'a> {
        pub fn new(text: &'a str) -> Lexer<'a> {
            Lexer {
                original: text,
                remaining: text,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Loc {
        pub start: usize,
        pub end: usize,
    }

    impl<'a> Iterator for Lexer<'a> {
        type Item = (Token, Loc);
        fn next(&mut self) -> Option<(Token, Loc)> {
            loop {
                if let Some((token, rem)) = next_token(self.remaining) {
                    let start = self.original.len() - self.remaining.len();
                    let end = self.original.len() - rem.len();
                    self.remaining = rem;
                    match token {
                        Token::Comment | Token::Whitespace => continue,
                        token => return Some((token, Loc { start, end })),
                    }
                } else {
                    return None;
                };
            }
        }
    }
}

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
