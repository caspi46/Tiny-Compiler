use crate::frontend::fsm::{
    token::{
        Ident::UserDefined, Op::ADD, Op::DIV, Op::MUL, Op::SUB, RelOp::EQ, RelOp::GE, RelOp::GT,
        RelOp::LE, RelOp::LT, RelOp::NE, Symbol, Token, Token::Ident,
    },
    tokenizer::Tokenizer,
};
use crate::frontend::operators::{Inst, InstStorage, Operator};
use crate::frontend::parsing::block::Block;
use crate::frontend::parsing::result::{Kind, Result};
use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, HashMap, LinkedList};
use std::sync::Arc;

#[derive()]
pub struct Parser {
    tokens: Vec<Token>,
    blocks: BTreeMap<usize, RefCell<Block>>,
    vars: HashMap<String, Option<i32>>,
    insts: HashMap<Operator, i32>,
    funcs: HashMap<Token, Vec<Token>>,
    cur_token_index: usize,
    // cur_block: &'a RefCell<Block>,
    cur_block_num: usize,
    // busy: Vec<i32>,
    block0: RefCell<Block>, // after the computation, it will be added in top
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
        let block0 = RefCell::new(Block::new(0, "block_".to_string(), HashMap::new()));
        let zero_inst = Inst::new(0, Operator::Const(0));
        block0.borrow_mut().push_head(zero_inst);
        let mut blocks = BTreeMap::new();
        blocks.insert(
            1,
            RefCell::new(Block::new(1, "block_".to_string(), HashMap::new())),
        );

