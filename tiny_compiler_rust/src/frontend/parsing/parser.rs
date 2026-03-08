use crate::frontend::fsm::{
    token::{
        Ident, Ident::UserDefined, Op::ADD, Op::DIV, Op::MUL, Op::SUB, RelOp::EQ, RelOp::GE,
        RelOp::GT, RelOp::LE, RelOp::LT, RelOp::NE, Symbol, Token,
    },
    tokenizer::Tokenizer,
};
use crate::frontend::operators::{Inst, InstStorage, Operator};
use crate::frontend::parsing::block::Block;
// use crate::frontend::parsing::result::{Kind, Result};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

#[derive()]
pub struct Parser {
    tokens: Vec<Token>,
    blocks: BTreeMap<usize, RefCell<Block>>,
    vars: HashMap<String, Option<i32>>,
    insts: HashMap<Operator, i32>,
    funcs: HashMap<String, usize>,
    void_funcs: Vec<String>,
    cur_token_index: usize,
    // cur_block: &'a RefCell<Block>,
    cur_block_num: usize,
    // busy: Vec<i32>,
    block0_num: usize,
    total_inst: i32,
    total_block: usize,
    inst_storage: InstStorage,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.generate_token();
        let tokens = tokenizer.get_tokens();
        // if tokens.len() < 0 {
        //     panic!("No User Input/Toeken");
        // }
        // let mut busy = vec![0; 32];
        // busy[0] = 1; // register 0
        let block0 = RefCell::new(Block::new(0, "block".to_string(), HashMap::new()));
        let zero_inst = Inst::new(0, Operator::Const(0));
        block0.borrow_mut().push_head(zero_inst);
        let mut blocks = BTreeMap::new();
        blocks.insert(0, block0);

