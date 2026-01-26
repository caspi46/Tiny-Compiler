// Operators 
// Currently, this is just skeleton structures

pub enum Operator {
    Const(Const), 
    Add(Add),
    Sub(Sub), 
    Mul(Mul),
    Div(Div), 
}

pub struct Const {
    value: String // just for now 
}

pub struct Add {
    x : i32, 
    y : i32, 
}

pub struct Sub {
    x : i32, 
    y : i32, 
}

pub struct Mul {
    x : i32, 
    y : i32,
}

pub struct Div {
    x : i32, 
    y : i32,
}

pub struct Cmp {
    x : i32, 
    y : i32,
}

// Phi Function
pub struct Phi {
    x1 : i32, 
    x2 : i32, 
}

pub struct End {

}

pub struct Bra {
    y : String,
}

pub struct Bne {
    x : String, 
    y : String, 
}

pub struct Beq {
    x : String,
    y : String,
}

pub struct Blt {
    x : String, 
    y : String,
}

pub struct Bge {
    x : String, 
    y : String,
}

pub struct Bgt {
    x : String, 
    y : String,
}

pub struct Jsr {
    x : String,
}

pub struct Ret {
    x : String,
}



