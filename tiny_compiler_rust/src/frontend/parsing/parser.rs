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
            cur_block: Block::new(0, "blcok".to_string()),
            // busy,
            total_inst: 0,
            inst_storage: InstStorage::new(),
            block0,
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

        match self.current() {
            Token::Symbol(Symbol::OpenParen) => {
                let inst_num = self.expression();
                self.move_token();
                if self.current() == &Token::Symbol(Symbol::CloseParen) {
                } else {
                    panic!("Error, missing closed parentheses, \")\"");
                }
                return inst_num;
            }
            Token::Number(num) => {
                // add num value in block 0
                let inst_num = -1 * self.block0.get_inst_num();
                let op = Operator::Const(*num);
                if !self.insts.contains_key(&op) {
                    let new_num = Inst::new((inst_num, op.clone()));
                    self.block0.push_tail(new_num);
                    self.insts.insert(op, inst_num);
                }
                return inst_num;
            }
            Token::Ident(UserDefined(var)) => {
                if let Some(&inst_num) = self.cur_block.check_table(var) {
                    return inst_num;
                }
                self.total_inst += 1;
                return self.total_inst;
            }
            Token::Call => {
                let inst_num = self.func_call();
                return -1; // Should call funcCall
            }
            _ => {
                panic!("Error: Invalid factor format");
            }
        }
    }
    fn term(&mut self) {
        // factor { ("*" | "/") factor }
        let x = self.factor().clone();
        // self.vars.insert(x, self.total_inst);
        self.move_token();
        while matches!(self.current(), Token::Op(MUL)) {
            self.move_token();
            let y = self.factor();
            let mul_op = Operator::Mul(x, y);
            self.add_inst_to_tail(mul_op.clone());
            self.insts.insert(mul_op, self.total_inst);
        }

        while matches!(self.current(), Token::Op(DIV)) {
            self.move_token();
            let y = self.factor();
            let div_op = Operator::Div(x, y);
            self.add_inst_to_tail(div_op.clone());
            self.insts.insert(div_op, self.total_inst);
        }
    }
    fn expression(&mut self) -> i32 {
        // term { ("+" | "-") terrm }
        self.term();
        self.move_token();
        while self.current() == &Token::Op(ADD) {
            self.move_token();
            self.factor();
        }

        while self.current() == &Token::Op(SUB) {
            self.move_token();
            self.factor();
        }
        0 // for now
    }
    fn relation(&mut self) {
        // should return
        // // expression relOp expression
        // let first = self.expression(); // first in RelOp (v1, v2)
        // self.move_token();
        // match self.current() {
        //     &Token::RelOp(EQ) => {
        //         self.move_token();
        //         let second = self.expression();
        //         let cmp = Operator::Cmp(first, second);
        //         self.add_inst_to_tail(cmp);
        //         let eq = Operator::Eq(first, second);
        //         self.add_inst_to_tail(eq);
        //     }
        //     &Token::RelOp(NE) => (),
        //     &Token::RelOp(GT) => (),
        //     &Token::RelOp(LT) => (),
        //     &Token::RelOp(GE) => (),
        //     &Token::RelOp(LE) => (),
        //     _ => {
        //         panic!("Error, missing relOp: ==, !=, >, <, >=, <=");
        //     }
        // }
        self.move_token();
        self.expression();
    }
    fn assignment(&mut self) {
        // "let" ident "<-"  expression
        self.move_token();
        if matches!(self.current(), &Token::Ident(UserDefined(_))) {
            panic!("Error: Invalid Assignment Format, Missing Identifier");
        }
        let var = self.current().clone();
        self.move_token();
        if self.current() != &Token::Symbol(Symbol::Init) {
            panic!("Error: Invalid Assignment Format, Missing \"<-\"");
        }
        self.move_token();
        self.expression(); // TODO: Return value (Identify the value)
        self.vars.insert(var, self.total_inst);
        self.total_inst += 1;
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
        // let mut if_block = Some(Block::new(self.blocks.len() as i32, "if_".to_string()));
        // let mut then_block = Some(Block::new(self.blocks.len() as i32, "then_".to_string()));
        // let mut else_block = Some(Block::new(self.blocks.len() as i32, "else_".to_string()));
        // let mut fi_block = Some(Block::new(self.blocks.len() as i32, "fi_".to_string()));

        self.move_token();
        self.relation(); // TODO: Return value 
        self.move_token();
        if self.current() != &Token::Then {
            panic!("Error: Invalid If Statement Format, Missing \"then\"");
        }
        self.move_token();
        self.stat_sequence();
        self.move_token();
        if self.current() == &Token::Else {
            // since else is optional
            self.move_token();
            self.stat_sequence();
            self.move_token();
        }

        if self.current() != &Token::Fi {
            panic!("Error: Invalid If Statement Format, Missing \"fi\"");
        }
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
            &Token::Let => {
                self.assignment();
                -1
            }
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
            _ => panic!("Error: Invalid Statement format"),
        };
        -1
    }
    fn stat_sequence(&mut self) {
        // assignment | funcCall | ifStatement | whileStatement | returnStatement
        self.move_token();
        self.statement();
        self.move_token();
        while self.current() == &Token::Symbol(Symbol::SemiColon) {
            self.move_token();
            self.statement();
        }
    }

    // TODO: TEST
    fn var_decl(&mut self) {
        // "var" ident {"," ident} ";"
        self.move_token();
        if matches!(self.current(), &Ident(_)) {
            // self.formalParam();
            self.vars.insert(self.current().clone(), 0); // 0 for default
        } else {
            panic!("Error: Missing Identifier");
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
    }
    fn computation(&mut self) {
        // "main" [varDecl] {funcDecl} "{" statSequence "}" "."
        while self.current() == &Token::Main {
            self.move_token();
            if self.current() == &Token::Symbol(Symbol::Period) {
                break; // in case there is no variable or { }
            }
            if self.current() == &Token::Var {
                // Var Decl Path
                self.var_decl();
            }
            if self.current() == &Token::Void {
                // Func Decl Path
                self.func_decl();
            }
            if self.current() == &Token::Symbol(Symbol::OpenBrace) {
                self.stat_sequence(); // Stat Sequence Path
                if self.current() == &Token::Symbol(Symbol::CloseBrace) {
                    self.move_token();
                } else {
                    panic!("Error: Missing Closed Brace ");
                }
            }
        }
        if self.current() != &Token::Symbol(Symbol::Period) {
            panic!("Eror: Missing Period: \".\"");
        }
    }

    // helper functions to add instruction to the current block
    fn add_inst_to_tail(&mut self, op: Operator) {
        self.total_inst += 1;
        let data = (self.total_inst, op);
        let new_inst = Inst::new(data);
        self.cur_block.push_tail(new_inst);
    }

    fn add_inst_to_head(&mut self, op: Operator) {
        self.total_inst += 1;
        let data = (self.total_inst, op);
        let new_inst = Inst::new(data);
        self.cur_block.push_head(new_inst);
    }

    fn set_up_table(&mut self) {}

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
