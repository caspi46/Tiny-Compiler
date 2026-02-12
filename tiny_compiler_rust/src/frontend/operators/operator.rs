// Operators
#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    //Constant
    Const(String),
    // Op
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    // Compare
    Cmp(String, String),
    // Phi function
    Phi(String, String),
    // End
    End,
    // RelOp
    Bra(String, String),
    Bne(String, String),
    Ble(String, String),
    Bge(String, String),
    Bgt(String, String),
    Beq(String, String),
    Blt(String, String),

    // Read & Write
    Read,
    Write(String),
    WriteNL,
    // Operator for user-defined functions
    Jsr(String),
    Ret(String),
    // For now, only three parameters
    GetPar1,
    GetPar2,
    GetPar3,
    SetPar1(String),
    SetPar2(String),
    SetPar3(String),
}
