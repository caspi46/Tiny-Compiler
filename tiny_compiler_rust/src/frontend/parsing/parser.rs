use crate::frontend::fsm::{
    token::{
        Ident, Ident::UserDefined, Op::ADD, Op::DIV, Op::MUL, Op::SUB, RelOp::EQ, RelOp::GE,
        RelOp::GT, RelOp::LE, RelOp::LT, RelOp::NE, Symbol, Token,
    },
    tokenizer::Tokenizer,
};
use crate::frontend::operators::{Inst, InstStorage, Operator};
use crate::frontend::parsing::block::Block;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

/// Parser Struct:
///
/// tokens: Token list from Tokenizer
///
/// blocks: Block list
///
/// vars: global variable list (In this project, only consider global variables)
///       and variables in function (will be deleted after parsing user defined functions)
///
/// insts: instruction list
///
/// funcs: function list
///
/// void_funcs: void function list to identify function call's return type existance
///
/// cur_token_index: current token index
///
/// cur_block_num: current block key
///
/// block0_num: current block0 key
///
/// total_inst: total number of instructions
///
/// total_block: total number of blocks
///
/// inst_storage: instruction storage for optimization
#[derive()]
pub struct Parser {
    tokens: Vec<Token>,
    blocks: BTreeMap<usize, RefCell<Block>>,
    vars: HashMap<String, Option<i32>>,
    insts: HashMap<Operator, i32>,
    funcs: HashMap<String, usize>,
    void_funcs: Vec<String>,
    cur_token_index: usize,
    cur_block_num: usize,
    block0_num: usize,
    total_inst: i32,
    total_block: usize,
    inst_storage: InstStorage,
}

impl Parser {
    /// new (constructor)
    ///
    /// input: String (Source Code)
    pub fn new(input: String) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.generate_token();
        let tokens = tokenizer.get_tokens();
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

    /// current function:
    ///
    /// return the current token
    fn current(&self) -> &Token {
        if self.cur_token_index == self.tokens.len() {
            panic!("Error: Out of range");
        }
        &self.tokens[self.cur_token_index]
    }

    /// move_token function:
    ///
    /// increase current token index and
    /// move the next token
    fn move_token(&mut self) {
        self.cur_token_index += 1;
    }

