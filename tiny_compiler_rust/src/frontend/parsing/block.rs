// use crate::frontend::operators::Operator::{
//     self, Add, Beq, Bge, Bgt, Ble, Blt, Bne, Bra, Cmp, Const, Div, End, Jsr, Mul, Phi, Ret, Sub,
// };
use crate::frontend::fsm::token::{Ident, Token};
use crate::frontend::operators::InstStorage;
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
    name: String,
    block_num: usize,
    insts: VecDeque<Inst>,
    table: HashMap<String, Option<i32>>,
    inst_storage: InstStorage,
    nexts: Vec<usize>,
    doms: Vec<usize>,
}

impl<'a> Block {
    pub fn new(cur_num: usize, name: String, table: HashMap<String, Option<i32>>) -> Self {
        let block_name = name.clone() + "_" + &cur_num.to_string();

        Self {
            block_name,
            name,
            block_num: cur_num,
            insts: VecDeque::new(),
            table: table,
            nexts: Vec::new(),
            inst_storage: InstStorage::new(),
            doms: Vec::new(),
        }
    }

    pub fn get_block_name(&self) -> String {
        self.block_name.clone()
    }

    pub fn get_func_name(&self) -> String {
        self.name.clone()
    }

    /// add_next
    /// add new next block in block's next
    pub fn add_next(&mut self, block_num: usize) {
        self.nexts.push(block_num);
    }

    pub fn add_dom(&mut self, block_num: usize) {
        self.doms.push(block_num);
    }

    /// get_head
    /// get the first inst of the block
    pub fn get_head(&self) -> &Inst {
        &self.insts[0]
    }

    pub fn get_head_num(&self) -> Option<i32> {
        if self.insts.len() == 0 {
            return None;
        }
        Some(self.insts[0].clone().get_inst_num())
    }

    pub fn get_inst_num_index(&self, inst_num: i32) -> Option<usize> {
        for i in 0..self.insts.len() {
            if self.insts[i].clone().get_inst_num() == inst_num {
                return Some(i);
            }
        }
        return None;
    }

    /// get_tail
    /// get the last inst of the block
    pub fn get_tail(&self) -> &Inst {
        &self.insts[self.insts.len()]
    }

    pub fn get_tail_op(&self) -> Operator {
        self.insts[self.insts.len() - 1].clone().get_operator()
    }

    /// get_table
    /// get the table
    pub fn get_table(&self) -> HashMap<String, Option<i32>> {
        self.table.clone()
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

    pub fn get_insts(&self) -> VecDeque<Inst> {
        self.insts.clone()
    }

    pub fn get_nexts(&self) -> &Vec<usize> {
        &self.nexts
    }

    pub fn get_doms(&self) -> &Vec<usize> {
        &self.doms
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
        self.table.insert(ident, Some(inst_num));
    }

    pub fn update_table_with_insts(&mut self, var_to_phi: &HashMap<String, i32>) {
        for (var, phi) in var_to_phi {
            self.table.insert(var.clone(), Some(*phi));
        }
    }

    /// check_table
    /// check if the ident exists in the table
    /// if so, return the inst#
    pub fn check_table(&self, ident: &String) -> Option<i32> {
        match self.table.get(ident) {
            Some(&val) => val,
            _ => None,
        }
    }

    pub fn compare_table(&self, other: &RefCell<Block>) -> HashMap<String, (i32, i32)> {
        let mut updated_vars = HashMap::new();
        for (var, inst_n) in self.table.clone() {
            match (other.borrow().check_table(&var), inst_n) {
                (Some(now_i), Some(other_i)) => {
                    println!(
                        "Variable: {}, my inst: {}, other inst: {}",
                        var, now_i, other_i
                    );
                    if now_i != other_i {
                        updated_vars.insert(var, (now_i, other_i));
                    }
                }
                (Some(i), None) => {
                    updated_vars.insert(var, (i, 0));
                }
                (None, Some(i)) => {
                    updated_vars.insert(var, (0, i));
                }
                _ => (),
            }
        }
        updated_vars
    }

    pub fn get_block_num(&self) -> usize {
        self.block_num
    }

    pub fn get_inst(&self, op: &Operator) -> Option<i32> {
        for inst in &self.insts {
            let (n, o) = inst.clone().get_data();
            if o == *op {
                return Some(n);
            }
        }
        return None;
    }

    pub fn set_pair_for_phi(&mut self, phis: &Vec<(String, i32)>) -> HashMap<i32, i32> {
        let mut pair = HashMap::new();
        for (v, n) in phis {
            if let Some(k) = self.table.get(v) {
                if let Some(original) = k {
                    pair.insert(*original, *n);
                    self.table.insert(v.clone(), Some(*n));
                }
            }
        }
        pair
    }

    pub fn update_inst(&mut self, ori_to_new: &HashMap<i32, i32>) {
        println!("Current Table for update Inst: {}", self.block_name);
        println!("Current ori_to_new: {:?}", ori_to_new);
        for i in 0..self.insts.len() {
            println!("Current i for update Inst: {}", i);
            let mut inst = self.insts[i].clone();
            // case 1 to check two
            if let (Some(a), Some(b)) = &inst.clone().get_op_two() {
                let a_key: i32 = match ori_to_new.get(&a) {
                    Some(new_a) => *new_a,
                    _ => *a,
                };
                println!("a_key: {}", a_key);
                let b_key = match ori_to_new.get(&b) {
                    Some(new_b) => *new_b,
                    _ => *b,
                };
                inst.update_op_two(a_key, b_key);
                println!("Update by Phi: {}", inst.clone().get_operator());
            } else if let Some(a) = &inst.clone().get_op_one() {
                let new_a = match ori_to_new.get(&a) {
                    Some(new_a) => *new_a,
                    _ => *a,
                };
                inst.update_op_one(new_a);
            }

            self.insts[i] = inst;
        }
    }

    pub fn fill_in_none(&mut self, loc_rhs: (i32, String)) -> bool {
        if let Some(i) = self.get_inst_num_index(loc_rhs.0) {
            self.insts[i].update_rel_op_inst2(loc_rhs.1);
            return true;
        }
        return false;
    }

    pub fn set_no_const_sign(&mut self) {
        let mut insts: VecDeque<Inst> = VecDeque::new();
        for mut inst in self.insts.clone() {
            inst.update_const();
            insts.push_back(inst);
        }
        self.insts = insts.clone();
    }

    pub fn optimize_block(&mut self, inst_storage: &InstStorage) {
        // variable table update
        // instruction update
        // remove instruction
        println!(
            "\n\nInsts Before Opt: {:?} for this Block: {}\n\n\n",
            self.insts, self.block_name
        );

        let mut delete_insts = HashMap::new();

        let mut insts: VecDeque<Inst> = VecDeque::new();
        for inst in self.insts.clone() {
            let check = inst.clone();
            let check_i = check.clone().get_inst_num();
            let check_op = check.clone().get_operator();
            match check_op {
                Operator::Add(_, _)
                | Operator::Sub(_, _)
                | Operator::Mul(_, _)
                | Operator::Div(_, _)
                | Operator::Phi(_, _) => {
                    if let Some(opt_i) = inst_storage.get_inst_num(&check_op) {
                        if opt_i != check_i {
                            delete_insts.insert(check_i, opt_i);
                            continue;
                        }
                    }
                    insts.push_back(check);
                }
                _ => {
                    insts.push_back(check);
                }
            }
        }
        for mut inst in insts.clone() {
            match inst.clone().get_op_two() {
                (Some(x), Some(y)) => {
                    if let Some(v) = delete_insts.get(&x) {
                        inst.update_op_inst1(*v);
                    }
                    if let Some(v) = delete_insts.get(&y) {
                        inst.update_op_inst2(*v);
                    }
                    continue;
                }
                _ => (),
            }

            match inst.clone().get_op_one() {
                Some(x) => {
                    if let Some(v) = delete_insts.get(&x) {
                        inst.update_op_inst1(*v);
                    }
                }
                _ => (),
            }
        }
        self.insts = insts;
        for (var, i) in self.table.clone() {
            if let Some(inst) = i
                && let Some(opt_i) = delete_insts.get(&inst)
            {
                self.table.insert(var, Some(*opt_i));
            }
        }
    }

    // pub fn fill_in_table(&mut self, ident: Ident, inst_num: i32) {
    //     self.table.insert(ident, inst_num);
    // }
}
