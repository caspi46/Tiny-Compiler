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

    pub fn get_inst_num_index(&self, inst_num: i32) -> Option<usize> {
        for i in 0..self.insts.len() {
            if self.insts[i].clone().get_inst_num() == inst_num {
                return Some(i);
            }
        }
        return None;
    }

    pub fn get_tail_op(&self) -> Operator {
        self.insts[self.insts.len() - 1].clone().get_operator()
    }

    /// get_table
    /// get the table
    pub fn get_table(&self) -> HashMap<String, Option<i32>> {
        self.table.clone()
    }

    pub fn update_storage(&mut self) {
        for inst in self.insts.clone() {
            let inst_num = inst.clone().get_inst_num();
            let op = inst.get_operator();
            self.inst_storage.adds(op, inst_num);
        }
    }

    pub fn update_storage_with(&mut self, upon_storage: InstStorage) {
        self.inst_storage = upon_storage;
    }

    pub fn get_inst_storage(&self) -> InstStorage {
        self.inst_storage.clone()
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

    pub fn get_insts(&self) -> VecDeque<Inst> {
        self.insts.clone()
    }

    pub fn get_nexts(&self) -> &Vec<usize> {
        &self.nexts
    }

    pub fn get_doms(&self) -> &Vec<usize> {
        &self.doms
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

    pub fn get_inst(&self, op: &Operator) -> Option<i32> {
        for inst in &self.insts {
            let (n, o) = inst.clone().get_data();
            if o == *op {
                return Some(n);
            }
        }
        return None;
    }

    // var_to_pairs: var: (old, new)
    pub fn update_inst(&mut self, var_to_pairs: &HashMap<String, (i32, i32)>) {
        for i in 0..self.insts.len() {
            let mut inst = self.insts[i].clone();
            // case 1 to check two
            if let (Some(a), Some(b)) = &inst.clone().get_identifiers() {
                let a_key = match var_to_pairs.get(a) {
                    Some(a_pair) => {
                        if inst.clone().is_op1(&a_pair.0) {
                            Some(a_pair.1)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                let b_key = match var_to_pairs.get(b) {
                    Some(b_pair) => {
                        if inst.clone().is_op2(&b_pair.0) {
                            Some(b_pair.1)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let (Some(k1), Some(k2)) = (a_key, b_key) {
                    inst.update_op_two(k1, k2);
                } else if let Some(k1) = a_key {
                    inst.update_op_inst1(k1);
                } else if let Some(k2) = b_key {
                    inst.update_op_inst2(k2);
                }
            } else if let Some(a) = &inst.clone().get_id1() {
                let new_a = match var_to_pairs.get(a) {
                    Some(a_pair) => {
                        if inst.clone().is_op1(&a_pair.0) {
                            Some(a_pair.1)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some(a) = new_a {
                    inst.update_op_inst1(a);
                }
            } else if let Some(b) = &inst.clone().get_id2() {
                let new_b = match var_to_pairs.get(b) {
                    Some(b_pair) => {
                        if inst.clone().is_op2(&b_pair.0) {
                            Some(b_pair.1)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some(b) = new_b {
                    inst.update_op_inst2(b);
                }
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

    pub fn optimize_block(&mut self) -> HashMap<(i32, String), Option<i32>> {
        let mut delete_insts = HashMap::new();
        let mut table_collector = HashMap::new();
        let mut insts: VecDeque<Inst> = VecDeque::new();
        for inst in self.insts.clone() {
            let check = inst.clone();
            let check_i = check.clone().get_inst_num();
            let check_op = check.clone().get_operator();
            match check_op {
                Operator::Add(_, _)
                | Operator::Sub(_, _)
                | Operator::Mul(_, _)
                | Operator::Div(_, _) => {
                    if let Some(opt_i) = self.inst_storage.get_inst_num(&check_op) {
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
        let mut opt_inst = VecDeque::new();
        println!(
            "BLOCK NAME: {}\tDETEL INSTS : {:?}",
            self.block_name, delete_insts
        );
        for mut inst in insts.clone() {
            match inst.clone().get_op_two() {
                (Some(x), Some(y)) => {
                    if let Some(v) = delete_insts.get(&x) {
                        inst.update_op_inst1(*v);
                    }
                    if let Some(v) = delete_insts.get(&y) {
                        inst.update_op_inst2(*v);
                    }
                    println!(
                        "\n\nNEW INST AFTER UPDATE OP: {}",
                        inst.clone().get_operator()
                    );
                    opt_inst.push_back(inst);
                    continue;
                }
                _ => (),
            }

            match inst.clone().get_op_one() {
                Some(x) => {
                    println!(
                        "\n\n<<<DETECTED INST>>> : {} \t {}",
                        x,
                        inst.clone().get_operator()
                    );
                    if let Some(v) = delete_insts.get(&x) {
                        println!("<<ENTERED>> : {} \t {}", v, inst.clone().get_operator());
                        inst.update_op_inst1(*v);
                    }
                }
                _ => (),
            }
            opt_inst.push_back(inst);
        }
        self.insts = opt_inst;
        for (var, i) in self.table.clone() {
            if let Some(inst) = i
                && let Some(opt_i) = delete_insts.get(&inst)
            {
                println!("VAR: {}, INST_NUM: {}", var, opt_i);
                table_collector.insert((inst, var.clone()), Some(*opt_i));
                self.table.insert(var, Some(*opt_i));
            }
        }
        return table_collector;
    }

    // var_to_pairs: pairs => (old, new)
    pub fn optimize_fully(&mut self, var_to_pairs: &HashMap<(i32, String), Option<i32>>) {
        let mut opt_insts = VecDeque::new();
        for mut inst in self.insts.clone() {
            match inst.clone().get_op_two() {
                (Some(x), Some(y)) => {
                    if let Some(var_name) = inst.clone().get_id1()
                        && let Some(Some(opt_one)) = var_to_pairs.get(&(x, var_name))
                    {
                        inst.update_op_inst1(*opt_one);
                    }
                    if let Some(var_name) = inst.clone().get_id2()
                        && let Some(Some(opt_one)) = var_to_pairs.get(&(y, var_name))
                    {
                        inst.update_op_inst2(*opt_one);
                    }
                    opt_insts.push_back(inst.clone());
                    continue;
                }
                _ => (),
            }
            match inst.clone().get_op_one() {
                Some(x) => {
                    println!(
                        "\n\n<<<DETECTED INST>>> : {} \t {}",
                        x,
                        inst.clone().get_operator()
                    );
                    if let Some(var_name) = inst.clone().get_id1()
                        && let Some(Some(opt_one)) = var_to_pairs.get(&(x, var_name))
                    {
                        inst.update_op_inst1(*opt_one);
                    }
                }
                _ => (),
            }
            opt_insts.push_back(inst);
        }
        self.insts = opt_insts;
        for (var, i) in self.table.clone() {
            if let Some(inst) = i
                && let Some(Some(opt_i)) = var_to_pairs.get(&(inst, var.clone()))
            {
                println!("VAR: {}, INST_NUM: {}", var, opt_i);
                self.table.insert(var, Some(*opt_i));
            }
        }
    }
}
