use crate::token::Token;
use plex::lexer;

const MAX_STR_CONST: usize = 1025;

lexer! {
    fn next_token(text: 'a) -> Token;
    "class" => Token::Class_,
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

    r#""[^"]*""# => escape_string(text),
    "[0-9]+" => Token::IntConst(text.to_owned()),
    "(true)|(false)" => Token::BoolConst(text == "true"),
    "[A-Z][a-zA-Z0-9_]*" => Token::Typeid(text.to_owned()),
    "[a-z][a-zA-Z0-9_]*" => Token::Objectid(text.to_owned()),

    r"[ \t\r\n]" => Token::Whitespace,
    r"--[^\n\r]*" => Token::Comment,
    r"\(\*(~(.*\*\).*))\*\)" => Token::Comment,
    r"\(\*(~(.*\*\).*))" => Token::Error("EOF in comment".to_string()),
    r"\*\)" => Token::Error("Unmatched *)".to_string()),

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
    r"\." => Token::Period,
    "." => Token::Error(format!("Unexpected character: {}", text.to_owned())),
}

fn escape_string(text: &str) -> Token {
    // " a b \ n c d " -> " a b \n c d "
    // " a b \ \ c d " -> " a b \\ c d "
    // " a b \ c d " -> " a b c d "
    // " a b \n c d -> Error
    // " a b \0 c d -> Error
    let mut val = "".to_string();
    let mut esc = false;
    if text.len() > MAX_STR_CONST {
        return Token::Error("String constant too long".to_string());
    }
    for c in (&text[1..text.len() - 1]).chars() {
        match c {
            '\n' => {
                // Allow an escaped newline, but not an unescaped newline.
                if !esc {
                    return Token::Error("Unterminated string constant".to_string());
                }
            }
            '\0' => return Token::Error("String contains null character".to_string()),
            _ => {}
        }
        if !esc && c == '\\' {
            // Enter escape character state.
            esc = true;
            continue;
        }
        if !esc {
            val.push(c);
            continue;
        }
        // Resolve escape character state.
        match c {
            'b' => val.push('\x08'),
            't' => val.push('\t'),
            'n' => val.push('\n'),
            'f' => val.push('\x0A'),
            _ => val.push(c),
        }
        esc = false;
    }
    return Token::StrConst(val);
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
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
