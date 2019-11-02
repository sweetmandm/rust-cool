#[derive(Debug)]
pub struct Program {
    pub class_list: Vec<Class>
}

#[derive(Debug)]
pub struct Class {
    pub inherits: Option<String>,
    pub name: String,
    pub feature_list: Vec<Feature>,
}

#[derive(Debug)]
pub enum Feature {
    Attribute(VarDecl),
    Method(Identifier, Box<Vec<ArgDecl>>, Type, Box<Option<Expr>>),
}

pub type Identifier = String;
pub type Type = String;
pub type Boolean = bool;
pub type Int = u32;
pub type Str = String;

#[derive(Debug)]
pub struct VarDecl {
    pub oid: Identifier,
    pub tid: Type,
    pub expr: Box<Option<Expr>>,
}

#[derive(Debug)]
pub struct CaseBranch {
    pub id: Identifier,
    pub tid: Type,
    pub expr: Box<Expr>,
}

pub type ArgDecl = (Identifier, Type);

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
    Bool(Boolean),
    Int(Int),
    Str(Str),
    Assignment(Identifier, Box<Expr>),
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
        exec: Box<Expr>,
    },
    Block(Box<Vec<Expr>>),
    Let(Box<Vec<VarDecl>>, Box<Expr>),
    Case(Box<Expr>, Box<Vec<CaseBranch>>),
    New(Type),
    Isvoid(Box<Expr>),
    Math {
        lhs: Box<Expr>,
        op: Box<MathOp>,
        rhs: Box<Expr>,
    },
    Paren(Box<Expr>),
}
