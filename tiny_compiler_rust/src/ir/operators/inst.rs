use crate::ir::operators::opeartor::{
    Add, Beq, Bge, Bgt, Ble, Blt, Bne, Bra, Cmp, Const, Div, End, Jsr, Mul, Operator, Phi, Ret, Sub,
};

pub struct Inst {
    inst_n: i32,
    operator: Operator,
    prev: Inst,
    next: Inst,
}

impl Inst {
    fn new(cur_inst: i32, op: Operator, prev: Inst, next: Inst) -> Self {
        Self {
            inst_n: cur_inst + 1,
            operator: op,
            prev,
            next,
        }
    }
}
