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
    blocks: BTreeMap<usize, Block>,
    vars: HashMap<Token, i32>,
    insts: HashMap<Operator, i32>,
    funcs: HashMap<Token, Vec<Token>>,
    cur_token_index: usize,
    cur_block: Block,
    // busy: Vec<i32>,
    block0: Block, // after the computation, it will be added in top
    total_inst: i32,
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
        let mut block0 = Block::new(0, "block_".to_string());
        let zero_inst = Inst::new((0, Operator::Const(0)));
        block0.push_head(zero_inst);
        Self {
            tokens,
            blocks: BTreeMap::new(),
            vars: HashMap::new(),
            insts: HashMap::new(),
            funcs: HashMap::new(),
            cur_token_index: 0,
            cur_block: Block::new(1, "blcok_".to_string()), // starts with the block1 /
            // busy,
            total_inst: 0,
            inst_storage: InstStorage::new(),
            block0, // block0 will be added to front at the end
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
        match self.current() {
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
                let inst_num = -1 * self.block0.get_inst_num();
                let op = Operator::Const(*num);
                if !self.insts.contains_key(&op) {
                    println!("{}", num);
                    let new_num = Inst::new((inst_num, op.clone()));
                    self.block0.push_tail(new_num);
                    self.insts.insert(op, inst_num);
                }
                println!("Num's Token: {}", self.current());
                // match self.insts.get(&op) {
                //     Some(inst_num) => println!("Num's inst#: {}", inst_num),
                //     _ => println!("No inst# for Num {}", self.current()),
                // };
                self.move_token();
                return inst_num;
            }
            Token::Ident(UserDefined(var)) => {
                // user-defined variable
                //
                // if the variable is already initialized to other var in the table
                if let Some(&inst_num) = self.cur_block.check_table(var) {
                    self.move_token();
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
        let x = self.factor();
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
            self.add_inst_to_tail(op.clone());
            self.insts.insert(op, self.total_inst);
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
            self.add_inst_to_tail(op.clone());
            self.insts.insert(op, self.total_inst);
        }
        self.total_inst // TODO: test if the return value is correct
        // return the i32 value for that takes the calculation for example: add (1) (2) -> (1)
    }
    fn relation(&mut self) -> (i32, i32) {
        // should return
        // // expression relOp expression
        let lhs = self.expression(); // first in RelOp (v1, v2)
        self.move_token();
        // while matches!(self.current(), &Token::RelOp(_)) {

        // }
        match self.current() {
            &Token::RelOp(EQ) => {
                self.move_token();
                let rhs = self.expression();
                let cmp_inst = self.add_inst_to_tail(Operator::Cmp(lhs, rhs));
                let eq_inst = self.add_inst_to_tail(Operator::Beq(lhs, rhs));
                (cmp_inst, eq_inst)
            }
            &Token::RelOp(NE) => {
                self.move_token();
                let rhs = self.expression();
                let cmp_inst = self.add_inst_to_tail(Operator::Cmp(lhs, rhs));
                let ne_inst = self.add_inst_to_tail(Operator::Bne(lhs, rhs));
                (cmp_inst, ne_inst)
            }
            &Token::RelOp(GT) => {
                self.move_token();
                let rhs = self.expression();
                let cmp_inst = self.add_inst_to_tail(Operator::Cmp(lhs, rhs));
                let gt_inst = self.add_inst_to_tail(Operator::Bgt(lhs, rhs));
                (cmp_inst, gt_inst)
            }
            &Token::RelOp(LT) => {
                self.move_token();
                let rhs = self.expression();
                let cmp_inst = self.add_inst_to_tail(Operator::Cmp(lhs, rhs));
                let lt_inst = self.add_inst_to_tail(Operator::Blt(lhs, rhs));
                (cmp_inst, lt_inst)
            }
            &Token::RelOp(GE) => {
                self.move_token();
                let rhs = self.expression();
                let cmp_inst = self.add_inst_to_tail(Operator::Cmp(lhs, rhs));
                let ge_inst = self.add_inst_to_tail(Operator::Bge(lhs, rhs));
                (cmp_inst, ge_inst)
            }
            &Token::RelOp(LE) => {
                self.move_token();
                let rhs = self.expression();
                let cmp_inst = self.add_inst_to_tail(Operator::Cmp(lhs, rhs));
                let le_inst = self.add_inst_to_tail(Operator::Blt(lhs, rhs));
                (cmp_inst, le_inst)
            }
            _ => {
                panic!("Error, missing relOp: ==, !=, >, <, >=, <=");
            }
        }
    }
    fn assignment(&mut self) -> i32 {
        // "let" ident "<-"  expression
        self.move_token();
        if !matches!(self.current(), &Token::Ident(UserDefined(_))) {
            panic!("Error: Invalid Assignment Format - Missing Identifier");
        }
        let var = self.current().clone();
        self.move_token();
        if self.current() != &Token::Symbol(Symbol::Init) {
            panic!("Error: Invalid Assignment Format, Missing \"<-\"");
        }
        println!("Current token before expression: {}", self.current());
        let rhs = self.expression(); // TODO: Return value (Identify the value)
        self.vars.insert(var, rhs);
        rhs
    }

    fn func_call(&mut self) -> i32 {
        // "call" ident [ "(" [expression {"," expression}] ")"]
        self.move_token();
        match self.current() {
            Token::Ident(InputNum) => (),
            Token::Ident(OutputNum) => (),
            Token::Ident(OutputNewLine) => (),
            Token::Ident(UserDefined(var)) => (),
            _ => panic!("Error: Invalid funcCall format"),
        };
        -1
    }
    fn if_statement(&mut self) {
        // "if" relation "then" statSequence ["else" statSequence] "fi"
        // new block: if, then, else, fi
        let mut if_block = Block::new(self.blocks.len() as i32, "if_".to_string());
        let mut then_block = Block::new(self.blocks.len() as i32, "then_".to_string());
        let mut fi_block = Block::new(self.blocks.len() as i32, "fi_".to_string());
        self.switch_block(if_block);
        self.move_token();
        let (cmp, cond) = self.relation(); // TODO: Return value 
        self.move_token();
        if self.current() != &Token::Then {
            panic!("Error: Invalid If Statement Format, Missing \"then\"");
        }
        self.switch_block(then_block);
        self.move_token();
        self.stat_sequence();
        // create phi functioins here for fi block (LEFT in Phi(Left, Right))
        // At this point Right should be the original inst#
        // TODO: identify which variable is updated
        self.move_token();
        if self.current() == &Token::Else {
            // since else is optional
            let mut else_block = Block::new(self.blocks.len() as i32, "else_".to_string());
            self.switch_block(else_block);
            self.move_token();
            self.stat_sequence();
            self.move_token();
            // update phi functions here for fi block
            // TODO: identify which variable is updated
            // find the phi function in the fi block
            // update its RHS in phi function
        }

        if self.current() != &Token::Fi {
            panic!("Error: Invalid If Statement Format, Missing \"fi\"");
        }

        self.switch_block(fi_block);
        // add phi functions if exists
    }
    fn while_statement(&mut self) {
        // "while" relaton "do" StatSequence "od"
        // new block: while, then
        self.move_token();
        self.relation(); // TODO: Return Value 
        self.move_token();
        if self.current() != &Token::Do {
            panic!("Error: Invalid While Statement Format, Missing \"do\"");
        }
        self.move_token();
        self.stat_sequence(); // 
        self.move_token();
        if self.current() != &Token::Od {
            panic!("Error: Invalid While Statement Format, Missing \"od\"");
        }
    }
    fn return_statement(&mut self) {
        // "return" [ expression ]
        self.move_token();
        self.expression();
    }
    fn statement(&mut self) -> i32 {
        // statement {";" statement } [";"]
        // placeholder for now
        // each function should return i32 value (inst#) like func_call
        self.move_token();
        match self.current() {
            &Token::Let => self.assignment(),
            &Token::Call => self.func_call(),
            &Token::If => {
                self.if_statement();
                -1
            }
            &Token::While => {
                self.while_statement();
                -1
            }
            &Token::Return => {
                self.return_statement();
                -1
            }
            _ => panic!("Error: Invalid Statement format: {}", self.current()),
        };

        -1
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
        if matches!(self.current(), &Ident(_)) {
            // self.formalParam();
            self.vars.insert(self.current().clone(), 0); // 0 for default
        } else {
            return;
        }
        self.move_token();
        while self.current() == &Token::Symbol(Symbol::Comma) {
            self.move_token();
            // check if the token type is identifier (Ident)
            if matches!(self.current(), &Token::Ident(_)) {
                self.vars.insert(self.current().clone(), 0);
            }
        }
        if self.current() != &Token::Symbol(Symbol::SemiColon) {
            panic!("Error: Missing SemiColon: \";\"");
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
            panic!("Error: Missing Closed Brace: {}", self.current());
        }
        self.move_token();
        println!("Current Token after closed brace: {}", self.current());

        if self.current() != &Token::Symbol(Symbol::Period) {
            panic!(
                "Eror: Missing Period: \".\"\n Current Token: {}",
                self.current()
            );
        }
        self.blocks.insert(0, self.block0.clone());
    }

    /// helper functions to add instruction to the current block
    ///
    ///
    fn add_inst_to_tail(&mut self, op: Operator) -> i32 {
        self.total_inst += 1;
        let data = (self.total_inst, op);
        let new_inst = Inst::new(data);
        self.cur_block.push_tail(new_inst);
        self.total_inst
    }

    fn add_inst_to_head(&mut self, op: Operator) -> i32 {
        self.total_inst += 1;
        let data = (self.total_inst, op);
        let new_inst = Inst::new(data);
        self.cur_block.push_head(new_inst);
        self.total_inst
    }

    fn set_up_table(&mut self) {}

    fn switch_block(&mut self, block: Block) {
        self.cur_block = block;
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
        var a; {
            let a <- 1 + 2 * 3
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
    }

    #[test]
    fn calc_test2() {
        let input = String::from(
            "main
        var a; {
            let a <- 1 + 2 * 3 - 2
    }.",
        );
        let mut parse = Parser::new(input);
        parse.computation();
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
    }
}
