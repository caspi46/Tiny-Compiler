// use crate::frontend::operators::Operator::{
//     self, Add, Beq, Bge, Bgt, Ble, Blt, Bne, Bra, Cmp, Const, Div, End, Jsr, Mul, Phi, Ret, Sub,
// };
use crate::frontend::fsm::token::{Ident, Token};
use crate::frontend::operators::{inst::Inst, operator::Operator};
use std::collections::{BTreeMap, HashMap, VecDeque};
// use std::cell::RefCell;
// use std::rc::Rc;

// type Link = Option<Rc<RefCell<Inst>>>;
#[derive(Debug, Eq, PartialEq)]
pub struct Block {
    block_name: String,
    insts: VecDeque<Inst>,
    table: HashMap<String, i32>,
    prevs: Vec<Block>,
    nexts: Vec<Block>,
}

impl<'a> Block {
    pub fn new(cur_num: i32, name: String) -> Self {
        let block_name = name + &cur_num.to_string();

        Self {
            block_name,
            insts: VecDeque::new(),
            table: HashMap::new(),
            prevs: Vec::new(),
            nexts: Vec::new(),
        }
    }

    /// add_prev
    /// add new prev block in block's prevs
    pub fn add_prev(&mut self, prev_block: Block) {
        self.prevs.push(prev_block);
    }

    /// add_next
    /// add new next block in block's next
    pub fn add_next(&mut self, next_block: Block) {
        self.nexts.push(next_block);
    }

    /// get_head
    /// get the first inst of the block
    pub fn get_head(&self) -> &Inst {
        &self.insts[0]
    }

    /// get_tail
    /// get the last inst of the block
    pub fn get_tail(&self) -> &Inst {
        &self.insts[self.insts.len()]
    }

    /// push_head
    /// add inst to the front of the block
    pub fn push_head(&mut self, new_head: Inst) {
        self.insts.push_front(new_head);
    }

    /// push_tail
    /// add inst to the end of the block
    pub fn push_tail(&mut self, new_tail: Inst) {
        self.insts.push_back(new_tail);
    }

    /// get_inst_num
    /// get the total number of inst in the block
    pub fn get_inst_num(&self) -> i32 {
        self.insts.len() as i32
    }

    /// contains_inst
    /// check if the inst is in the block
    pub fn contains_inst(&self, inst: Inst) -> bool {
        for i in self.insts.clone() {
            if i == inst {
                return true;
            }
        }
        false
    }

    /// update_table
    /// update the ident's information in the table
    pub fn update_table(&mut self, ident: String, inst_num: i32) {
        self.table.insert(ident, inst_num);
    }

    /// check_table
    /// check if the ident exists in the table
    /// if so, return the inst#
    pub fn check_table(&self, ident: &String) -> Option<&i32> {
        self.table.get(ident)
    }

    // pub fn fill_in_table(&mut self, ident: Ident, inst_num: i32) {
    //     self.table.insert(ident, inst_num);
    // }
}
