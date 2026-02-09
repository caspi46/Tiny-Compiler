use std::fmt;
// Tokens
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(String),
    Ident(String), // variable name
    Let,
    If,
    Then,
    Else,
    Fi,
    While,
    Do,
    Od,
    Return,
    Var,
    Void,
    Function,
    Main,
    // Ops
    RelOp(RelOp),
    Op(Op),
    // Symbol
    Symbol(Symbol),
    PreDefFunc(PreDefFunc),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "Number({})", n),
            Token::Ident(n) => write!(f, "Identifier({}", n),
            Token::Let => write!(f, "Let"),
            Token::If => write!(f, "If"),
            Token::Then => write!(f, "Then"),
            Token::Else => write!(f, "Else"),
            Token::Fi => write!(f, "Fi"),
            Token::While => write!(f, "While"),
            Token::Do => write!(f, "Do"),
            Token::Od => write!(f, "Od"),
            Token::Return => write!(f, "Return"),
            Token::Var => write!(f, "Var"),
            Token::Void => write!(f, "Void"),
            Token::Main => write!(f, "Main"),
            Token::RelOp(n) => write!(f, "RelOp({})", n),
            Token::Op(n) => write!(f, "Op({})", n),
            Token::Symbol(n) => write!(f, "Symbol({})", n),
            Token::Function => write!(f, "Function"),
            Token::PreDefFunc(n) => write!(f, "Predefined Function({})", n),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelOp {
    EQ, // ==
    NE, // !=
    GT, // >
    LT, // <
    GE, // >=
    LE, // <=
}

impl fmt::Display for RelOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelOp::EQ => write!(f, "EQ"),
            RelOp::NE => write!(f, "NE"),
            RelOp::GT => write!(f, "GT"),
            RelOp::LT => write!(f, "LT"),
            RelOp::GE => write!(f, "GE"),
            RelOp::LE => write!(f, "LE"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    ADD, // +
    SUB, // -
    DIV, // /
    MUL, // *
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::ADD => write!(f, "ADD"),
            Op::SUB => write!(f, "SUB"),
            Op::DIV => write!(f, "DIV"),
            Op::MUL => write!(f, "MUL"),
        }
    }
}
// Symbol
#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    OpenParen,  // (
    CloseParen, // )
    OpenBrace,  // {
    CloseBrace, // }
    Init,       // <-
    SemiColon,  // ;
    Period,     // .
    Comma,      // ,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::OpenParen => write!(f, "OpenParen"),
            Symbol::CloseParen => write!(f, "CloseParen"),
            Symbol::OpenBrace => write!(f, "OpenBrace"),
            Symbol::CloseBrace => write!(f, "CloseBrace"),
            Symbol::Init => write!(f, "Init"),
            Symbol::SemiColon => write!(f, "SemiColon"),
            Symbol::Period => write!(f, "Period"),
            Symbol::Comma => write!(f, "Comma"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PreDefFunc {
    InputNum,
    OutputNum,
    OutputNewLine,
}

impl fmt::Display for PreDefFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreDefFunc::InputNum => write!(f, "InputNum"),
            PreDefFunc::OutputNum => write!(f, "OutputNum"),
            PreDefFunc::OutputNewLine => write!(f, "OutputNewLine"),
        }
    }
}