        Self {
            tokens,
            blocks,
            vars: HashMap::new(),
            insts: HashMap::new(),
            funcs: HashMap::new(),
            void_funcs: Vec::new(),
            cur_token_index: 0,
            cur_block_num: 0,
            total_inst: 0,
            total_block: 0,
            inst_storage: InstStorage::new(),
            block0_num: 0, // block0 will be added to front at the end
        }
    }
    fn current(&self) -> &Token {
        if self.cur_token_index == self.tokens.len() {
            panic!("Error: Out of range");
        }
        &self.tokens[self.cur_token_index]
    }

    fn move_token(&mut self) {
        self.cur_token_index += 1;
    }

    fn factor(&mut self) -> i32 {
        // return: i32
        // ident | number | "(" expression ")" | funcCall
        self.move_token();
        println!("Factor's current Token: {}", self.current());
        let factor_token = self.current().clone();
        match factor_token {
            Token::Symbol(Symbol::OpenParen) => {
                let inst_num = self.expression();
                if self.current() != &Token::Symbol(Symbol::CloseParen) {
                    panic!("Error, missing closed parentheses: {}", self.current());
                }
                self.move_token();
                return inst_num;
            }
            Token::Number(num) => {
                // add num value in block 0

                let op = Operator::Const(num);
                self.move_token();
                if let Some(i_num) = self.get_bb0(&op) {
                    return i_num;
                }

                let inst_num = self.add_const_to_bb0(op);
                println!("Num's Token: {}", inst_num);
                return inst_num;
            }
            Token::Ident(UserDefined(var)) => {
                // user-defined variable
                //
                // if the variable is already initialized to other var in the table
                self.move_token();
                let cur_block = if let Some(b) = self.blocks.get(&self.total_block) {
                    b
                } else {
                    panic!("Error: No Block Found at {}", self.total_block)
                };

                if let Some(inst_num) = cur_block.borrow().check_table(&var) {
                    inst_num
                } else {
                    println!("Warning: Uninitialized variable");
                    0
                }
            }
            Token::Call => {
                // skip for now (maybe after I solve the ident and num)
                println!("Factor is Call");
                self.move_token();
                match self.current() {
                    // TODO: HAVE TO ADD THE non-void user-defined function
                    Token::Ident(Ident::UserDefined(func)) => {
                        if self.void_funcs.contains(func) {
                            panic!("Error: Void function detected in assignment");
                        }
                        let inst_num = self.func_call();
                        return inst_num;
                    }
                    Token::Ident(_) => {
                        let inst_num = self.func_call();
                        return inst_num; // Should call funcCall 
                    }

                    _ => panic!("Error: Invalid Function Call"),
                }
            }
            _ => {
                panic!("Error: Invalid factor format: {}", self.current());
            }
        }
    }
    fn term(&mut self) -> i32 {
        // factor { ("*" | "/") factor }
        let mut x: i32 = self.factor();
        // self.vars.insert(x, self.total_inst);
        println!("Current token after Factor: {}", self.current());
        if self.current() != &Token::Op(MUL) && self.current() != &Token::Op(DIV) {
            return x;
        }
        let mut return_val = x;
        let mut calc;
        while self.current() == &Token::Op(MUL) || self.current() == &Token::Op(DIV) {
            println!("Current Token at MUL&DIV loop: {}", self.current());
            calc = self.current().clone();
            let y = self.factor();
            let op = match calc {
                Token::Op(MUL) => Operator::Mul(x, y),
                Token::Op(DIV) => Operator::Div(x, y),
                _ => panic!("Error, Invalid Div or Mul"),
            };
            let expected_num = self.total_inst + 1;
            if let Some(mul) = self.inst_storage.add_muls(op.clone(), expected_num) {
                return_val = if mul == expected_num {
                    let inst_num = self.add_inst_to_tail(op.clone());
                    self.insts.insert(op.clone(), inst_num);
                    inst_num
                } else {
                    mul
                };
            }
            if let Some(div) = self.inst_storage.add_divs(op.clone(), expected_num) {
                return_val = if div == expected_num {
                    let inst_num = self.add_inst_to_tail(op.clone());
                    self.insts.insert(op, inst_num);
                    inst_num
                } else {
                    div
                };
            }
            x = return_val;
            println!("end of the MUL&DIV loop : {}", x);
        }
        // should return i32
        // i32 is the inst# that takes the calculation result
        return_val
    }
    fn expression(&mut self) -> i32 {
        // term { ("+" | "-") term }
        let mut x = self.term(); // return the 
        // no add or sub calculation
        // gets the same inst# as term
        // self.move_token();
        if self.current() != &Token::Op(ADD) && self.current() != &Token::Op(SUB) {
            // self.move_token();
            return x;
        }
        let mut calc;
        let mut return_val = x;
        while self.current() == &Token::Op(ADD) || self.current() == &Token::Op(SUB) {
            println!("Current Token at ADD&SUB loop: {}", self.current());
            calc = self.current().clone();
            let y = self.term();
            let op = match calc {
                Token::Op(ADD) => Operator::Add(x, y),
                Token::Op(SUB) => Operator::Sub(x, y),
                _ => panic!("Error: Invalid ADD or SUB format"),
            };
            let expected_num = self.total_inst + 1;
            if let Some(add) = self.inst_storage.add_adds(op.clone(), expected_num) {
                return_val = if add == expected_num {
                    let inst_num = self.add_inst_to_tail(op.clone());
                    self.insts.insert(op.clone(), inst_num);
                    println!("Inst Num for add optimization: {}", inst_num);
                    inst_num
                } else {
                    println!("Inst Num for no optimization add: {}", add);
                    add
                };
            } else if let Some(sub) = self.inst_storage.add_subs(op.clone(), expected_num) {
                return_val = if sub == expected_num {
                    let inst_num = self.add_inst_to_tail(op.clone());
                    self.insts.insert(op, inst_num);
                    println!("Inst Num for sub optimization: {}", inst_num);

                    inst_num
                } else {
                    println!("Inst Num for no optimization sub: {}", sub);
                    sub
                };
                x = return_val;
            }
        }
        return_val
        // return the i32 value for that takes the calculation for example: add (1) (2) -> (1)
    }
    fn relation(&mut self) -> i32 {
        // should return
        // // expression relOp expression]
        // self.move_token();
        println!("Current Token in Relation: {}", self.current());
        let lhs = self.expression(); // first in RelOp (v1, v2)
        // while matches!(self.current(), &Token::RelOp(_)) {
        print!("Current Token after LHS in relation: {}", self.current());
        // }
        let rel_op = self.current().clone();
        let rhs = self.expression();
        let cmp = self.add_inst_to_tail(Operator::Cmp(lhs, rhs));
        let rel_inst = match rel_op {
            Token::RelOp(EQ) => self.add_inst_to_tail(Operator::Bne(cmp, None)),
            Token::RelOp(NE) => self.add_inst_to_tail(Operator::Beq(cmp, None)),
            Token::RelOp(GT) => self.add_inst_to_tail(Operator::Ble(cmp, None)),
            Token::RelOp(LT) => self.add_inst_to_tail(Operator::Bge(cmp, None)),
            Token::RelOp(GE) => self.add_inst_to_tail(Operator::Blt(cmp, None)),
            Token::RelOp(LE) => self.add_inst_to_tail(Operator::Bgt(cmp, None)),
            _ => {
                panic!("Error, missing relOp: ==, !=, >, <, >=, <=");
            }
        };
        rel_inst
    }
    fn assignment(&mut self) -> i32 {
        // "let" ident "<-"  expression
        self.move_token();

        let var = match self.current() {
            Token::Ident(UserDefined(name)) => name.to_string(),
            _ => panic!("Error: Invalid Assignment Forat - Missing Variable"),
        };
        self.move_token();
        if self.current() != &Token::Symbol(Symbol::Init) {
            panic!("Error: Invalid Assignment Format, Missing \"<-\"");
        }
        println!("Current token before expression: {}", self.current());
        let rhs = self.expression(); // TODO: Return value (Identify the value)

        let updated_rhs = if rhs > 0 { rhs } else { rhs * (-1) };
        self.vars.insert(var.clone(), Some(updated_rhs));
        let cur_block = if let Some(b) = self.blocks.get(&self.cur_block_num) {
            b
        } else {
            panic!("Error: No Block Found at {}", self.cur_block_num)
        };
        cur_block.borrow_mut().update_table(var, updated_rhs);
        rhs
    }

    fn func_call(&mut self) -> i32 {
        // "call" ident [ "(" [expression {"," expression}] ")"]
        // self.move_token();
        match self.current() {
            Token::Ident(Ident::InputNum) => {
                // no parameter
                let op = Operator::Read;
                self.move_token();
                if &Token::Symbol(Symbol::OpenParen) != self.current() {
                    panic!("Error: no open parenthesis for function call");
                }
                self.move_token();
                if &Token::Symbol(Symbol::CloseParen) != self.current() {
                    panic!("Error: no closed parenthesis");
                }

                let _ = self.add_inst_to_tail(op.clone());
                self.move_token();
                self.total_inst
            }
            Token::Ident(Ident::OutputNewLine) => {
                // no parameter
                let op = Operator::WriteNL;
                self.move_token();
                if &Token::Symbol(Symbol::OpenParen) != self.current() {
                    panic!("Error: no open parenthesis for function call");
                }
                self.move_token();
                if &Token::Symbol(Symbol::CloseParen) != self.current() {
                    panic!("Error: no closed parenthesis");
                }

                let inst_num = self.add_inst_to_tail(op.clone());
                self.insts.insert(op, inst_num);
                self.move_token();
                self.total_inst
            }
            Token::Ident(Ident::OutputNum) => {
                self.move_token();
                if &Token::Symbol(Symbol::OpenParen) == self.current() {
                    let arg = self.expression();
                    if &Token::Symbol(Symbol::CloseParen) != self.current() {
                        panic!("Error: no closed parenthesis: {}", self.current());
                    }

                    let write_op = Operator::Write(arg);
                    let inst_num = self.add_inst_to_tail(write_op.clone());
                    self.insts.insert(write_op, inst_num);
                    self.move_token();
                    self.total_inst
                } else {
                    panic!("Errorr: no opened parenthesis");
                }
            }

            Token::Ident(UserDefined(func)) => {
                let params = if let Some(&p) = self.funcs.get(func) {
                    p
                } else {
                    panic!(
                        "Error: Invalid Function Format: {}\nFunc:{}\nFuncs:{:?}",
                        self.current(),
                        func,
                        self.funcs,
                    )
                };

                let func_call = Operator::Jsr(func.to_string());
                self.move_token();
                if &Token::Symbol(Symbol::OpenParen) == self.current() {
                    if params >= 1 {
                        // self.move_token();
                        let param1 = self.expression();
                        let set_param1 = Operator::SetPar1(param1);
                        let inst_num = self.add_inst_to_tail(set_param1.clone());
                        self.insts.insert(set_param1, inst_num);
                    }

                    println!("After Param1 Check: {}", self.current());

                    if self.current() == &Token::Symbol(Symbol::Comma) {
                        println!("Entered for 2nd params: {}", self.current());
                        if params >= 2 {
                            // self.move_token();
                            let param2 = self.expression();
                            let set_param2 = Operator::SetPar2(param2);
                            let inst_num = self.add_inst_to_tail(set_param2.clone());
                            self.insts.insert(set_param2, inst_num);
                        }
                    }
                    println!("Finished 2nd params: {}", self.current());
                    if self.current() == &Token::Symbol(Symbol::Comma) {
                        if params == 3 {
                            let param3 = self.expression();
                            let set_param3 = Operator::SetPar3(param3);
                            let inst_num = self.add_inst_to_tail(set_param3.clone());
                            self.insts.insert(set_param3, inst_num);
                        }
                    }
                    if params > 3 {
                        panic!("Error: More than 3 Parameter for Function");
                    }
                    if &Token::Symbol(Symbol::CloseParen) != self.current() {
                        panic!(
                            "Error: Missing Closed Parenthesis for function call: {}",
                            self.current()
                        );
                    }
                    let inst_num = self.add_inst_to_tail(func_call.clone());
                    self.insts.insert(func_call, inst_num);
                    println!("End of Func Call: {}", self.current());
                    self.move_token();
                    self.total_inst
                } else {
                    panic!("Error: Missing Opened Paranthesis");
                }
            }
            _ => panic!("Error: Invalid funcCall format"),
        }
    }
    fn if_statement(&mut self) {
        // "if" relation "then" statSequence ["else" statSequence] "fi"
        // new block: if, then, else, fi

        // new blocks! IF, THEN, FI
        // if block
        // self.total_block += 1;
        let before_table = self.vars.clone();
        let if_key = self.cur_block_num;

        // then block
        self.total_block += 1;
        let then_block = RefCell::new(Block::new(
            self.total_block,
            "then".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block, then_block);
        let then_key = self.total_block;

        // fi block
        self.total_block += 1;
        let fi_block = RefCell::new(Block::new(
            self.total_block,
            "fi".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block, fi_block);
        let fi_key = self.total_block;
        self.set_dom(if_key, fi_key);
        // (if_block, then_block) = self.connect(if_block, then_block);

        // add insts in if block
        // self.switch_block(if_key);
        // self.move_token();
        let cond = self.relation(); // if_block saved them already
        println!("Current Token after IF: {}", self.current());

        if self.current() != &Token::Then {
            panic!("Error: Invalid If Statement Format, Missing \"then\"");
        }
        // (then_block, fi_block) = self.connect(then_block, fi_block);
        self.switch_block(then_key);
        self.stat_sequence(); // add all the instructions in the then to the then block
        self.connect(if_key, then_key);
        self.set_dom(if_key, then_key);

        // generate phis for then
        let mut phis;
        if self.cur_block_num != then_key {
            self.connect(self.cur_block_num, fi_key);
            phis = self.generate_phi(self.cur_block_num, then_key);
        } else {
            self.connect(then_key, fi_key);
            phis = self.generate_phi(if_key, then_key);
        }
        // create phi functioins here for fi block (LEFT in Phi(Left, Right))
        println!("Current Token after THEN: {}", self.current());

        if self.current() == &Token::Else {
            // since else is optional
            self.total_block += 1;
            println!("Current Table at Else : {:?}", before_table.clone());
            let else_block = RefCell::new(Block::new(
                self.total_block,
                "else".to_string(),
                before_table,
            ));
            // (if_block, else_block) = self.connect(if_block, else_block);
            self.blocks.insert(self.total_block, else_block);
            let else_key = self.total_block;
            self.connect(if_key, else_key);
            self.set_dom(if_key, else_key);
            self.switch_block(else_key);
            self.stat_sequence();
            // self.add_inst_to_tail(Operator::Bra(jump_inst));
            println!("Current Total Inst before phi: {}", self.total_inst);
            // Update condition instruction

            if self.cur_block_num != else_key {
                self.connect(self.cur_block_num, fi_key);
                phis = self.generate_phi(self.cur_block_num, else_key);
            } else {
                self.connect(else_key, fi_key);
                phis = self.generate_phi(then_key, else_key);
            }
            self.switch_block(else_key);
            let mut loc_rhs = (cond, "".to_string());
            if let Some(else_head) = self.get_current_block_name() {
                loc_rhs.1 = else_head;
            }
            self.update_rel_op(if_key, loc_rhs);
        } else {
            // (if_block, fi_block) = self.connect(if_block, fi_block);
            // let branch;
            self.switch_block(fi_key);
            let loc_rhs = if let Some(block_name) = self.get_current_block_name() {
                (cond, block_name)
            } else {
                (cond, "".to_string())
            };
            self.update_rel_op(if_key, loc_rhs);
            self.connect(if_key, fi_key);
        }

        if self.current() != &Token::Fi {
            panic!("Error: Invalid If Statement Format, Missing \"fi\"");
        }
        println!("Current Total Inst:{}", self.total_inst);
        self.switch_block(fi_key);
        for (var, phi) in phis {
            let phi_op = Operator::Phi(phi.0, phi.1);
            let inst_num = self.add_inst_to_tail(phi_op);
            self.update_table(var, inst_num);
        }
        self.move_token();
        // add phi functions if exists
    }
    fn while_statement(&mut self) {
        // "while" relaton "do" StatSequence "od"
        // new block: while, then
        let before_table = self.vars.clone();
        self.total_block += 1;
        let while_key = self.total_block;
        let while_block = RefCell::new(Block::new(
            self.total_block,
            "while".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block, while_block);

        self.total_block += 1;
        let do_key = self.total_block;
        let do_block = RefCell::new(Block::new(
            self.total_block,
            "do".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block, do_block);

        // Edges
        self.connect(self.cur_block_num, while_key);
        self.connect(while_key, do_key);

        // doms
        self.set_dom(self.cur_block_num, while_key);
        self.set_dom(while_key, do_key);

        self.switch_block(while_key);
        let cond = self.relation();
        if self.current() != &Token::Do {
            panic!("Error: Invalid While Statement Format, Missing \"do\"");
        }
        self.switch_block(do_key);
        self.stat_sequence();
        let phis;
        if self.cur_block_num == do_key {
            self.connect(do_key, while_key);
            phis = self.generate_phi(while_key, do_key);
        } else {
            self.connect(self.cur_block_num, while_key);
            phis = self.generate_phi(while_key, self.cur_block_num);
        }
        if self.current() != &Token::Od {
            panic!("Error: Invalid While Statement Format, Missing \"od\"");
        }
        // phi instructions
        self.switch_block(while_key);
        let mut ori_to_new = HashMap::new();
        let mut var_to_phi = HashMap::new();
        let mut phi_insts = Vec::new();
        // let mut choices = Vec::new();
        for (var, phi) in phis {
            self.total_inst += 1;
            let phi_op = Operator::Phi(phi.0, phi.1);
            let phi_inst = Inst::new(self.total_inst, phi_op);
            ori_to_new.insert(phi.0, self.total_inst);
            var_to_phi.insert(var, self.total_inst);
            phi_insts.push(phi_inst);
        }
        // update instruction based on phi
        self.update_by_phi(ori_to_new, while_key);
        self.update_table_with_insts(var_to_phi, while_key);
        for inst in phi_insts {
            let cur_block = if let Some(b) = self.blocks.get(&self.cur_block_num) {
                b
            } else {
                panic!("Error: No Block Found at {}", self.cur_block_num)
            };
            cur_block.borrow_mut().push_head(inst);
        }

        self.total_block += 1;
        let od_key = self.total_block;
        let od_block = RefCell::new(Block::new(
            self.total_block,
            "od".to_string(),
            self.get_table_from_block(while_key),
        ));

        self.blocks.insert(self.total_block, od_block);
        self.connect(while_key, od_key);
        self.set_dom(while_key, od_key);

        self.switch_block(od_key);
        self.add_inst_to_head(Operator::EMPTY);
        if let Some(block_name) = self.get_current_block_name() {
            self.update_rel_op(while_key, (cond, block_name));
        }
        self.move_token();
    }
    fn return_statement(&mut self) {
        // "return" [ expression ]
        println!("In Return: {}", self.current());
        let return_var = self.expression();
        let return_op = Operator::Ret(return_var);
        let return_inst = self.add_inst_to_tail(return_op);
    }
    fn statement(&mut self) {
        // statement {";" statement } [";"]
        // placeholder for now
        // each function should return i32 value (inst#) like func_call
        self.move_token();
        match &self.current() {
            Token::Let => {
                self.assignment();
            }
            Token::Call => {
                self.move_token();
                self.func_call();
            }
            Token::If => {
                self.if_statement();
            }
            Token::While => {
                self.while_statement();
            }
            Token::Return => {
                self.return_statement();
            }
            Token::Symbol(Symbol::CloseBrace) | Token::Fi | Token::Else | Token::Od => (),
            _ => panic!("Error: Invalid Statement format: {}", self.current()),
        };
    }
    fn stat_sequence(&mut self) {
        // assignment | funcCall | ifStatement | whileStatement | returnStatement
        self.statement();
        println!("Current Token at StatSequence: {}", self.current());
        while self.current() == &Token::Symbol(Symbol::SemiColon) {
            println!(
                "In While loop: StatSequence\nCurrent Token:{}",
                self.current()
            );
            self.statement();
        }
    }

    fn var_decl(&mut self) {
        // "var" ident {"," ident} ";"
        println!("Current Token at VarDecl: {}", self.current());
        self.move_token();
        match self.current() {
            Token::Ident(UserDefined(var)) => self.vars.insert(var.to_string(), None),
            _ => return,
        };
        self.move_token();
        while self.current() == &Token::Symbol(Symbol::Comma) {
            self.move_token();
            // check if the token type is identifier (Ident)
            match self.current() {
                Token::Ident(UserDefined(var)) => self.vars.insert(var.to_string(), None),
                _ => panic!("Error: Invalid VarDecl Format - Missing Variable Name after Comma"),
            };
            self.move_token();
        }
        if self.current() != &Token::Symbol(Symbol::SemiColon) {
            panic!(
                "Error: Missing SemiColon: \";\"\nCurrent Token: {}",
                self.current()
            );
        }
        self.inst_storage = InstStorage::new();
        println!("End of Var Decl {}", self.current());
    }

    // TODO: TEST
    fn func_decl(&mut self) {
        // ["void"] "function" ident formalParam ";" funcBody ";"
        // new block for bb0
        // new block for start
        let global_vars = self.vars.clone();
        // self.move_token();
        // this tells that the function must have return value or not
        let is_void = if self.current() == &Token::Void {
            self.move_token();
            true
        } else {
            false
        };

        if self.current() != &Token::Function {
            panic!("Error: Missing function keyword: Function");
        }
        self.move_token();
        let func_name = match self.current() {
            Token::Ident(UserDefined(func)) => func.clone(),
            _ => panic!("Error: Missing function name: Ident(_)"),
        };
        if is_void {
            self.void_funcs.push(func_name.clone());
        }

        self.total_block += 1;
        let func_block0_key = self.total_block;
        let func_block0 = RefCell::new(Block::new(
            self.total_block,
            "func_block0".to_string(),
            HashMap::new(),
        ));
        self.blocks.insert(func_block0_key, func_block0);
        self.switch_block0(func_block0_key);

        self.total_block += 1;
        let func_key = self.total_block;
        let func_block = RefCell::new(Block::new(self.total_block, func_name, HashMap::new()));
        self.blocks.insert(func_key, func_block);
        self.switch_block(func_key);

        self.connect(func_block0_key, func_key);
        let params = self.formal_param();
        if self.current() == &Token::Symbol(Symbol::SemiColon) {
            println!("Going to Func Body");
            self.move_token();
            self.func_body();
            if is_void != self.is_void(self.cur_block_num) {
                panic!(
                    "Error: not matching the function type and return type: {}",
                    self.current()
                );
            }
        } else {
            panic!("Error: Missing Semicolon for formal param");
        }
        if self.current() != &Token::Symbol(Symbol::SemiColon) {
            panic!("Error: Missing Semicolon for func body");
        }
        self.vars = global_vars;
        self.insts = HashMap::new();
        self.switch_block(func_key);
        if let Some(name) = self.get_current_name() {
            self.funcs.insert(name, params);
        }
        self.switch_block0(0);
    }
    fn formal_param(&mut self) -> usize {
        // return # of parameters will be nice
        // "( [ident {"," ident}] ")"
        // getPar insts! TODO: Fix the format below. No loop, but check

        let mut params = 0;
        self.move_token();
        if self.current() == &Token::Symbol(Symbol::OpenParen) {
            // self.move_token();
            // if matches!(self.current(), &Token::Ident(_)) {
            //     // do something with identifier
            //     params += 1;
            // }
            // self.move_token();
            // while self.current() == &Token::Symbol(Symbol::Comma) {
            //     self.move_token();
            //     if matches!(self.current(), &Token::Ident(UserDefined(_))) {
            //         // do something with identifier
            //         params += 1;
            //     }
            //     self.move_token();
            // }

            let mut op;
            let mut par;
            // param1:
            self.move_token();
            match self.current() {
                Token::Ident(UserDefined(par1)) => {
                    params += 1;
                    op = Operator::GetPar1;
                    par = par1.to_string();

                    let par1_inst = self.add_inst_to_tail(op);
                    self.update_table(par.to_string(), par1_inst);
                    println!("vars : {:?}", self.vars);
                    self.move_token();
                }
                _ => (),
            }
            // param2:
            if self.current() == &Token::Symbol(Symbol::Comma) {
                self.move_token();
                match self.current() {
                    Token::Ident(UserDefined(par2)) => {
                        params += 1;
                        op = Operator::GetPar2;
                        par = par2.to_string();

                        let par2_inst = self.add_inst_to_tail(op);
                        self.update_table(par.to_string(), par2_inst);
                        self.move_token();
                    }
                    _ => (),
                }
            }
            // param3:
            if self.current() == &Token::Symbol(Symbol::Comma) {
                self.move_token();
                match self.current() {
                    Token::Ident(UserDefined(par3)) => {
                        params += 1;
                        op = Operator::GetPar3;
                        par = par3.to_string();

                        let par3_inst = self.add_inst_to_tail(op);
                        self.update_table(par.to_string(), par3_inst);
                        self.move_token();
                    }
                    _ => (),
                }
            }
            if self.current() == &Token::Symbol(Symbol::CloseParen) {
                self.move_token();
            } else {
                panic!("Error: Missing Closed Parenthesis: {}", self.current());
            }
        }
        return params;
    }
    fn func_body(&mut self) {
        // [varDecl] "{" [statSequence] "}"
        // I'm thinking that the return value must be the return inst or none
        // so, I can check it with the existance of "void"

        // here
        println!("Vars in func body: {:?}", self.vars);
        println!("In FuncBody: {}", self.current());
        self.var_decl();
        println!("After Func's VarDecl: {}", self.current());
        self.move_token();
        if self.current() == &Token::Symbol(Symbol::OpenBrace) {
            println!("Before Stat Sequence for Func: {}", self.current());
            self.stat_sequence();
            if self.current() != &Token::Symbol(Symbol::CloseBrace) {
                panic!("Error: FuncBody missing Closed Brace: \"}}\"");
            }
        }
        self.move_token();
        println!("End of Func Body: {}", self.current());
    }
    fn computation(&mut self) {
        // "main" [varDecl] {funcDecl} "{" statSequence "}" "."
        println!("Current Token at the beginning: {}", self.current());
        if self.current() != &Token::Main {
            panic!("Error: Missing Main keyword");
        }
        self.move_token();
        if self.current() == &Token::Var {
            self.var_decl();
            self.move_token();
        }
        println!("Before Checking Function: {}", self.current());
        while self.current() == &Token::Void || self.current() == &Token::Function {
            println!("Going to Func Decl");
            self.func_decl();
            self.move_token();
        }
        println!("After Function: {}", self.current());
        self.total_block += 1;
        let main_key = self.total_block;
        let main_block = RefCell::new(Block::new(main_key, "block".to_string(), HashMap::new()));
        self.switch_block(self.total_block);
        self.blocks.insert(self.total_block, main_block);

        if self.current() != &Token::Symbol(Symbol::OpenBrace) {
            panic!("Error: Missing Opened Brace for Main");
        }
        self.stat_sequence(); // Stat Sequence Path
        if self.current() != &Token::Symbol(Symbol::CloseBrace) {
            panic!(
                "Error: Missing Closed Brace in Computation: {}",
                self.current()
            );
        }
        self.move_token();
        println!("Current Token after closed brace: {}", self.current());

        if self.current() != &Token::Symbol(Symbol::Period) {
            panic!(
                "Eror: Missing Period in Computation: \".\"\n Current Token: {}",
                self.current()
            );
        }
        // self.blocks.push(block);
        self.connect(0, main_key);
    }

    /// helper functions to add instruction to the current block
    ///
    ///
    fn add_const_to_bb0(&mut self, op: Operator) -> i32 {
        self.total_inst += 1;
        let inst_num = self.total_inst * (-1);
        let new_const = Inst::new(inst_num, op.clone());
        let bb0 = if let Some(b) = self.blocks.get(&self.block0_num) {
            b
        } else {
            panic!("Error: No Block Found at {}", self.cur_block_num)
        };
        bb0.borrow_mut().push_tail(new_const);
        self.insts.insert(op, inst_num);
        inst_num
    }

    fn add_inst_to_tail(&mut self, op: Operator) -> i32 {
        self.total_inst += 1;
        let new_inst = Inst::new(self.total_inst, op);
        let cur_block = if let Some(b) = self.blocks.get(&self.cur_block_num) {
            b
        } else {
            panic!("Error: No Block Found at {}", self.cur_block_num)
        };
        cur_block.borrow_mut().push_tail(new_inst);
        self.total_inst
    }

    fn add_inst_to_head(&mut self, op: Operator) -> i32 {
        self.total_inst += 1;
        let new_inst = Inst::new(self.total_inst, op);
        let cur_block = if let Some(b) = self.blocks.get(&self.cur_block_num) {
            b
        } else {
            panic!("Error: No Block Found at {}", self.cur_block_num)
        };
        cur_block.borrow_mut().push_head(new_inst);
        self.total_inst
    }

    fn set_up_table(&mut self) {}

    fn switch_block(&mut self, key: usize) {
        self.cur_block_num = key;
    }

    fn switch_block0(&mut self, key: usize) {
        self.block0_num = key;
    }

    fn show_vars(&self) {
        println!("Vars: {:?}", self.vars);
    }

    fn show_insts(&self) {
        println!("Insts: {:?}", self.insts);
    }

    fn show_inst_storage(&self) {
        println!("Inst Storage: {:?}", self.inst_storage);
    }

    fn show_blocks(&self) {
        // println!("Block0: ");
        println!("Block#: {}", self.blocks.len());
        println!("Blocks:");
        for i in 0..self.blocks.len() {
            if let Some(b) = self.blocks.get(&i) {
                println!("{:?}", b.borrow());
            }
        }
        // println!("{:?}", self.block0.borrow());
    }

    fn update_table(&mut self, var: String, inst_num: i32) {
        if let Some(b) = self.blocks.get(&self.cur_block_num) {
            b.borrow_mut().update_table(var.clone(), inst_num);
            self.vars.insert(var, Some(inst_num));
        }
    }

    fn connect(&mut self, front_num: usize, back_num: usize) {
        let front_block = if let Some(front) = self.blocks.get(&front_num) {
            front
        } else {
            panic!(
                "Error: No Found Front Block at {} in connect function",
                front_num
            )
        };
        front_block.borrow_mut().add_next(back_num as usize);
        // back.borrow_mut()
        //     .add_prev(front.borrow().get_block_num() as usize);
    }

    fn set_dom(&mut self, x_num: usize, y_num: usize) {
        let x_block = if let Some(x) = self.blocks.get(&x_num) {
            x
        } else {
            panic!("Error: No X Block Found at {} in set_dom function", x_num,)
        };
        x_block.borrow_mut().add_dom(y_num as usize);
    }

    fn generate_phi(&mut self, pre_key: usize, now_key: usize) -> Vec<(String, (i32, i32))> {
        if let (Some(pre), Some(now)) = (self.blocks.get(&pre_key), self.blocks.get(&now_key)) {
            let vars = pre.borrow().compare_table(now);
            let mut phis = Vec::new();
            for (var, (pre_b, b)) in vars {
                let phi_inst = (pre_b, b);
                phis.push((var, phi_inst));
            }
            return phis;
        }
        vec![]
    }

    fn update_by_phi(&mut self, phis: HashMap<i32, i32>, start_key: usize) {
        println!("Update By Phi");
        println!(
            "start key: {}\ntotal_block: {}",
            start_key, self.total_block
        );
        for i in start_key..self.total_block + 1 {
            println!("Current i: {}", i);
            if let Some(block) = self.blocks.get(&i) {
                block.borrow_mut().update_inst(&phis);
            }
        }
    }

    fn update_rel_op(&mut self, cond_key: usize, loc_rhs: (i32, String)) {
        if let Some(bb) = self.blocks.get(&cond_key) {
            bb.borrow_mut().fill_in_none(loc_rhs);
        }
    }

    fn update_table_with_insts(&mut self, var_to_phi: HashMap<String, i32>, start_key: usize) {
        for i in start_key..self.total_block + 1 {
            println!("Current i: {}", i);
            if let Some(block) = self.blocks.get(&i) {
                block.borrow_mut().update_table_with_insts(&var_to_phi);
            }
        }
    }

    fn get_table_from_block(&self, key: usize) -> HashMap<String, Option<i32>> {
        if let Some(block) = self.blocks.get(&key) {
            return block.borrow().get_table();
        }
        HashMap::new()
    }

    fn is_empty_inst(&self) -> bool {
        if let Some(block) = self.blocks.get(&self.cur_block_num) {
            return block.borrow().get_inst_num() == 0;
        }
        return true;
    }

    fn get_current_name(&self) -> Option<String> {
        if let Some(block) = self.blocks.get(&self.cur_block_num) {
            return Some(block.borrow().get_func_name());
        }
        None
    }
    fn get_current_block_name(&self) -> Option<String> {
        if let Some(block) = self.blocks.get(&self.cur_block_num) {
            return Some(block.borrow().get_block_name());
        }
        None
    }

    fn get_current_head(&self) -> Option<i32> {
        if let Some(block) = self.blocks.get(&self.cur_block_num) {
            if let Some(h) = block.borrow().get_head_num() {
                return Some(h);
            }
        }
        None
    }
    fn get_head(&self, key: usize) -> Option<i32> {
        if let Some(block) = self.blocks.get(&key) {
            if let Some(h) = block.borrow().get_head_num() {
                return Some(h);
            }
        }
        None
    }

    fn is_void(&self, key: usize) -> bool {
        if let Some(block) = self.blocks.get(&key) {
            if matches!(block.borrow().get_tail_op(), Operator::Ret(_)) {
                return false;
            }
        }
        true
    }

    fn get_bb0(&self, op: &Operator) -> Option<i32> {
        if let Some(bb0) = self.blocks.get(&self.block0_num) {
            return bb0.borrow().get_inst(op);
        }
        None
    }

    fn visualize_ir(&self) {
        println!("digraph G {{");
        for i in 0..self.total_block + 1 {
            if let Some(bb) = self.blocks.get(&i) {
                print!(
                    "{} [shape=record, label=\"<b>{} |{{",
                    bb.borrow().get_block_name(),
                    bb.borrow().get_block_name(),
                );
                for j in 0..bb.borrow().get_insts().len() {
                    if j == bb.borrow().get_insts().len() - 1 {
                        print!(
                            "{}:{}",
                            bb.borrow().get_insts()[j].clone().get_inst_num(),
                            bb.borrow().get_insts()[j].clone().get_operator()
                        );
                        break;
                    }
                    print!(
                        "{}:{}|",
                        bb.borrow().get_insts()[j].clone().get_inst_num(),
                        bb.borrow().get_insts()[j].clone().get_operator()
                    );
                }
                println!("}}\"]");
            }
        }

        for i in 0..self.total_block + 1 {
            if let Some(bb) = self.blocks.get(&i) {
                for n in bb.borrow().clone().get_nexts() {
                    if let Some(dd) = self.blocks.get(&n) {
                        println!(
                            "{} :s -> {} :n ;",
                            bb.borrow().get_block_name(),
                            dd.borrow().get_block_name(),
                        );
                    }
                }
            }
        }

        // doms:
        for i in 0..self.total_block + 1 {
            if let Some(bb) = self.blocks.get(&i) {
                for n in bb.borrow().clone().get_doms() {
                    if let Some(dd) = self.blocks.get(&n) {
                        println!(
                            "{}:b -> {}:b[color=blue, style=dotted, label=\"dom\"]",
                            bb.borrow().get_block_name(),
                            dd.borrow().get_block_name(),
                        )
                    }
                }
            }
        }
        println!("}}");
    }
    // pub fn arithm(&mut self, op: Operator, x: &mut Result, y: &mut Result) {
    //     let mut z = Result::new(Kind::Const, 0, 0, 0);
    //     if x.get_kind() == Kind::Const && y.get_kind() == Kind::Const {
    //         let x_value = x.get_value();
    //         let y_value = y.get_value();
    //         match op {
    //             Operator::Add(_, _) => z.set_value(x_value + y_value),
    //             Operator::Mul(_, _) => z.set_value(x_value * y_value),
    //             Operator::Sub(_, _) => z.set_value(x_value - y_value),
    //             Operator::Div(_, _) => z.set_value(x_value / y_value),
    //             _ => (),
    //         }
    //     } else {
    //         self.load(x);
    //         if y.get_kind() == Kind::Const {
    //             z.set_regn(self.allocateReg());
    //             // PUT immop[op], z.regn, x.regn, y.value
    //             self.deallocate(x.get_regn());
    //             z.set_kind(Kind::Reg);
    //         } else {
    //             self.load(y);
    //             z.set_regn(self.allocateReg());
    //             self.deallocate(x.get_regn());
    //             self.deallocate(y.get_regn());
    //         }
    //     }
    // }

    // fn allocateReg(&mut self) -> i32 {
    //     for i in 1..32 {
    //         if self.busy[i] == 0 {
    //             return i as i32;
    //         }
    //     }
    //     return -1;
    // }

    // fn deallocate(&mut self, i: i32) {
    //     self.busy[i as usize] = 0;
    // }

    // pub fn load(&mut self, x: &mut Result) {
    //     if x.get_kind() == Kind::Const {
    //         x.set_regn(self.allocateReg());
    //         x.set_kind(Kind::Reg);
    //         // Put occurs: ADDI x.regn, 0, x.value
    //     } else if x.get_kind() == Kind::Var {
    //         x.set_regn(self.allocateReg());
    //         // Put occurs: LOAD x.regn, base_reg, x.address
    //         x.set_kind(Kind::Reg);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_propa_test() {
        let input = String::from(
            "main
        var a, b, c, d; {
            let a <- 1 + 1;
            let b <- a; 
            let c <- a + 1; 
            let d <- b + 1;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn calc_test1() {
        let input = String::from(
            "main
        var a, b, c; {
            let a <- 1;
            let b <- 2; 
            let a <- a + 1;
            let c <- b + a - 2;
            let a <- c / 2;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn calc_test2() {
        let input = String::from(
            "main
        var a; {
            let a <- -1;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn calc_test3() {
        let input = String::from(
            "main
        var a; {
            let a <- 1 - (3 + 1) - 3;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn calc_test4() {
        let input = String::from(
            "main
        var a, b; {
            let a <- 1 / (3 * 1) * 3;
            let b <- 2 / (3 * 1) * 3 - 1;
            let b <- call InputNum() + call InputNum();
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }
    #[test]
    fn func_call_test() {
        let input = String::from(
            "main
        var a; {
            let a <- call InputNum();
            call OutputNewLine();
            call OutputNum(a);
            call OutputNum(call InputNum());
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn if_statement_test1() {
        let input = String::from(
            "main
        var a; {
        let a <- call InputNum();
            if 1 == 2 then
    let a <- a - 1;
fi
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }
    #[test]
    fn if_statement_test2() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
            if 1 > 2 then
    let a <- a - 1;
    else
    let a <- a;
fi
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn nested_if() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
            if 1 == 2 then
                let a <- a + 1;
                if 1 == 2 then
                    let a <- a - 1;
                fi;
                else 
                    if 1 == 3 then let a <- a - 1; fi;
                let a <- 67 + 67;
            fi;
        let a <- 2;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }
    #[test]
    fn nested_if2() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
            if 1 == 2 then
                let a <- a + 1;
                if 1 == 2 then
                    let a <- a;
                else 
                    let a <- 1;
                fi;
                else 
                    if 1 == 3 then let a <- a - 1; fi;
                let a <- 67 + 67;
            fi;
        let a <- 2;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
    }
    #[test]
    fn nested_while() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
            while 1 == a do
                while 1 == a do 
                    let a <- a - 1;
                od;
            let a <- a + 1;
            od
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn nested_while2() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
            while 1 == a do
                while 1 == a do 
                    while 2 == a do
                        let a <- a - 1;
                    od;
                od;
            let a <- a + 1;
            od
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn while_test() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
            while 1 == a do
                
                    let a <- a - 1;
                od
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
    }
    #[test]
    fn if_while_test() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
        if 1 == 2 then
            while 1 == a do
                
                    let a <- a - 1;
                od;
        fi;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
    }
    #[test]
    fn if_else_while_test() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
        if 1 == 2 then
            while 1 == a do
                
                    let a <- a - 1;
                od;
        else 
            while 1 == a do 
                let a <- a + 1;
            od;
        fi;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn while_if_test() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
        while 1 == 2 do
            if 1 == a then
                let a <- a - 1;
            else 
                while 1 == a do 
                    let a <- a + 1;
                od;
            fi;
        od;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn phi_func_test() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
        if 1 == 2 then
            let a <- 4;
        fi;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
    }

    #[test]
    fn opt_test1() {
        let input = String::from(
            "main
        var a, b, c; {
            let a <- 1;
            let b <- a * 1;
            let a <- a * 1 + a * 1;
            
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.show_inst_storage();
        parse.visualize_ir();
    }

    #[test]
    fn user_defined_test() {
        let input = String::from(
            "main
    var a, b;

    function sum(a); var c; {
        let c <- a;
        return c;
    };

    {
    let a <- 1;
    let b <- call sum(a);
    call OutputNum(b);
    }
    .",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn two_param_test() {
        let input = String::from(
            "main
    var a, b;

    function sum(a, b); var c; {
        let c <- a;
        return c;
    };

    {
    let a <- 1;
    let b <- call sum(a, b);
    call OutputNum(b);
    }
    .",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn three_param_test() {
        let input = String::from(
            "main
    var a, b, e;

    function sum(a, b, d); var c; {
        let c <- a + d;
        return c;
    };

    {
    let a <- 1;
    let b <- 2; 
    let e <- 3;
    let b <- call sum(a, b, e);
    call OutputNum(b);
    }
    .",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn no_need_phi_test() {
        let input = String::from(
            "main
        var a; {
            let a <- 1;
            if 1 == 2 then
            let a <- a;
        fi;
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn func_func_call_test() {
        let input = String::from(
            "main
    var a, b, e;

    function sum(a, b, d); var c; {
        let c <- a + d;
        let a <- a + d; 
        return c;
    };

    {
    let a <- 1;
    let b <- 2; 
    let e <- 3;
    call OutputNum(call sum(a, b));
    }
    .",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn while_func_call_test() {
        let input = String::from(
            "main
    var a, b, e;

    function sum(a, b, d); var c; {
        let c <- a + d;
        let a <- a + d; 
        return c;
    };

    {
    let a <- 1;
    let b <- 2; 
    let e <- 3;
    while a > b do 
        let a <- call sum(a, b, e);
    od;
    }
    .",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn if_func_call_test() {
        let input = String::from(
            "main
    var a, b, e;

    function sum(a, b, d); var c; {
        let c <- a + d;
        let a <- a + d; 
        return c;
    };

    {
    let a <- 1;
    let b <- 2; 
    let e <- 3;
    if a > b then
        let a <- call sum(a, b, e);
    fi;
    }
    .",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn func_while_test() {
        let input = String::from(
            "main
    var a, b, e;

    function sum(a, b, d); var c; {
        while 1 == a do
            let a <- a - 1;
        od;
        return c;
    };

    {
    let a <- 2;
    let a <- a - 1;
    let b <- 2; 
    let e <- 3;
    let a <- call sum(a, b);
    call OutputNum(a);
    }
    .",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    #[should_panic]
    fn detect_void_assignment() {
        let input = String::from(
            "main
    var a, b, e;

    function sum(a, b, d); var c; {
        let c <- a + d;
        return c;
    };

    function sum2(a, b); var d; {
        let d <- a + b + call sum(a, b, d);
    };

    {
    let a <- 1;
    let b <- 2; 
    let e <- 3;
    let b <- call sum2(a, b);
    call OutputNum(b);
    }
    .",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    // true positive tests
    // for user-defined functions
    #[test]
    #[should_panic]
    fn different_func_return_type() {
        let input = String::from(
            "main
    var a, b, e;

    function sum(a, b, d); var c; {
        let c <- a + d;
        return c;
    };

    void function sum2(a, b); var d; {
        if 1 == 2 then
            let a <- a;
        fi;
        return a;
    };

    {
    let a <- 1;
    let b <- 2; 
    let e <- 3;
    let b <- call sum2(a, b);
    call OutputNum(b);
    }
    .",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }
}
