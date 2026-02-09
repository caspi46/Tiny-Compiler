use crate::frontend::fsm::{
    token::{Op, RelOp, Symbol, Token},
    tokenizer::Tokenizer,
};

use std::collections::HashMap;
use std::mem::discriment;

pub struct Parser {
    tokens: Vec<Token>,
    blocks: Vec<Block>,
    vars: HashMap<Token::Ident, i32>,
    cur_token: Toekn,
    cur_i: usize,
    cur_block: Option<Block>,
}

impl Parser {
    fn new(input: String) -> Self {
        let tokenizer = Tokenizer::new(input);
        tokenizer.generate_token();
        if tokens.len() < 0 {
            panic!("No User Input/Toeken");
        }
        Self {
            tokens: tokenizer.get_token(),
            blocks: Vec::new(),
            vars: HashMap<Token::Ident, i32>::new(),
            cur_token: tokens[0],
            cur_i: 0,
            cur_block: None,
        }
    }
    fn get_cur_token(self) -> Token {
        cur_token
    }

    fn inc_cur_i(&mut self) {
        cur_i += 1;
        self.cur_token = self.tokens[cur_i];
    }

    fn factor(&self) {
        // ident | number | "(" expression ")" | funcCall
    }
    fn term(&self) {
        // factor { ("*" | "/") factor }
    }
    fn expression(&self) {
        // term { ("+" | "-") terrm }
    }
    fn relation(&self) {
        // expression relOp expression
    }
    fn assignment(&self) {
        // "let" ident "<-"  expression
    }
    fn funcCall(&self) {
        // "call" ident [ "(" [expression {"," expression}] ")"]
    }
    fn ifStatement(&self) {
        // "if" relation "then" statSequence ["else" statSequence] "fi"
        // new block: if, then, else, fi
    }
    fn whileStatement(&self) {
        // "while" relaton "do" StatSequence "od"
        // new block: while, then
    }
    fn returnStatement(&self) {
        // assignment | funcCall | ifStatement | whileStatement | returnStatement
    }
    fn statement(&self) {
        // statement {";" statement } [";"]
    }
    fn statSequence(&self) {
        // statement {";" statement } [";"]
        self.inc_cur_i();
    }
    fn varDecl(&self) {
        // "var" ident {"," ident} ";"
        self.inc_cur_i();
        if discriment(self.cur_token) == discriment(Token::Ident("")) {
            // self.formalParam();
            vars.insert(self.cur_token, 0); // 0 for default
        } else {
            panic!("Error: Missing Identifier");
        }
        self.inc_cur_i();
        while self.cur_token == Token::Symbol(Symbol::Comma) {
            self.inc_cur_i();
            // check if the token type is identifier (Ident)
            if discriment(self.cur_token) == discriment(Token::Ident("")) {
                vars.insert(self.cur_token, 0);
            }
        }
        if self.cur_token != Token::Symbol(Symbol::SemiColon) {
            panic!("Error: Missing SemiColon: \";\"");
        }
    }
    fn funcDecl(&self) {
        // ["void"] "function" ident formalParam ";" funcBody ";"
        // new block
        self.inc_cur_i();
        if self.cur_token == Token::Function {
            self.inc_cur_i();
            if discriment(self.cur_token) == discriment(Token::Ident("")) {}
        }
    }
    fn formalParam(&self) {
        // "( [ident {"," ident}] ")"
        self.inc_cur_i();
        if self.cur_token == Token::Symbol(Symbol::OpenParen) {
            self.inc_cur_i();
            if discriment(self.cur_token) == discriment(Token::Ident("")) {
                // do something with identifier
            }
            self.inc_cur_i();
            while self.cur_token == Token::Symbol(Symbol::SemiColon) {
                self.inc_cur_i();
                if discriment(self.cur_token) == discriment(Token::Ident("")) {
                    // do something with identifier
                }
                self.inc_cur_i();
            }
            if self.cur_token == Token::Symbol(Symbol::CloseParen) {
                self.inc_cur_i();
            } else {
                panic!("Error: Missing Closed Parenthesis: \")\"");
            }
        }
    }
    fn funcBody(&self) {
        // [varDecl] "{" [statSequence] "}"
    }
    fn computation(&self) {
        // "main" [varDecl] {funcDecl} "{" statSequence "}" "."
        while self.cur_token == Token::Main {
            self.inc_cur_i();
            if self.cur_token == Token::Symbol(Symbol::Period) {
                break;
            }
            if self.cur_token == Token::Var {
                // Var Decl Path
                self.varDecl();
            }
            if self.cur_token == Token::Void {
                // Func Decl Path
                self.funcDecl();
            }
            if self.cur_token == Token::Symbol(Symbol::OpenBrace) {
                self.statSequence(); // Stat Sequence Path
                if self.cur_token == Token::Symbol(Symbol::CloseBrace) {
                    self.inc_cur_i();
                } else {
                    panic!("Error: Missing Closed Brace: \"}\" ");
                }
            }
        }
        if self.cur_token != Token::Symbol(Symbol::Period) {
            panic!("Eror: Missing Period: \".\"");
        }
    }
}
