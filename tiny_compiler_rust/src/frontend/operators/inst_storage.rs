use crate::frontend::operators::inst::Inst;
use crate::frontend::operators::operator::Operator;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
// For optimization
// each vector contains the each Inst type instructions
// Every time new inst is created, it is added into the storage.

// My thought process lol
// The add function should be called before inserting it into block or insts
// Since this is only for optimizing so far, this is not used for the actual Inst collection yet
// But, depending on the situation, it could be used as collection
// But, the collection is only for Add, Sub, Div, and Mul, so creating extra collection is unavoidable.
pub struct InstStorage {
    adds: HashMap<Operator, i32>,
    subs: HashMap<Operator, i32>,
    muls: HashMap<Operator, i32>,
    divs: HashMap<Operator, i32>,
}

impl InstStorage {
    pub fn new() -> Self {
        Self {
            adds: HashMap::new(),
            subs: HashMap::new(),
            muls: HashMap::new(),
            divs: HashMap::new(),
        }
    }

    fn add_adds(&mut self, new_add: Operator, inst_num: i32) -> i32 {
        if matches!(new_add, Operator::Sub(_, _)) {
            panic!("Error: Failed to add new add instruction in storage");
        }
        if !self.subs.contains_key(&new_add) {
            self.subs.insert(new_add, inst_num);
            return inst_num;
        }
        match self.subs.get(&new_add) {
            Some(&i) => i,
            _ => panic!("Error: Failed to add new add operator in storage"),
        }
    }

    fn add_subs(&mut self, new_sub: Operator, inst_num: i32) -> i32 {
        if matches!(new_sub, Operator::Sub(_, _)) {
            panic!("Error: Failed to add new sub instruction in storage");
        }
        if !self.subs.contains_key(&new_sub) {
            self.subs.insert(new_sub, inst_num);
            return inst_num;
        }
        match self.subs.get(&new_sub) {
            Some(&i) => i,
            _ => panic!("Error: Failed to add new sub operator in storage"),
        }
    }

    fn add_divs(&mut self, new_div: Operator, inst_num: i32) -> i32 {
        if matches!(new_div, Operator::Div(_, _)) {
            panic!("Error: Failed to add new div instruction in storage");
        }
        if !self.divs.contains_key(&new_div) {
            self.divs.insert(new_div, inst_num);
            return inst_num;
        }
        match self.subs.get(&new_div) {
            Some(&i) => i,
            _ => panic!("Error: Failed to add new div operator in storage"),
        }
    }

    fn add_muls(&mut self, new_mul: Operator, inst_num: i32) -> i32 {
        if matches!(new_mul, Operator::Mul(_, _)) {
            panic!("Error: Failed to add new mul instruction in storage");
        }
        if !self.muls.contains_key(&new_mul) {
            self.muls.insert(new_mul, inst_num);
            return inst_num;
        }
        match self.subs.get(&new_mul) {
            Some(&i) => i,
            _ => panic!("Error: Failed to add new mul operator in storage"),
        }
    }
}
