// use crate::frontend::operators::Operator::{
//     self, Add, Beq, Bge, Bgt, Ble, Blt, Bne, Bra, Cmp, Const, Div, End, Jsr, Mul, Phi, Ret, Sub,
// };
use crate::frontend::fsm::token::{Ident, Token};
use crate::frontend::operators::inst::Inst;
use std::collections::{HashMap, VecDeque};
// use std::cell::RefCell;
// use std::rc::Rc;

// type Link = Option<Rc<RefCell<Inst>>>;
#[derive(Debug)]
pub struct Block {
    block_name: String,
    insts: VecDeque<Inst>,
    table: HashMap<Ident, i32>,
}

impl<'a> Block {
    pub fn new(cur_num: i32, name: String) -> Self {
        let block_name = name + &cur_num.to_string();

        Self {
            block_name,
            insts: VecDeque::new(),
            table: HashMap::new(),
        }
    }

    pub fn get_head(&self) -> &Inst {
        &self.insts[0]
    }

    pub fn get_tail(&self) -> &Inst {
        &self.insts[self.insts.len()]
    }

    pub fn push_head(&mut self, new_head: Inst) {
        self.insts.push_front(new_head);
    }

    pub fn push_tail(&mut self, new_tail: Inst) {
        self.insts.push_back(new_tail);
    }

    pub fn fill_in_table(&mut self, ident: Ident, inst_num: i32) {
        self.table.insert(ident, inst_num);
    }
}
