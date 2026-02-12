use crate::frontend::operators::operator::Operator;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Inst {
    data: (i32, Operator),
}

impl<'a> Inst {
    pub fn new(
        data: (i32, Operator),
        // prev: Option<*const Inst>,
        // next: Option<*const Inst>,
    ) -> Self {
        Self { data }
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
        self.data = (self.data.0, operator);
    }
}
