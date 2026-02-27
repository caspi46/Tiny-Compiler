use crate::frontend::operators::operator::Operator;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Inst {
    inst_num: i32,
    op: Operator,
}

impl<'a> Inst {
    pub fn new(
        inst_num: i32,
        op: Operator,
        // prev: Option<*const Inst>,
        // next: Option<*const Inst>,
    ) -> Self {
        Self { inst_num, op }
    }

    // pub fn set_next(&mut self, next_inst: &Inst) {
    //     let raw_ptr_nxt_inst = next_inst as *const Inst;
    //     self.next = Some(raw_ptr_nxt_inst);
    // }
    // pub fn set_prev(&mut self, next_inst: &Inst) {
    //     let raw_ptr_prev_inst = next_inst as *const Inst;
    //     self.prev = Some(raw_ptr_prev_inst);
    // }

    pub fn update_operator(&mut self, operator: Operator) {
        self.op = operator;
    }

    pub fn get_operator(self) -> Operator {
        self.op
    }
    pub fn get_inst_num(self) -> i32 {
        self.inst_num
    }
    pub fn get_data(self) -> (i32, Operator) {
        (self.inst_num, self.op)
    }

    pub fn check_op_data(&self, check: i32) -> bool {
        match self.op {
            Operator::Add(a, b)
            | Operator::Sub(a, b)
            | Operator::Mul(a, b)
            | Operator::Div(a, b)
            | Operator::Cmp(a, b)
            | Operator::Phi(a, b)
            | Operator::Bra(a, b)
            | Operator::Bne(a, b)
            | Operator::Ble(a, b)
            | Operator::Beq(a, b)
            | Operator::Bge(a, b)
            | Operator::Bgt(a, b)
            | Operator::Blt(a, b) => {
                if a == check || b == check {
                    return true;
                }
            }
            _ => return false,
        }
        false
    }

    pub fn get_op_data(self) -> (Option<i32>, Option<i32>) {
        match self.op {
            Operator::Add(a, b)
            | Operator::Sub(a, b)
            | Operator::Mul(a, b)
            | Operator::Div(a, b)
            | Operator::Cmp(a, b)
            | Operator::Phi(a, b)
            | Operator::Bra(a, b)
            | Operator::Bne(a, b)
            | Operator::Ble(a, b)
            | Operator::Beq(a, b)
            | Operator::Bge(a, b)
            | Operator::Bgt(a, b)
            | Operator::Blt(a, b) => (Some(a), Some(b)),
            _ => (None, None),
        }
    }

    pub fn update_op_insts(&mut self, updated_a: i32, updated_b: i32) {
        let updated_op = match self.op {
            Operator::Add(_, _) => Some(Operator::Add(updated_a, updated_b)),
            Operator::Sub(_, _) => Some(Operator::Sub(updated_a, updated_b)),
            Operator::Mul(_, _) => Some(Operator::Mul(updated_a, updated_b)),
            Operator::Div(_, _) => Some(Operator::Div(updated_a, updated_b)),
            Operator::Cmp(_, _) => Some(Operator::Cmp(updated_a, updated_b)),
            Operator::Phi(_, _) => Some(Operator::Phi(updated_a, updated_b)),
            Operator::Bra(_, _) => Some(Operator::Bra(updated_a, updated_b)),
            Operator::Bne(_, _) => Some(Operator::Bne(updated_a, updated_b)),
            Operator::Ble(_, _) => Some(Operator::Ble(updated_a, updated_b)),
            Operator::Beq(_, _) => Some(Operator::Beq(updated_a, updated_b)),
            Operator::Bge(_, _) => Some(Operator::Bge(updated_a, updated_b)),
            Operator::Bgt(_, _) => Some(Operator::Bgt(updated_a, updated_b)),
            Operator::Blt(_, _) => Some(Operator::Blt(updated_a, updated_b)),
            _ => None,
        };
        if let Some(op) = updated_op {
            self.op = op;
        }
    }

    pub fn update_op_inst1(&mut self, updated_a: i32) {
        let updated_op = match self.op {
            Operator::Add(_, b) => Some(Operator::Add(updated_a, b)),
            Operator::Sub(_, b) => Some(Operator::Sub(updated_a, b)),
            Operator::Mul(_, b) => Some(Operator::Mul(updated_a, b)),
            Operator::Div(_, b) => Some(Operator::Div(updated_a, b)),
            Operator::Cmp(_, b) => Some(Operator::Cmp(updated_a, b)),
            Operator::Phi(_, b) => Some(Operator::Phi(updated_a, b)),
            Operator::Bra(_, b) => Some(Operator::Bra(updated_a, b)),
            Operator::Bne(_, b) => Some(Operator::Bne(updated_a, b)),
            Operator::Ble(_, b) => Some(Operator::Ble(updated_a, b)),
            Operator::Beq(_, b) => Some(Operator::Beq(updated_a, b)),
            Operator::Bge(_, b) => Some(Operator::Bge(updated_a, b)),
            Operator::Bgt(_, b) => Some(Operator::Bgt(updated_a, b)),
            Operator::Blt(_, b) => Some(Operator::Blt(updated_a, b)),
            _ => None,
        };
        if let Some(op) = updated_op {
            self.op = op;
        }
    }

    pub fn update_op_inst2(&mut self, updated_b: i32) {
        let updated_op = match self.op {
            Operator::Add(a, _) => Some(Operator::Add(a, updated_b)),
            Operator::Sub(a, _) => Some(Operator::Sub(a, updated_b)),
            Operator::Mul(a, _) => Some(Operator::Mul(a, updated_b)),
            Operator::Div(a, _) => Some(Operator::Div(a, updated_b)),
            Operator::Cmp(a, _) => Some(Operator::Cmp(a, updated_b)),
            Operator::Phi(a, _) => Some(Operator::Phi(a, updated_b)),
            Operator::Bra(a, _) => Some(Operator::Bra(a, updated_b)),
            Operator::Bne(a, _) => Some(Operator::Bne(a, updated_b)),
            Operator::Ble(a, _) => Some(Operator::Ble(a, updated_b)),
            Operator::Beq(a, _) => Some(Operator::Beq(a, updated_b)),
            Operator::Bge(a, _) => Some(Operator::Bge(a, updated_b)),
            Operator::Bgt(a, _) => Some(Operator::Bgt(a, updated_b)),
            Operator::Blt(a, _) => Some(Operator::Blt(a, updated_b)),
            _ => None,
        };
        if let Some(op) = updated_op {
            self.op = op;
        }
    }

    pub fn update_op_inst(&mut self, updated: i32) {
        let updated_op = match self.op {
            Operator::SetPar1(_) => Some(Operator::SetPar1(updated)),
            Operator::SetPar2(_) => Some(Operator::SetPar2(updated)),
            Operator::SetPar3(_) => Some(Operator::SetPar3(updated)),
            _ => None,
        };

        if let Some(op) = updated_op {
            self.op = op;
        }
    }
}
