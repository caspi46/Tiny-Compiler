// use crate::frontend::operators::Operator::{
//     self, Add, Beq, Bge, Bgt, Ble, Blt, Bne, Bra, Cmp, Const, Div, End, Jsr, Mul, Phi, Ret, Sub,
// };
// use crate::frontend::operators::inst::Inst;
// use std::cell::RefCell;
// use std::rc::Rc;

// type Link = Option<Rc<RefCell<Inst>>>;

// pub struct Block {
//     block_name: String,
//     head: Link,
//     tail: Link,
// }

// // Block 0 will be the constant value container:
// // ex: 0 : 0, -1 : 1, -2 : 11

// impl Block {
//     fn new(cur_num: i32, name: String) -> Self {
//         let block_name = name + &cur_num.to_string();

//         Self {
//             block_name,
//             head: None,
//             tail: None,
//         }
//     }

//     fn push_head(&mut self, num_op: (i32, Operator)) {
//         let new_inst = Some(Rc::new(RefCell::new(Inst::new(
//             num_op,
//             None,
//             self.head.copy(),
//         ))));
//         self.head = new_inst;
//     }

//     fn push_tail(&mut self, num_op: (i32, Operator)) {
//         let new_inst = Some(Rc::new(RefCell::new(Inst::new(num_op, self.tail, None))));
//         self.tail = new_inst;
//     }

//     fn get_head(&mut self) -> &mut Link {
//         &mut self.head
//     }

//     fn get_tail(&mut self) -> &mut Link {
//         &mut self.tail
//     }
// }
