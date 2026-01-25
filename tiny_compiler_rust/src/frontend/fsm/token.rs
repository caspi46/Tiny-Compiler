use std::fmt;
// Tokens
#[derive(Clone, Debug)]
pub enum Token {
    Number(String),
    Ident(String),
    Let(String),
    If(String),
    Then(String),
    Else(String),
    Fi(String),
    While(String),
    Do(String),
    Od(String),
    Return(String),
    Var(String),
    Void(String),
    Function(String),
    Main(String),
    // Ops
    RelOp(RelOp),
    Op(Op),
    // Symbol
    Symbol(Symbol),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "Number({})", n),
            Token::Ident(n) => write!(f, "Identifier({}", n),
            Token::Let(n) => write!(f, "Let({})", n),
            Token::If(n) => write!(f, "If({})", n),
            Token::Then(n) => write!(f, "Then({})", n),
            Token::Else(n) => write!(f, "Else({})", n),
            Token::Fi(n) => write!(f, "Fi({})", n),
            Token::While(n) => write!(f, "While({})", n),
            Token::Do(n) => write!(f, "Do({})", n),
            Token::Od(n) => write!(f, "Od({})", n),
            Token::Return(n) => write!(f, "Return({})", n),
            Token::Var(n) => write!(f, "Var({})", n),
            Token::Void(n) => write!(f, "Void({})", n),
            Token::Main(n) => write!(f, "Main({})", n),
            Token::RelOp(n) => write!(f, "RelOp({})", n),
            Token::Op(n) => write!(f, "Op({})", n),
            Token::Symbol(n) => write!(f, "Symbol({})", n),
            Token::Function(n) => write!(f, "Function({})", n),
        }
    }
}

#[derive(Clone, Debug)]
pub enum RelOp {
    EQ(String), // ==
    NE(String), // !=
    GT(String), // >
    LT(String), // <
    GE(String), // >=
    LE(String), // <=
}

impl fmt::Display for RelOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelOp::EQ(o) => write!(f, "EQ({})", o),
            RelOp::NE(o) => write!(f, "NE({})", o),
            RelOp::GT(o) => write!(f, "GT({})", o),
            RelOp::LT(o) => write!(f, "LT({})", o),
            RelOp::GE(o) => write!(f, "GE({})", o),
            RelOp::LE(o) => write!(f, "LE({})", o),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Op {
    ADD(String), // +
    SUB(String), // -
    DIV(String), // /
    MUL(String), // *
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::ADD(o) => write!(f, "ADD({})", o),
            Op::SUB(o) => write!(f, "SUB({})", o),
            Op::DIV(o) => write!(f, "DIV({})", o),
            Op::MUL(o) => write!(f, "MUL{})", o),
        }
    }
}
// Symbol
#[derive(Clone, Debug)]
pub enum Symbol {
    OpenParen(String),  // (
    CloseParen(String), // )
    OpenBrace(String),  // {
    CloseBrace(String), // }
    Init(String),       // <-
    SemiColon(String),  // ;
    Period(String),     // .
    Comma(String),      // ,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::OpenParen(s) => write!(f, "OpenParen({})", s),
            Symbol::CloseParen(s) => write!(f, "CloseParen({})", s),
            Symbol::OpenBrace(s) => write!(f, "OpenBrace({})", s),
            Symbol::CloseBrace(s) => write!(f, "CloseBrace({})", s),
            Symbol::Init(s) => write!(f, "Init({})", s),
            Symbol::SemiColon(s) => write!(f, "SemiColon({})", s),
            Symbol::Period(s) => write!(f, "Period({})", s),
            Symbol::Comma(s) => write!(f, "Comma({})", s),
        }
    }
}

// Phi Function..?
enum Phi {
    x,
    y,
}
