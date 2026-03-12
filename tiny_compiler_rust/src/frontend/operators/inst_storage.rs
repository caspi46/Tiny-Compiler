use crate::frontend::operators::operator::Operator;
use std::collections::HashMap;
// For optimization
// each vector contains the each Inst type instructions
// Every time new inst is created, it is added into the storage.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstStorage {
    adds: HashMap<Operator, i32>,
    subs: HashMap<Operator, i32>,
    muls: HashMap<Operator, i32>,
    divs: HashMap<Operator, i32>,
    // all_insts: HashMap<Operator, i32>,
}

impl InstStorage {
    pub fn new() -> Self {
        Self {
            adds: HashMap::new(),
            subs: HashMap::new(),
            muls: HashMap::new(),
            divs: HashMap::new(),
            // all_insts: HashMap::new(),
        }
    }

    pub fn adds(&mut self, new_op: Operator, inst_num: i32) {
        self.add_adds(new_op.clone(), inst_num);
        self.add_subs(new_op.clone(), inst_num);
        self.add_muls(new_op.clone(), inst_num);
        self.add_divs(new_op, inst_num);
    }

    pub fn add_adds(&mut self, new_add: Operator, inst_num: i32) -> Option<i32> {
        let new_abs_add = match new_add {
            Operator::Add(x, y) => Operator::Add(x.abs(), y.abs()),
            _ => {
                return None;
            }
        };

        match new_abs_add {
            Operator::Add(x, y) => match self.muls.get(&Operator::Add(y, x)) {
                Some(&i) => {
                    return Some(i);
                }
                _ => (),
            },
            _ => panic!("Error: Failed to check add operator in storage"),
        };
        if !self.adds.contains_key(&new_abs_add) {
            self.adds.insert(new_abs_add.clone(), inst_num);
            return Some(inst_num);
        }
        match self.adds.get(&new_abs_add) {
            Some(&i) => Some(i),
            _ => panic!("Error: Failed to add new mul operator in storage"),
        }
    }

    pub fn add_subs(&mut self, new_sub: Operator, inst_num: i32) -> Option<i32> {
        let new_abs_sub = match new_sub {
            Operator::Sub(x, y) => Operator::Sub(x.abs(), y.abs()),
            _ => {
                return None;
            }
        };
        if !self.subs.contains_key(&new_abs_sub) {
            self.subs.insert(new_abs_sub.clone(), inst_num);
            return Some(inst_num);
        }
        match self.subs.get(&new_abs_sub) {
            Some(&i) => Some(i),
            _ => panic!("Error: Failed to add new sub operator in storage"),
        }
    }

    pub fn add_divs(&mut self, new_div: Operator, inst_num: i32) -> Option<i32> {
        let new_abs_div = match new_div {
            Operator::Div(x, y) => Operator::Div(x.abs(), y.abs()),
            _ => {
                return None;
            }
        };
        if !self.divs.contains_key(&new_abs_div) {
            self.divs.insert(new_abs_div.clone(), inst_num);
            return Some(inst_num);
        }
        match self.divs.get(&new_abs_div) {
            Some(&i) => Some(i),
            _ => panic!("Error: Failed to add new div operator in storage"),
        }
    }

    pub fn add_muls(&mut self, new_mul: Operator, inst_num: i32) -> Option<i32> {
        let new_abs_mul = match new_mul {
            Operator::Mul(x, y) => Operator::Mul(x.abs(), y.abs()),
            _ => {
                return None;
            }
        };
        match new_abs_mul {
            Operator::Mul(x, y) => match self.muls.get(&Operator::Mul(y, x)) {
                Some(&i) => {
                    return Some(i);
                }
                _ => (),
            },
            _ => panic!("Error: Failed to check add operator in storage"),
        };
        if !self.muls.contains_key(&new_abs_mul) {
            self.muls.insert(new_abs_mul.clone(), inst_num);
            return Some(inst_num);
        }
        match self.muls.get(&new_abs_mul) {
            Some(&i) => Some(i),
            _ => panic!("Error: Failed to add new mul operator in storage"),
        }
    }

    pub fn get_inst_num(&self, op: &Operator) -> Option<i32> {
        match self.adds.get(op) {
            Some(i) => {
                return Some(*i);
            }
            _ => (),
        }
        match self.subs.get(op) {
            Some(i) => {
                return Some(*i);
            }
            _ => (),
        }
        match self.muls.get(op) {
            Some(i) => {
                return Some(*i);
            }
            _ => (),
        }
        match self.divs.get(op) {
            Some(i) => {
                return Some(*i);
            }
            _ => (),
        }
        None
    }
}