        Self {
            tokens,
            blocks,
            vars: HashMap::new(),
            insts: HashMap::new(),
            funcs: HashMap::new(),
            cur_token_index: 0,
            cur_block_num: 1,
            // busy,
            total_inst: 0,
            total_block: 1,
            inst_storage: InstStorage::new(),
            block0: block0, // block0 will be added to front at the end
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
                self.move_token();
                if self.current() != &Token::Symbol(Symbol::CloseParen) {
                    panic!("Error, missing closed parentheses, \")\"");
                }
                self.move_token();
                return inst_num;
            }
            Token::Number(num) => {
                // add num value in block 0

                let op = Operator::Const(num);
                self.move_token();
                if let Some(i_num) = self.insts.get(&op) {
                    return *i_num;
                }
                self.total_inst += 1;
                let inst_num = self.total_inst;
                let new_num = Inst::new(inst_num * (-1), op.clone());
                self.block0.borrow_mut().push_tail(new_num);
                self.insts.insert(op, inst_num);
                println!("Num's Token: {}", self.current());
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
                    return inst_num;
                }
                panic!("Error: Invalid factor, using the uninitialized variable");
            }
            Token::Call => {
                // skip for now (maybe after I solve the ident and num)
                let inst_num = self.func_call();
                return inst_num; // Should call funcCall 
            }
            _ => {
                panic!("Error: Invalid factor format: {}", self.current());
            }
        }
    }
    fn term(&mut self) -> i32 {
        // factor { ("*" | "/") factor }
        let x: i32 = self.factor();
        // self.vars.insert(x, self.total_inst);
        println!("Current token after Factor: {}", self.current());
        if self.current() != &Token::Op(MUL) && self.current() != &Token::Op(DIV) {
            return x;
        }

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
            let inst_num = self.add_inst_to_tail(op.clone());
            self.insts.insert(op, inst_num);
            println!("end of the MUL&DIV loop : {}", self.current());
        }
        // should return i32
        // i32 is the inst# that takes the calculation result
        self.total_inst
    }
    fn expression(&mut self) -> i32 {
        // term { ("+" | "-") terrm }
        let x = self.term(); // return the 
        // no add or sub calculation
        // gets the same inst# as term
        // self.move_token();
        if self.current() != &Token::Op(ADD) && self.current() != &Token::Op(SUB) {
            // self.move_token();
            return x;
        }
        let mut calc;
        while self.current() == &Token::Op(ADD) || self.current() == &Token::Op(SUB) {
            println!("Current Token at ADD&SUB loop: {}", self.current());
            calc = self.current().clone();
            let y = self.term();
            let op = match calc {
                Token::Op(ADD) => Operator::Add(x, y),
                Token::Op(SUB) => Operator::Sub(x, y),
                _ => panic!("Error: Invalid ADD or SUB format"),
            };
            let inst_num = self.add_inst_to_tail(op.clone());
            self.insts.insert(op, inst_num);
        }
        self.total_inst // TODO: test if the return value is correct
        // return the i32 value for that takes the calculation for example: add (1) (2) -> (1)
    }
    fn relation(&mut self) -> (i32, i32) {
        // should return
        // // expression relOp expression
        println!("Current Token in Relation: {}", self.current());
        let lhs = self.expression(); // first in RelOp (v1, v2)
        // while matches!(self.current(), &Token::RelOp(_)) {
        print!("Current Token after LHS in relation: {}", self.current());
        // }
        let rhs = self.expression();
        let cmp_inst = self.add_inst_to_tail(Operator::Cmp(lhs, rhs));
        let rel_inst = match self.current() {
            &Token::RelOp(EQ) => self.add_inst_to_tail(Operator::Beq(lhs, rhs)),
            &Token::RelOp(NE) => self.add_inst_to_tail(Operator::Bne(lhs, rhs)),
            &Token::RelOp(GT) => self.add_inst_to_tail(Operator::Bgt(lhs, rhs)),

            &Token::RelOp(LT) => self.add_inst_to_tail(Operator::Blt(lhs, rhs)),
            &Token::RelOp(GE) => self.add_inst_to_tail(Operator::Bge(lhs, rhs)),
            &Token::RelOp(LE) => self.add_inst_to_tail(Operator::Blt(lhs, rhs)),
            _ => {
                panic!("Error, missing relOp: ==, !=, >, <, >=, <=");
            }
        };
        (cmp_inst, rel_inst)
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
        self.vars.insert(var.clone(), Some(rhs));
        let cur_block = if let Some(b) = self.blocks.get(&self.cur_block_num) {
            b
        } else {
            panic!("Error: No Block Found at {}", self.cur_block_num)
        };
        cur_block.borrow_mut().update_table(var, rhs);
        rhs
    }

    fn func_call(&mut self) -> i32 {
        // "call" ident [ "(" [expression {"," expression}] ")"]
        self.move_token();
        match self.current() {
            Token::Ident(InputNum) => (),
            Token::Ident(OutputNum) => (),
            Token::Ident(OutputNewLine) => (),
            Token::Ident(UserDefined(func)) => (),
            _ => panic!("Error: Invalid funcCall format"),
        };
        -1
    }
    fn if_statement(&mut self) {
        // "if" relation "then" statSequence ["else" statSequence] "fi"
        // new block: if, then, else, fi

        // new blocks! IF, THEN, FI
        // if block
        self.total_block += 1;
        let before_table = self.vars.clone();
        let if_block = RefCell::new(Block::new(
            self.total_block,
            "if_".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block as usize, if_block);
        let if_key = self.total_block;
        self.connect(self.cur_block_num, if_key);

        // then block
        self.total_block += 1;
        let then_block = RefCell::new(Block::new(
            self.total_block,
            "then_".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block, then_block);
        let then_key = self.total_block;

        // fi block
        self.total_block += 1;
        let fi_block = RefCell::new(Block::new(
            self.total_block,
            "fi_".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block, fi_block);
        let fi_key = self.total_block;
        // (if_block, then_block) = self.connect(if_block, then_block);

        // add insts in if block
        self.switch_block(if_key);
        let (cmp, cond) = self.relation(); // if_block saved them already
        println!("Current Token after IF: {}", self.current());

        if self.current() != &Token::Then {
            panic!("Error: Invalid If Statement Format, Missing \"then\"");
        }
        // (then_block, fi_block) = self.connect(then_block, fi_block);
        self.switch_block(then_key);
        self.stat_sequence(); // add all the instructions in the then to the then block
        self.connect(if_key, then_key);

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
        // At this point Right should be the original inst#
        // TODO: identify which variable is updated
        println!("Current Token after THEN: {}", self.current());
        if self.current() == &Token::Else {
            // since else is optional
            self.total_block += 1;
            println!("Current Table at Else : {:?}", before_table.clone());
            let else_block = RefCell::new(Block::new(
                self.total_block,
                "else_".to_string(),
                before_table,
            ));
            // (if_block, else_block) = self.connect(if_block, else_block);
            self.blocks.insert(self.total_block, else_block);
            let else_key = self.total_block;
            self.connect(if_key, else_key);
            self.switch_block(else_key);
            self.stat_sequence();
            if self.cur_block_num != else_key {
                self.connect(self.cur_block_num, fi_key);
                phis = self.generate_phi(self.cur_block_num, else_key);
            } else {
                self.connect(else_key, fi_key);
                phis = self.generate_phi(then_key, else_key);
            }
            // update phi functions here for fi block
            // TODO: identify which variable is updated
            // find the phi function in the fi block
            // update its RHS in phi function
        } else {
            // (if_block, fi_block) = self.connect(if_block, fi_block);
            self.connect(if_key, fi_key);
        }

        if self.current() != &Token::Fi {
            panic!("Error: Invalid If Statement Format, Missing \"fi\"");
        }

        self.switch_block(fi_key);
        for (var, phi) in phis {
            let inst_num = self.add_inst_to_tail(phi);
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
        let while_num = self.total_block;
        let while_block = RefCell::new(Block::new(
            self.total_block,
            "while_".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block, while_block);

        self.total_block += 1;
        let do_num = self.total_block;
        let do_block = RefCell::new(Block::new(
            self.total_block,
            "do_".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block, do_block);

        self.total_block += 1;
        let od_num = self.total_block;
        let od_block = RefCell::new(Block::new(
            self.total_block,
            "od_".to_string(),
            before_table.clone(),
        ));
        self.blocks.insert(self.total_block, od_block);

        // Edges
        self.connect(self.cur_block_num, while_num);
        self.connect(while_num, do_num);
        self.connect(while_num, od_num);

        self.switch_block(while_num);
        self.relation(); // TODO: Return Value 
        if self.current() != &Token::Do {
            panic!("Error: Invalid While Statement Format, Missing \"do\"");
        }
        self.switch_block(do_num);
        self.stat_sequence();
        let phis;
        if self.cur_block_num == do_num {
            self.connect(do_num, while_num);
            phis = self.generate_phi(while_num, do_num);
        } else {
            self.connect(self.cur_block_num, while_num);
            phis = self.generate_phi(while_num, self.cur_block_num);
        }
        if self.current() != &Token::Od {
            panic!("Error: Invalid While Statement Format, Missing \"od\"");
        }
        self.switch_block(od_num);
        self.move_token();
    }
    fn return_statement(&mut self) {
        // "return" [ expression ]
        self.move_token();
        self.expression();
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

    // TODO: TEST
    fn var_decl(&mut self) {
        // "var" ident {"," ident} ";"
        println!("Current Token at VarDecl: {}", self.current());
        self.move_token();
        // if matches!(self.current(), &Ident(_)) {
        //     // self.formalParam();
        //     self.vars.insert(self.current().clone(), 0); // 0 for default
        // } else {
        //     return;
        // }
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
    }

    // TODO: TEST
    fn func_decl(&mut self) {
        // ["void"] "function" ident formalParam ";" funcBody ";"
        // new block
        self.move_token();
        if self.current() != &Token::Function {
            panic!("Error: Missing function keyword: Function");
        }
        self.move_token();
        if !matches!(self.current(), &Token::Ident(_)) {
            panic!("Error: Missing function name: Ident(_)");
        }
        let func = self.current().clone();
        let params = self.formal_param();
        self.funcs.insert(func, params);
        if self.current() == &Token::Symbol(Symbol::SemiColon) {
            self.move_token();
            self.func_body();
        }
        if self.current() != &Token::Symbol(Symbol::SemiColon) {
            panic!("Error: Missing Semicolon in funcDecl");
        }
    }
    fn formal_param(&mut self) -> Vec<Token> {
        // "( [ident {"," ident}] ")"
        let mut params: Vec<Token> = vec![];
        self.move_token();
        if self.current() == &Token::Symbol(Symbol::OpenParen) {
            self.move_token();
            if matches!(self.current(), &Token::Ident(_)) {
                // do something with identifier
                params.push(self.current().clone());
            }
            self.move_token();
            while self.current() == &Token::Symbol(Symbol::SemiColon) {
                self.move_token();
                if matches!(self.current(), &Token::Ident(UserDefined(_))) {
                    // do something with identifier
                    params.push(self.current().clone());
                }
                self.move_token();
            }
            if self.current() == &Token::Symbol(Symbol::CloseParen) {
                self.move_token();
            } else {
                panic!("Error: Missing Closed Parenthesis: \")\"");
            }
        }
        params
    }
    fn func_body(&mut self) {
        // [varDecl] "{" [statSequence] "}"
        self.var_decl();
        if self.current() == &Token::Symbol(Symbol::OpenBrace) {
            self.stat_sequence();
            if self.current() != &Token::Symbol(Symbol::CloseBrace) {
                panic!("Error: FuncBody missing Closed Brace: \"}}\"");
            }
        }
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
        while self.current() == &Token::Void {
            self.func_decl();
            self.move_token();
        }

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
        self.blocks.insert(0, self.block0.clone());
        self.connect(0, 1);
    }

    /// helper functions to add instruction to the current block
    ///
    ///
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

    fn show_vars(&self) {
        println!("Vars: {:?}", self.vars);
    }

    fn show_insts(&self) {
        println!("Insts: {:?}", self.insts);
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
        let mut front_block = if let Some(front) = self.blocks.get(&front_num) {
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

    fn generate_phi(&mut self, pre_key: usize, now_key: usize) -> Vec<(String, Operator)> {
        if let (Some(pre), Some(now)) = (self.blocks.get(&pre_key), self.blocks.get(&now_key)) {
            let vars = pre.borrow().compare_table(now);
            let mut phis = Vec::new();
            for (var, (pre_b, b)) in vars {
                self.total_inst += 1;
                let phi_inst = Operator::Phi(pre_b, b);
                phis.push((var, phi_inst));
            }
            return phis;
        }
        vec![]
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
    }

    #[test]
    fn calc_test2() {
        let input = String::from(
            "main
        var a; {
            let a <- -1
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
    }

    #[test]
    fn calc_test3() {
        let input = String::from(
            "main
        var a; {
            let a <- 1 - 2 / 3 - 2
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
    }

    #[test]
    fn if_statement_test1() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
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
    }
    #[test]
    fn if_statement_test2() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
            if 1 == 2 then
    let a <- a - 1;
    else
    let a <- 67 + 67;
fi
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
    }

    #[test]
    fn nested_if() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
            if 1 == 2 then
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
    }
    #[test]
    fn nested_while() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
            while 1 == 2 do
                while 1 == 2 do 
                    let a <- a - 1;
                od
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
}
