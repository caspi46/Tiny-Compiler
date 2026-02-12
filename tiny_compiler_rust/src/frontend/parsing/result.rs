use crate::frontend::operators::operator::Operator;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Kind {
    Const,
    Var,
    Reg,
}
pub struct Result {
    kind: Kind,
    value: i32,
    address: i32,
    regn: i32,
}

impl Result {
    pub fn new(kind: Kind, value: i32, address: i32, regn: i32) -> Self {
        Self {
            kind,
            value,
            address,
            regn,
        }
    }

    pub fn get_kind(&self) -> Kind {
        self.kind
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_addr(&self) -> i32 {
        self.address
    }

    pub fn get_regn(&self) -> i32 {
        self.regn
    }

    pub fn set_kind(&mut self, new_kind: Kind) {
        self.kind = new_kind;
    }

    pub fn set_value(&mut self, new_value: i32) {
        self.value = new_value;
    }

    pub fn set_regn(&mut self, new_regn: i32) {
        self.value = new_regn;
    }
}
