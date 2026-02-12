use std::fmt;
// Tokens
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Token {
    Number(String),
    Ident(Ident), // variable name (predefined, user-defined)
    Call,
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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "Number({})", n),
            Token::Ident(n) => write!(f, "Identifier({}", n),
            Token::Call => write!(f, "Call"),
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
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
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
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
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

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Ident {
    InputNum,
    OutputNum,
    OutputNewLine,
    UserDefined(String),
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ident::InputNum => write!(f, "InputNum"),
            Ident::OutputNum => write!(f, "OutputNum"),
            Ident::OutputNewLine => write!(f, "OutputNewLine"),
            Ident::UserDefined(s) => write!(f, "UserDfined {}", s),
        }
    }
}
