#[derive(Debug)]
pub struct Program {
    pub class_list: Vec<Class>
}

#[derive(Debug)]
pub struct Class {
    pub feature_list: Vec<Feature>
}

#[derive(Debug)]
pub enum Feature {
    Attribute(Assignment),
    Method(Identifier, Box<Vec<Arg>>, Type, Box<Option<Expr>>),
}

pub type Identifier = String;
pub type Type = String;
pub type Boolean = bool;
pub type Int = u32;
pub type Str = String;

#[derive(Debug)]
pub struct Assignment {
    pub oid: Identifier,
    pub tid: Type,
    pub expr: Box<Option<Expr>>,
}

#[derive(Debug)]
pub struct CaseBranch {
    id: Identifier,
    t: Box<Type>,
    expr: Box<Expr>,
}

#[derive(Debug)]
pub enum Term {
    Boolean(Boolean),
    Int(Int),
    Str(Str),
    Identifier(Identifier),
}

pub type Arg = (Identifier, Type);

#[derive(Debug)]
pub enum MathOp {
    Add,
    Subtract,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expr {
    Identifier(Identifier),
    Boolean(Boolean),
    Int(Int),
    Str(Str),
    Assignment(Assignment),
    Dispatch {
        target: Box<Option<Expr>>,
        targettype: Option<Identifier>,
        id: Identifier,
        exprs: Box<Vec<Expr>>,
    },
    Conditional {
        test: Box<Expr>,
        then: Box<Expr>,
        orelse: Box<Expr>,
    },
    While {
        test: Box<Expr>,
        then: Box<Expr>,
    },
    Block(Box<Vec<Expr>>),
    Let(Box<Vec<Assignment>>),
    Case(Box<Expr>, Box<Vec<CaseBranch>>),
    New(Type),
    Isvoid(Box<Expr>),
    Math {
        lhs: Box<Expr>,
        op: Box<MathOp>,
        rhs: Box<Expr>,
    },
}
