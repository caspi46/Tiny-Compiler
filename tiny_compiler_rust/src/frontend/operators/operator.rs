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
    Jsr(String), // block name
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
