use plex::parser;
use crate::token::Token::*;
use crate::token::Token;
use crate::lexer::Loc;
use crate::ast::*;

parser! {
    fn parse_(Token, Loc);

    (a, b) {
        Loc {
            start: a.start,
            end: b.end,
        }
    }

    program: Program {
        class[c] => Program { class_list: vec![c] },
        class_list[c] => Program { class_list: c }
    }

    class_list: Vec<Class> {
        class_list[mut cl] class[c] => {
            cl.push(c);
            cl
        }
    }

    class: Class {
        Class_ Typeid Lbrace feature_list[f] Rbrace Semicolon => Class {
            feature_list: f
        },
        Class_ Typeid Inherits Typeid Lbrace feature_list[f] Rbrace Semicolon => Class {
            feature_list: f,
        },
    }

    feature_list: Vec<Feature> {
        // empty class:
        => vec![],
        // non-empty class:
        feature_list[mut fl] feature[f] Semicolon => {
            fl.push(f);
            fl
        }
    }

    expr: Expr {
        Objectid(oid) => Expr::Identifier(oid),
        BoolConst(val) => Expr::Boolean(val),
        IntConst(val) => Expr::Int(val.parse().unwrap()),
        StrConst(val) => Expr::Str(val),
        Objectid(oid) Colon Typeid(tid) Assign expr[e] => Expr::Assignment(
            Assignment {
                oid: oid,
                tid: tid,
                expr: Box::new(Some(e)),
            }
        ),
    }

    maybe_expr: Option<Expr> {
        => None,
        expr[e] => Some(e),
    }

    arg_list: Vec<Arg> {
        arg_list[mut a] StrConst(id) Typeid(t) => {
            a.push((id, t));
            a
        }
    }

    feature: Feature {
        // Attributes
        Objectid(oid) Colon Typeid(tid) => {
            Feature::Attribute(Assignment {
                oid: oid,
                tid: tid,
                expr: Box::new(None)
            })
        },
        Objectid(oid) Colon Typeid(tid) Assign expr[e] => {
            Feature::Attribute(Assignment {
                oid: oid,
                tid: tid,
                expr: Box::new(Some(e))
            })
        },
        // Methods
        Objectid(oid) Lparen Rparen Colon Typeid(rtype) Lbrace maybe_expr[e] Rbrace => {
            Feature::Method(oid, Box::new(vec![]), rtype, Box::new(e))
        },
        Objectid(oid) Lparen arg_list[a] Rparen Colon Typeid(rtype) Lbrace maybe_expr[e] Rbrace => {
            Feature::Method(oid, Box::new(a), rtype, Box::new(e))
        },
    }
}

pub fn parse<I: Iterator<Item = (Token, Loc)>>(
    i: I,
) -> Result<Program, (Option<(Token, Loc)>, &'static str)> {
    parse_(i)
}
