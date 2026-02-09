use crate::frontend::operators::operator::Operator;
pub struct Inst {
    data: (i32, Operator),
    prev: Option<Box<Inst>>,
    next: Option<Box<Inst>>,
}
