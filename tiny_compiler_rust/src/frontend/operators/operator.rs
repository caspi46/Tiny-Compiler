use std::fmt;
// Operators
#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub enum Operator {
    //Constant
    Const(i32),
    // Op
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div(i32, i32),
    // Compare
    Cmp(i32, i32),
    // Phi function
    Phi(i32, i32),
    // End
    End,
    // RelOp
    Bra(i32),
    Bne(i32, Option<i32>),
    Ble(i32, Option<i32>),
    Bge(i32, Option<i32>),
    Bgt(i32, Option<i32>),
    Beq(i32, Option<i32>),
    Blt(i32, Option<i32>),

    // Read & Write
    Read,
    Write(i32),
    WriteNL,
    // Operator for user-defined functions
    Jsr(i32), // block name
    Ret(i32),
    // For now, only three parameters
    GetPar1,
    GetPar2,
    GetPar3,
    SetPar1(i32),
    SetPar2(i32),
    SetPar3(i32),
    EMPTY, // For branch connection
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Const(x) => write!(f, "Const={}", x),

            Operator::Add(x, y) => write!(f, "add ({}) ({})", x, y),
            Operator::Sub(x, y) => write!(f, "sub ({}) ({})", x, y),
            Operator::Mul(x, y) => write!(f, "mul ({}) ({})", x, y),
            Operator::Div(x, y) => write!(f, "div ({}) ({})", x, y),

            Operator::Cmp(x, y) => write!(f, "cmp ({}) ({})", x, y),

            Operator::Phi(x, y) => write!(f, "phi ({}) ({})", x, y),

            Operator::End => write!(f, "end"),

            Operator::Bra(x) => write!(f, "bra ({})", x),
            Operator::Bne(x, Some(y)) => write!(f, "bne ({}) ({})", x, y),
            Operator::Ble(x, Some(y)) => write!(f, "ble ({}) ({})", x, y),
            Operator::Bge(x, Some(y)) => write!(f, "bge ({}) ({})", x, y),
            Operator::Bgt(x, Some(y)) => write!(f, "bgt ({}) ({})", x, y),
            Operator::Beq(x, Some(y)) => write!(f, "beq ({}) ({})", x, y),
            Operator::Blt(x, Some(y)) => write!(f, "blt ({}) ({})", x, y),

            Operator::Read => write!(f, "read"),
            Operator::Write(x) => write!(f, "write ({})", x),
            Operator::WriteNL => write!(f, "writeNL"),

            Operator::Jsr(b) => write!(f, "jsr {}", b),
            Operator::Ret(x) => write!(f, "ret {}", x),

            Operator::GetPar1 => write!(f, "getPar1"),
            Operator::GetPar2 => write!(f, "getPar2"),
            Operator::GetPar3 => write!(f, "getPar3"),

            Operator::SetPar1(x) => write!(f, "setPar1 ({})", x),
            Operator::SetPar2(x) => write!(f, "setPar2 ({})", x),
            Operator::SetPar3(x) => write!(f, "setPar3 ({})", x),

            Operator::EMPTY => write!(f, "empty"),
            _ => write!(f, "Error detected for None"),
        }
    }
}
