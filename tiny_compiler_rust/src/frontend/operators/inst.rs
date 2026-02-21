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
}
