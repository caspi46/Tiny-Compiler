use crate::frontend::operators::operator::Operator;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Inst {
    inst_num: i32,
    op: Operator,
    x: Option<String>,
    y: Option<String>,
}

impl<'a> Inst {
    pub fn new(inst_num: i32, op: Operator, x: Option<String>, y: Option<String>) -> Self {
        Self { inst_num, op, x, y }
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

    pub fn get_op_one(self) -> Option<i32> {
        match self.op {
            Operator::SetPar1(a)
            | Operator::SetPar2(a)
            | Operator::SetPar3(a)
            | Operator::Ret(Some(a))
            | Operator::Write(a) => Some(a),
            _ => None,
        }
    }

    pub fn get_identifiers(&self) -> (Option<String>, Option<String>) {
        (self.x.clone(), self.y.clone())
    }
    pub fn get_id1(&self) -> Option<String> {
        self.x.clone()
    }

    pub fn get_id2(&self) -> Option<String> {
        self.y.clone()
    }

    pub fn get_op_two(self) -> (Option<i32>, Option<i32>) {
        match self.op {
            Operator::Add(a, b)
            | Operator::Sub(a, b)
            | Operator::Mul(a, b)
            | Operator::Div(a, b)
            | Operator::Cmp(a, b)
            | Operator::Phi(a, b) => (Some(a), Some(b)),
            _ => (None, None),
        }
    }
    pub fn is_op1(self, other_a: &i32) -> bool {
        match self.op {
            Operator::Add(a, _)
            | Operator::Sub(a, _)
            | Operator::Mul(a, _)
            | Operator::Div(a, _)
            | Operator::Cmp(a, _)
            | Operator::Phi(a, _)
            | Operator::SetPar1(a)
            | Operator::SetPar2(a)
            | Operator::SetPar3(a)
            | Operator::Ret(Some(a))
            | Operator::Write(a) => a == *other_a,
            _ => false,
        }
    }

    pub fn is_op2(self, other_b: &i32) -> bool {
        match self.op {
            Operator::Add(_, b)
            | Operator::Sub(_, b)
            | Operator::Mul(_, b)
            | Operator::Div(_, b)
            | Operator::Cmp(_, b)
            | Operator::Phi(_, b) => b == *other_b,
            _ => false,
        }
    }

    // pub fn update_op_one(&mut self, updated_a: i32) {
    //     let updated_op = match self.op {
    //         Operator::SetPar1(_) => Some(Operator::SetPar1(updated_a)),
    //         Operator::SetPar2(_) => Some(Operator::SetPar2(updated_a)),
    //         Operator::SetPar3(_) => Some(Operator::SetPar3(updated_a)),
    //         Operator::Ret(_) => Some(Operator::Ret(Some(updated_a))),
    //         _ => None,
    //     };
    //     if let Some(op) = updated_op {
    //         self.op = op;
    //     }
    // }

    pub fn update_op_two(&mut self, updated_a: i32, updated_b: i32) {
        let updated_op = match self.op {
            Operator::Add(_, _) => Some(Operator::Add(updated_a, updated_b)),
            Operator::Sub(_, _) => Some(Operator::Sub(updated_a, updated_b)),
            Operator::Mul(_, _) => Some(Operator::Mul(updated_a, updated_b)),
            Operator::Div(_, _) => Some(Operator::Div(updated_a, updated_b)),
            Operator::Cmp(_, _) => Some(Operator::Cmp(updated_a, updated_b)),
            Operator::Phi(_, _) => Some(Operator::Phi(updated_a, updated_b)),
            _ => None,
        };
        if let Some(op) = updated_op {
            println!("OP: {} & Original Op: {}", op, self.op);
            self.op = op;
            println!("UPDATE_OP_TWO: {}", self.op);
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
            Operator::SetPar1(_) => Some(Operator::SetPar1(updated_a)),
            Operator::SetPar2(_) => Some(Operator::SetPar2(updated_a)),
            Operator::SetPar3(_) => Some(Operator::SetPar3(updated_a)),
            Operator::Ret(_) => Some(Operator::Ret(Some(updated_a))),
            Operator::Write(_) => Some(Operator::Write(updated_a)),
            _ => None,
        };

        if let Some(op) = updated_op {
            println!("UPDATED OP: {}", op);
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
            _ => None,
        };
        if let Some(op) = updated_op {
            self.op = op;
        }
    }

    pub fn update_rel_op_inst2(&mut self, updated_b: String) {
        let updated_op = match self.op {
            Operator::Bne(a, _) => Some(Operator::Bne(a, Some(updated_b))),
            Operator::Ble(a, _) => Some(Operator::Ble(a, Some(updated_b))),
            Operator::Beq(a, _) => Some(Operator::Beq(a, Some(updated_b))),
            Operator::Bge(a, _) => Some(Operator::Bge(a, Some(updated_b))),
            Operator::Bgt(a, _) => Some(Operator::Bgt(a, Some(updated_b))),
            Operator::Blt(a, _) => Some(Operator::Blt(a, Some(updated_b))),
            _ => None,
        };
        if let Some(op) = updated_op {
            self.op = op;
        }
    }

    pub fn update_const(&mut self) {
        let updated_op = match self.op {
            Operator::Add(x, y) => Some(Operator::Add(x.abs(), y.abs())),
            Operator::Sub(x, y) => Some(Operator::Sub(x.abs(), y.abs())),
            Operator::Mul(x, y) => Some(Operator::Mul(x.abs(), y.abs())),
            Operator::Div(x, y) => Some(Operator::Div(x.abs(), y.abs())),
            Operator::Cmp(x, y) => Some(Operator::Cmp(x.abs(), y.abs())),
            Operator::SetPar1(x) => Some(Operator::SetPar1(x.abs())),
            Operator::SetPar2(x) => Some(Operator::SetPar2(x.abs())),
            Operator::SetPar3(x) => Some(Operator::SetPar3(x.abs())),
            Operator::Ret(Some(x)) => Some(Operator::Ret(Some(x.abs()))),
            Operator::Write(x) => Some(Operator::Write(x.abs())),
            _ => None,
        };
        if let Some(updated) = updated_op {
            self.op = updated;
        }
    }
}
