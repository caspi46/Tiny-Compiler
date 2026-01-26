pub mod inst;
pub mod operator;

pub use inst::Inst;
pub use operator::{
    Add, Beq, Bge, Bgt, Ble, Blt, Bne, Bra, Cmp, Const, Div, End, Jsr, Mul, Operator, Phi, Ret, Sub,
};
