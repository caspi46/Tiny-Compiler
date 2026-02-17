// use crate::frontend::operators::Operator::{
//     self, Add, Beq, Bge, Bgt, Ble, Blt, Bne, Bra, Cmp, Const, Div, End, Jsr, Mul, Phi, Ret, Sub,
// };
use crate::frontend::fsm::token::{Ident, Token};
use crate::frontend::operators::{inst::Inst, operator::Operator};
use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, HashMap, VecDeque};
// use std::cell::RefCell;
// use std::rc::Rc;

// type Link = Option<Rc<RefCell<Inst>>>;
/// Block
/// Fields:
///     block_name : the name of the block
///     insts: the total insts in the block in order
///     table: the table to identify which variable is which inst#
///            The format is variable_name : inst#
///            
///     prevs: previous blocks
///     nexts: next blocks
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Block {
    block_name: String,
    block_num: usize,
    insts: VecDeque<Inst>,
    table: HashMap<String, i32>,
    nexts: Vec<usize>,
}

impl<'a> Block {
    pub fn new(cur_num: usize, name: String, table: HashMap<String, i32>) -> Self {
        let block_name = name + &cur_num.to_string();

        Self {
            block_name,
            block_num: cur_num,
            insts: VecDeque::new(),
            table: table,
            nexts: Vec::new(),
        }
    }

    // /// add_prev
    // /// add new prev block in block's prevs
    // pub fn add_prev(&mut self, block_num: usize) {
    //     self.prevs.borrow_mut().push(block_num);
    // }

    /// add_next
    /// add new next block in block's next
    pub fn add_next(&mut self, block_num: usize) {
        self.nexts.push(block_num);
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

    pub fn get_block_num(&self) -> usize {
        self.block_num
    }

    // pub fn fill_in_table(&mut self, ident: Ident, inst_num: i32) {
    //     self.table.insert(ident, inst_num);
    // }
}
