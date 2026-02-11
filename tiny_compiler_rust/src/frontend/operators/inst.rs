use crate::frontend::operators::operator::Operator;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Inst {
    data: (i32, Operator),
    prev: Option<Rc<RefCell<Inst>>>,
    next: Option<Rc<RefCell<Inst>>>,
}

impl Inst {
    pub fn new(
        data: (i32, Operator),
        prev: Option<Rc<RefCell<Inst>>>,
        next: Option<Rc<RefCell<Inst>>>,
    ) -> Self {
        Self { data, prev, next }
    }
}