    /// factor:
    ///
    /// ident | number | "(" expression ")" | funcCall
    ///
    /// identifies the type and value of the current token
    /// and generate the instruction based on the information
    ///
    /// case 1: (
    ///     call expression to identify inside of the parantehsis
    ///     must have the closed paranethesis at the end (after expression)
    ///
    /// case 2: Number
    ///     identifies if the block0 (const value container) already has the instruction for the number
    ///
    ///     if not: generate new instruction for the number and return the instruction number
    ///
    ///     if so: return the existing instruction number
    ///
    /// case 3: Ident (Variable)
    ///     identifies if the variable is initialized already by looking at current block's table
    ///  
    ///     if not: warn that it is not initialized and return 0 which means the variable is 0
    ///  
    ///     if so: return the existing instruction number
    ///
    /// case 4: Function call
    ///     Identify if the function is user-defined
    ///   
    ///     if not: move onto the function call function (func_call)
    ///   
    ///     if so: check if it's a void or not. It must be non-void function
    ///            and move onto the function call (func_call)
    fn factor(&mut self) -> i32 {
        self.move_token();
        println!("Factor's current Token: {}", self.current());
        let factor_token = self.current().clone();
        match factor_token {
            // ( )
            Token::Symbol(Symbol::OpenParen) => {
                let inst_num = self.expression();
                if self.current() != &Token::Symbol(Symbol::CloseParen) {
                    panic!("Error, missing closed parentheses: {}", self.current());
                }
                self.move_token();
                return inst_num;
            }
            // number
            Token::Number(num) => {
                let op = Operator::Const(num);
                self.move_token();
                if let Some(i_num) = self.get_bb0(&op) {
                    return i_num * (-1);
                }

                let inst_num = self.add_const_to_bb0(op);
                println!("Num's Token: {}", inst_num);
                return inst_num * (-1);
            }
            // variable
            Token::Ident(UserDefined(var)) => {
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
            // function call
            Token::Call => {
                println!("Factor is Call");
                self.move_token();
                match self.current() {
                    // user-defined function
                    Token::Ident(Ident::UserDefined(func)) => {
                        if self.void_funcs.contains(func) {
                            panic!("Error: Void function detected in assignment");
                        }
                        let inst_num = self.func_call();
                        return inst_num;
                    }
                    // predefined function
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

    /// term:
    ///
    /// factor { ("*" | "/") factor }
    fn term(&mut self) -> i32 {
        let mut x: i32 = self.factor();
        println!("Current token after Factor: {}", self.current());
        if self.current() != &Token::Op(MUL) && self.current() != &Token::Op(DIV) {
            return x;
        }
        let mut return_val = x;
        let mut calc;
        while self.current() == &Token::Op(MUL) || self.current() == &Token::Op(DIV) {
            calc = self.current().clone();
            let y = self.factor();
            let op = match calc {
                Token::Op(MUL) => Operator::Mul(x, y),
                Token::Op(DIV) => Operator::Div(x, y),
                _ => panic!("Error, Invalid Div or Mul"),
            };
            return_val = self.add_inst_to_tail(op.clone());
            // self.inst_storage.add_muls(op.clone(), expected_num); 
            // self.inst_storage.add_divs(op.clone(), expected_num); 
            
            // if let Some(div) = self.inst_storage.add_divs(op.clone(), expected_num) {
            //     return_val = if div == expected_num {
            //         let inst_num = self.add_inst_to_tail(op.clone());
            //         self.insts.insert(op, inst_num);
            //         inst_num
            //     } else {
            //         div
            //     };
            // }
            x = return_val;
        }
        return_val
    }

    /// expression:
    ///
    /// term { ("+" | "-") term }
    fn expression(&mut self) -> i32 {
        let mut x = self.term();
        if self.current() != &Token::Op(ADD) && self.current() != &Token::Op(SUB) {
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
            return_val = self.add_inst_to_tail(op.clone());
            self.inst_storage.add_adds(op.clone(), return_val); 
            self.inst_storage.add_subs(op, return_val);

            // if let Some(add) = self.inst_storage.add_adds(op.clone(), expected_num) {
            //     println!("Add return Val: {}", add);
            //     return_val = if add == expected_num {
            //         let inst_num = self.add_inst_to_tail(op.clone());
            //         self.insts.insert(op.clone(), inst_num);
            //         println!("Inst Num for add optimization: {}", inst_num);
            //         inst_num
            //     } else {
            //         println!("Inst Num for no optimization add: {}", add);
            //         add
            //     };
            // } else if let Some(sub) = self.inst_storage.add_subs(op.clone(), expected_num) {
            //     return_val = if sub == expected_num {
            //         let inst_num = self.add_inst_to_tail(op.clone());
            //         self.insts.insert(op, inst_num);
            //         println!("Inst Num for sub optimization: {}", inst_num);

            //         inst_num
            //     } else {
            //         println!("Inst Num for no optimization sub: {}", sub);
            //         sub
            //     };
            // }
            x = return_val;
        }
        return_val
    }

    /// relation:
    ///
    /// expression relOp expression
    fn relation(&mut self) -> i32 {
        println!("Current Token in Relation: {}", self.current());
        let lhs = self.expression();
        print!("Current Token after LHS in relation: {}", self.current());
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

    /// assignment:
    ///
    /// "let" ident "<-"  expression
    fn assignment(&mut self) -> i32 {
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

    /// func_call
    ///
    /// "call" ident [ "(" [expression {"," expression}] ")"]
    fn func_call(&mut self) -> i32 {
        match self.current() {
            // InputNum(): no parameter
            Token::Ident(Ident::InputNum) => {
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
            // OutputNewLine(): no parameter
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
            // OutputNum(x): one parameter
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
            // User-defined function
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
                        let param1 = self.expression();
                        let set_param1 = Operator::SetPar1(param1);
                        let inst_num = self.add_inst_to_tail(set_param1.clone());
                        self.insts.insert(set_param1, inst_num);
                    }

                    if self.current() == &Token::Symbol(Symbol::Comma) {
                        if params >= 2 {
                            let param2 = self.expression();
                            let set_param2 = Operator::SetPar2(param2);
                            let inst_num = self.add_inst_to_tail(set_param2.clone());
                            self.insts.insert(set_param2, inst_num);
                        } else {
                            panic!("Error: Parameter # mismatching");
                        }
                    }
                    if self.current() == &Token::Symbol(Symbol::Comma) {
                        if params == 3 {
                            let param3 = self.expression();
                            let set_param3 = Operator::SetPar3(param3);
                            let inst_num = self.add_inst_to_tail(set_param3.clone());
                            self.insts.insert(set_param3, inst_num);
                        } else {
                            panic!("Error: Parameter # mismatching");
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
                    self.move_token();
                    self.total_inst
                } else {
                    panic!("Error: Missing Opened Paranthesis");
                }
            }
            _ => panic!("Error: Invalid funcCall format"),
        }
    }

    /// if_statement
    ///
    /// "if" relation "then" statSequence ["else" statSequence] "fi"
    fn if_statement(&mut self) {
        let before_table = self.get_current_table();
        let if_key = self.cur_block_num;

        // then block
        let then_key = self.create_new_block("then".to_string(), before_table.clone());
        self.set_dom(if_key, then_key);

        // fi block
        let fi_key = self.create_new_block("fi".to_string(), before_table.clone());
        let fi_block_name = "fi_".to_string() + &fi_key.to_string();
        self.set_dom(if_key, fi_key);

        // conditional statement
        let cond = self.relation();

        if self.current() != &Token::Then {
            panic!("Error: Invalid If Statement Format, Missing \"then\"");
        }

        // then block check
        self.switch_block(then_key);
        self.stat_sequence();
        self.connect(if_key, then_key);

        // generate phis for then
        let mut phis;
        self.connect(self.cur_block_num, fi_key);
        phis = self.generate_phi(self.cur_block_num, if_key);

        // else block check
        if self.current() == &Token::Else {
            // since else is optional
            println!("BEFORE TABLE AT ELSE: {:?}\n\n\n\n", before_table);
            let else_key = self.create_new_block("else".to_string(), before_table);
            self.connect(if_key, else_key);
            self.set_dom(if_key, else_key);
            self.switch_block(else_key);
            self.stat_sequence();

            self.connect(self.cur_block_num, fi_key);

            // phi function check
            println!("block num after else: {}", self.cur_block_num);
            phis = self.update_phi(phis, self.cur_block_num);
            println!(
                "\n\n\nAFTER TABLE at UPDATE PHI: {:?}\n\n\n",
                self.get_table_from_block(self.cur_block_num)
            );

            // add branch call instruction
            self.add_inst_to_tail(Operator::Bra(fi_block_name));
            self.switch_block(else_key);
            let mut loc_rhs = (cond, "".to_string());
            if let Some(else_name) = self.get_current_block_name() {
                loc_rhs.1 = else_name;
            }
            // Update condition instruction
            self.update_rel_op(if_key, loc_rhs);
        } else {
            // no else block
            self.switch_block(fi_key);
            let loc_rhs = if let Some(block_name) = self.get_current_block_name() {
                (cond, block_name)
            } else {
                (cond, "".to_string())
            };
            // update condition instruction
            self.update_rel_op(if_key, loc_rhs);
            self.connect(if_key, fi_key);
        }

        if self.current() != &Token::Fi {
            panic!("Error: Invalid If Statement Format, Missing \"fi\"");
        }

        // add phi function instruction(s)
        self.switch_block(fi_key);
        for (var, phi) in phis {
            let phi_op = Operator::Phi(phi.0, phi.1);
            let inst_num = self.add_inst_to_tail(phi_op);
            self.update_table(var, inst_num);
        }
        self.move_token();
    }

    /// while_statement
    /// "while" relaton "do" StatSequence "od"
    fn while_statement(&mut self) {
        let before_table = self.get_current_table();

        // while block
        let while_key = self.create_new_block("while".to_string(), before_table.clone());
        let while_block_name = "while_".to_string() + &while_key.to_string();

        // do block
        let do_key = self.create_new_block("do".to_string(), before_table.clone());

        // Edges
        self.connect(self.cur_block_num, while_key);
        self.connect(while_key, do_key);

        // Doms
        self.set_dom(self.cur_block_num, while_key);
        self.set_dom(while_key, do_key);

        // conditional statement
        self.switch_block(while_key);
        let cond = self.relation();
        if self.current() != &Token::Do {
            panic!("Error: Invalid While Statement Format, Missing \"do\"");
        }

        // do check
        self.switch_block(do_key);
        self.stat_sequence();
        self.add_inst_to_tail(Operator::Bra(while_block_name));

        // generate phi function(s)
        let phis;
        self.connect(self.cur_block_num, while_key);
        phis = self.generate_phi(while_key, self.cur_block_num);
        if self.current() != &Token::Od {
            panic!("Error: Invalid While Statement Format, Missing \"od\"");
        }

        // format phi function information
        // add phi function instructions to while block
        self.switch_block(while_key);
        let mut ori_to_new = HashMap::new();
        let mut var_to_phi = HashMap::new();
        let mut phi_insts = Vec::new();
        for (var, phi) in phis {
            self.total_inst += 1;
            let phi_op = Operator::Phi(phi.0, phi.1);
            println!("phi_op: {}", phi_op);
            let phi_inst = Inst::new(self.total_inst, phi_op);
            ori_to_new.insert(phi.0, self.total_inst);
            var_to_phi.insert(var, self.total_inst);
            phi_insts.push(phi_inst);
        }
        // update instruction based on phi
        self.update_by_phi(ori_to_new, while_key);
        self.update_table_with_insts(var_to_phi, while_key);
        // add phi functions on while block
        for inst in phi_insts {
            let cur_block = if let Some(b) = self.blocks.get(&self.cur_block_num) {
                b
            } else {
                panic!("Error: No Block Found at {}", self.cur_block_num)
            };
            cur_block.borrow_mut().push_head(inst);
        }

        // od block
        let od_key = self.create_new_block("od".to_string(), self.get_table_from_block(while_key));

        self.connect(while_key, od_key);
        self.set_dom(while_key, od_key);

        // update condition instruction
        self.switch_block(od_key);
        if let Some(block_name) = self.get_current_block_name() {
            self.update_rel_op(while_key, (cond, block_name));
        }
        self.move_token();
    }

    /// return_statement
    ///
    /// "return" [ expression ]
    fn return_statement(&mut self) {
        let return_var = self.expression();
        let return_op = Operator::Ret(Some(return_var));
        self.add_inst_to_tail(return_op);
    }

    /// statement
    ///
    /// assignment | funcCall | ifStatement | whileStatement | returnStatement
    fn statement(&mut self) {
        self.move_token();
        match &self.current() {
            // assignment
            Token::Let => {
                self.assignment();
            }

            // function call
            Token::Call => {
                self.move_token();
                self.func_call();
            }

            // if statement
            Token::If => {
                self.if_statement();
            }

            // while statement
            Token::While => {
                self.while_statement();
            }

            // return statement
            Token::Return => {
                self.return_statement();
            }

            // other acceptable cases (to skip)
            Token::Symbol(Symbol::CloseBrace) | Token::Fi | Token::Else | Token::Od => (),
            _ => panic!("Error: Invalid Statement format: {}", self.current()),
        };
    }

    /// stat_sequence
    ///
    /// statement { ";" statement } [";"]
    ///
    /// I decided the design choice that all statement must have ";"
    fn stat_sequence(&mut self) {
        self.statement();
        while self.current() == &Token::Symbol(Symbol::SemiColon) {
            self.statement();
        }
    }

    /// var_decl
    ///
    /// "var" ident {"," ident} ";"
    fn var_decl(&mut self) {
        self.move_token();
        match self.current() {
            Token::Ident(UserDefined(var)) => self.vars.insert(var.to_string(), None),
            _ => return,
        };
        self.move_token();
        while self.current() == &Token::Symbol(Symbol::Comma) {
            self.move_token();
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
    }

    /// func_decl
    ///
    /// ["void"] "function" ident formalParam ";" funcBody ";"
    fn func_decl(&mut self) {
        // prevent from corrupting the variable hash map
        let global_vars = self.vars.clone();

        // void status
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

        // block0 for the user-defined function
        let func_block0_key = self.create_new_block(func_name.clone() + &"0", HashMap::new());
        self.switch_block0(func_block0_key);

        // initial block for user-defined function
        let func_key: usize = self.create_new_block(func_name.clone(), HashMap::new());
        self.switch_block(func_key);

        self.connect(func_block0_key, func_key);

        // parameters & function body
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

        // void check
        if is_void {
            self.add_inst_to_tail(Operator::Ret(None));
            self.void_funcs.push(func_name);
        }

        // back up
        self.vars = global_vars;
        self.insts = HashMap::new();
        self.switch_block(func_key);
        if let Some(name) = self.get_current_name() {
            self.funcs.insert(name, params);
        }
        self.switch_block0(0);
    }

    /// formal_param
    ///
    ///  "( [ident {"," ident}] ")"
    ///
    /// return # of parameters
    fn formal_param(&mut self) -> usize {
        // may need to change (if max # of parameters is not 3)
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

    /// func_body
    ///
    /// [varDecl] "{" [statSequence] "}"
    fn func_body(&mut self) {
        // variable for the user-defined function
        self.var_decl();
        self.move_token();

        // function logic
        if self.current() == &Token::Symbol(Symbol::OpenBrace) {
            self.stat_sequence();
            if self.current() != &Token::Symbol(Symbol::CloseBrace) {
                panic!("Error: FuncBody missing Closed Brace: \"}}\"");
            }
        }
        self.move_token();
    }

    /// computation
    ///
    /// "main" [varDecl] {funcDecl} "{" statSequence "}" "."
    ///
    fn computation(&mut self) {
        // Main
        if self.current() != &Token::Main {
            panic!("Error: Missing Main keyword");
        }
        self.move_token();

        // Global Variables
        if self.current() == &Token::Var {
            self.var_decl();
            self.move_token();
        }
        let table = self.vars.clone();
        println!("table at computation: {:?}", table);

        // User-defined functions
        while self.current() == &Token::Void || self.current() == &Token::Function {
            self.func_decl();
            self.move_token();
        }

        // initial block for main function
        let main_key = self.create_new_block("main".to_string(), table);
        self.switch_block(self.total_block);

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
        self.add_inst_to_tail(Operator::End);
        self.connect(0, main_key);
    }

    /// helper functions to add instruction to the current block
    ///
    ///

    /// create_new_block
    ///
    /// name: String
    ///
    /// table: HashMap<String, Option<i32>>
    ///
    /// create new block and return its key
    fn create_new_block(&mut self, name: String, table: HashMap<String, Option<i32>>) -> usize {
        self.total_block += 1;
        let block_key = self.total_block;
        let block = RefCell::new(Block::new(self.total_block, name, table));
        self.blocks.insert(block_key, block);
        block_key
    }

    /// add_const_to_bb0
    ///
    /// op: Operator
    ///
    /// add constant variable to the current block 0
    fn add_const_to_bb0(&mut self, op: Operator) -> i32 {
        self.total_inst += 1;
        let inst_num = self.total_inst;
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

    /// add_inst_to_tail
    ///
    /// op: Operator
    ///
    /// add new instruction to the current block (Tail)
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

    /// add_inst_to_head
    ///
    /// op: Operator
    ///
    /// add new instruction to the current block (Head)
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

    /// swith_block
    ///
    /// key: usize
    ///
    /// switch the current block key
    fn switch_block(&mut self, key: usize) {
        self.cur_block_num = key;
    }

    /// switch_block0
    ///
    /// key: usize
    ///
    /// switch the current block 0 key
    fn switch_block0(&mut self, key: usize) {
        self.block0_num = key;
    }

    /// show_vars
    ///
    /// print vars (variable containers)
    fn show_vars(&self) {
        println!("Vars: {:?}", self.vars);
    }

    /// show_insts
    ///
    /// print insts (instruction containers for function
    ///
    /// Used to see all the instructions in the main function at the end
    fn show_insts(&self) {
        println!("Insts: {:?}", self.insts);
    }

    /// show_inst_storage
    ///
    /// show instruction storage for function
    ///
    /// Used to see the information of instruction storage of the main function at the end
    fn show_inst_storage(&self) {
        println!("Inst Storage: {:?}", self.inst_storage);
    }

    /// show_blocks
    ///
    /// show blocks
    ///
    /// Used to see all the blocks at the end (main + user-defined functions)
    fn show_blocks(&self) {
        // println!("Block0: ");
        println!("Block#: {}", self.blocks.len());
        println!("Blocks:");
        for i in 0..self.blocks.len() {
            if let Some(b) = self.blocks.get(&i) {
                println!("{:?}", b.borrow());
            }
        }
    }

    /// update_table
    ///
    /// var: String
    ///
    /// inst_num: i32
    ///
    /// update table information of the current block
    fn update_table(&mut self, var: String, inst_num: i32) {
        if let Some(b) = self.blocks.get(&self.cur_block_num) {
            b.borrow_mut().update_table(var.clone(), inst_num);
            self.vars.insert(var, Some(inst_num));
            println!("\n\n\nVAR AFTER UPDATE TABLE: {:?} \n\n\n", self.vars);
        }
    }

    /// connect
    ///
    /// front_num: usize
    ///
    /// back_num: usize
    ///
    /// connect blocks (front -> back)
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
    }

    /// set_dom
    ///
    /// x_num: usize
    ///
    /// y_num: usize
    ///
    /// set dominator x dom y
    fn set_dom(&mut self, x_num: usize, y_num: usize) {
        let x_block = if let Some(x) = self.blocks.get(&x_num) {
            x
        } else {
            panic!("Error: No X Block Found at {} in set_dom function", x_num,)
        };
        x_block.borrow_mut().add_dom(y_num as usize);
    }

    /// generate_phi
    ///
    /// pre_key: usize
    ///
    /// now_key: usize
    ///
    /// generate phi functions comparing two blocks (pre & now)
    ///
    /// return the information containing variable, previous instruction and current instruction number
    fn generate_phi(&mut self, now_key: usize, pre_key: usize) -> HashMap<String, (i32, i32)> {
        let mut phis = HashMap::new();
        if let (Some(pre), Some(now)) = (self.blocks.get(&pre_key), self.blocks.get(&now_key)) {
            println!(
                "generate phi's pre and now: {:?} : {:?}",
                pre.borrow().get_table(),
                now.borrow().get_table()
            );
            let vars = pre.borrow().compare_table(now);

            for (var, phi_insts) in vars {
                phis.insert(var, phi_insts);
            }
            println!("Phi after generation: {:?}", phis);
        }
        return phis;
    }

    fn update_phi(
        &mut self,
        mut phis: HashMap<String, (i32, i32)>,
        new_key: usize,
    ) -> HashMap<String, (i32, i32)> {
        println!("PHI insts befre update_phi: {:?}", phis);
        if let Some(b) = self.blocks.get(&new_key) {
            let b_table = b.borrow().get_table();
            for (var, insts) in phis.clone() {
                if let Some(Some(i)) = b_table.get(&var) {
                    println!("Current Inst: {:?}", insts);
                    let new_insts = (insts.0, *i);
                    println!("New Insts: {:?}", new_insts);
                    phis.insert(var, new_insts);
                }
            }
        }
        phis
    }

    /// update_by_phi
    ///
    /// phis: HashMap<i32, i32>
    ///
    /// start_key: usize
    ///
    /// update instructions based on phi function info
    fn update_by_phi(&mut self, phis: HashMap<i32, i32>, start_key: usize) {
        println!("\nUPDATE_BY_PHI:\n");
        for i in start_key..self.total_block + 1 {
            if let Some(block) = self.blocks.get(&i) {
                println!(
                    "Block name for update_by_phi: {}",
                    block.borrow().get_block_name()
                );
                block.borrow_mut().update_inst(&phis);
            }
        }
    }

    /// update_rel_op
    ///
    /// cond_key: usize
    ///
    /// loc_rhs: (i32, String)
    ///
    /// update rel_op instruction
    fn update_rel_op(&mut self, cond_key: usize, loc_rhs: (i32, String)) {
        if let Some(bb) = self.blocks.get(&cond_key) {
            bb.borrow_mut().fill_in_none(loc_rhs);
        }
    }

    /// update_table_with_insts
    ///
    /// var_to_phi: HashMap<String, i32>
    ///
    /// start_key: usize
    ///
    /// update table based on the phi function info
    fn update_table_with_insts(&mut self, var_to_phi: HashMap<String, i32>, start_key: usize) {
        for i in start_key..self.total_block + 1 {
            if let Some(block) = self.blocks.get(&i) {
                block.borrow_mut().update_table_with_insts(&var_to_phi);
            }
        }
    }

    /// get_table_from_block
    ///
    /// key: usize
    ///
    /// return the table of the block
    fn get_table_from_block(&self, key: usize) -> HashMap<String, Option<i32>> {
        if let Some(block) = self.blocks.get(&key) {
            return block.borrow().get_table();
        }
        HashMap::new()
    }

    /// get_current_name
    ///
    /// return current block's name
    fn get_current_name(&self) -> Option<String> {
        if let Some(block) = self.blocks.get(&self.cur_block_num) {
            return Some(block.borrow().get_func_name());
        }
        None
    }

    /// get_current_block_name
    ///
    /// return current block's block name (name + number)
    fn get_current_block_name(&self) -> Option<String> {
        if let Some(block) = self.blocks.get(&self.cur_block_num) {
            return Some(block.borrow().get_block_name());
        }
        None
    }

    fn get_current_table(&self) -> HashMap<String, Option<i32>> {
        if let Some(block) = self.blocks.get(&self.cur_block_num) {
            return block.borrow().get_table();
        }
        panic!("No Table for Current Block");
    }

    /// is_void
    ///
    /// key: usize
    ///
    /// check if the block (function) type is void or non-void
    fn is_void(&self, key: usize) -> bool {
        if let Some(block) = self.blocks.get(&key) {
            if matches!(block.borrow().get_tail_op(), Operator::Ret(_)) {
                return false;
            }
        }
        true
    }

    fn set_postive(&mut self) {
        for bb in self.blocks.clone(){
            bb.1.borrow_mut().set_no_const_sign();
        }
    }

    /// get_bb0
    ///
    /// op: &Operator
    ///
    /// return the operator's instruction number in block 0
    fn get_bb0(&self, op: &Operator) -> Option<i32> {
        if let Some(bb0) = self.blocks.get(&self.block0_num) {
            return bb0.borrow().get_inst(op);
        }
        None
    }

    /// visualize_ir
    ///
    /// visualize IR for a graph visualizer
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
}

/// Tests:
/// TODO: the cases:
///     Bug for if statement (unitialized variable)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_propa_test1() {
        let input = String::from(
            "main
        var a, b, c, d; {
            let a <- 1;
            let b <- a;
            let c <- a + 1; 
            let d <- b + 1;
            let c <- 1 + 1;
            let a <- 2;
            let d <- a + 1;
    }.",
        );
        // 1 + 1 = a + 1 = b + 1 before a <- 2;
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }

    #[test]
    fn copy_propa_test2() {
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
    fn opt_arge_sim_cse_test() {
        let input = String::from(
            "main
        var a, b, c; {
            let b <- a + 1;
            let b <- 1- 1;
            let b <- a * 2; 
            let a <- 2;
            let b <- 2* a;
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
    fn copy_propa_test3() {
        let input = String::from(
            "main
        var a, b, c; {
            let a <- 1; 
            let b <- 1; 
            let c <- a + b;
            if a == b then 
                let c <- 1 + 1;
                let b <- 3; 
                let c <- b - 1;
            fi;
            let b <- a + 1; 
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
    fn calc_test2() {
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
    fn calc_test3() {
        let input = String::from(
            "main
        var a, b, c; {
            let a <- 1 / (3 * 1) * 3;
            let b <- 2 / (3 * 1) * 3 - 1;
            let c <- call InputNum(); 
            let b <- c + call InputNum() + 2;
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
            let a <- 1 * 2 * 3 * 4; 
            let b <- 1 - 2 - 3 - 4; 
            let a <- 1 + 2 + 3 + 4; 
            let b <- 1 / 2 / 3 / 4; 
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
        var a, b; {

            if 1 == 2 then
                let b <- 2;

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
    let a <- a + 3;
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
                if 1 == 3 then 
                    let a <- a - 1; 
                fi;
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
                    let a <- a - 1;
                else 
                    let a <- 1 + 4;
                fi;
            else 
                if 1 == 3 then 
                    let a <- a - 1; 
                fi;
            fi;
        let a <- 2;
    }.",
        );
        // bug: no phi for a <- a; else a <- a - 1;
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
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
                    let a <- a + 3;
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
    fn while_test1() {
        let input = String::from(
            "main
        var a, b; {
        let a <- 1;
            while 1 == a do
                let b <- 2;
                let a <- b + 1;
                od
    }.",
        );
        // bug: phi function for uninitialized variable
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
    }
    #[test]
    fn while_test2() {
        let input = String::from(
            "main
        var a, b; {
        let a <- 1 + b;
            while 1 == a do
                let a <- a + 2;
            od
    }.",
        );
        // bug: phi function for uninitialized variable
        let mut parse = Parser::new(input);
        parse.computation();
        parse.show_vars();
        parse.show_insts();
        parse.show_blocks();
        parse.visualize_ir();
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
        parse.visualize_ir();
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
    fn while_while_test() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
        let b <- 2;
        while 1 == a do
                
            while 1 == a do 
                let a <- a + 1;
            od;
            let b <- 3;
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
    fn while_if_test1() {
        let input = String::from(
            "main
        var a; {
        let a <- 1;
        while 1 == 2 do
            if 1 == a then
                let a <- a - 1;
            else let a <- a + 3;
            fi;
        od;
        let a <- a + 1;
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
        let c <- a - 1 - 3; 
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
    call OutputNum(call sum(a, b, c));
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

    // ret value for void function
    #[test]
    fn void_test() {
        let input = String::from(
            "main
    var a, b, e;

    void function sum(a); var c; {
        let c <- a + d;
        let a <- a + d;
    };

    {
    let a <- 1;
    let b <- 2; 
    let e <- 3;
    call sum(a);
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
    fn long_test() {
        let input = String::from(
            "main
var a, b, c, d;
{
    let a <- call InputNum();
    let b <- 0;
    let c <- 1;
    let d <- 0;
    while a > 0 do
        if a < 10 then
            let b <- b + c;
            let c <- c + 1;
            while d < 3 do
                let b <- b + a;
                let d <- d + 1;
            od;
            let a <- a - 1;
        else
            let c <- c * 2;
            let d <- 0;
        fi;
        let a <- a - 1;
    od;
    call OutputNum(a);
    call OutputNum(b);
    call OutputNum(c);
    call OutputNum(d);
    call OutputNewLine();
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

    #[test]
    fn github_test1() {
        let input = String::from(
            "main
var zoink67, legalegends;

{
let zoink67 <- 1;
let legalegends <- 2;

if 1 < 2 then
    let zoink67 <- 1 + 1 + 1 + 1;
else
    let zoink67 <- 67 + 67;
fi;

if 1 == 2 then
    let zoink67 <- 1 - 1;
fi;

let zoink67 <- 67;

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
