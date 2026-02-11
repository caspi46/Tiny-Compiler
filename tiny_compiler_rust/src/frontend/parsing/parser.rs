use crate::frontend::fsm::{
    token::{
        Op::ADD, Op::DIV, Op::MUL, Op::SUB, RelOp::EQ, RelOp::GE, RelOp::GT, RelOp::LE, RelOp::LT,
        RelOp::NE, Symbol, Token, Token::Ident,
    },
    tokenizer::Tokenizer,
};
use crate::frontend::operators::Inst;

use std::collections::{HashMap, LinkedList};

pub struct Parser {
    tokens: Vec<Token>,
    blocks: LinkedList<LinkedList<Inst>>,
    vars: HashMap<Token, i32>,
    cur_token_index: usize,
    cur_block: Option<LinkedList<Inst>>,
}

impl Parser {
    fn new(input: String) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.generate_token();
        let tokens = tokenizer.get_tokens();
        // if tokens.len() < 0 {
        //     panic!("No User Input/Toeken");
        // }
        Self {
            tokens,
            blocks: LinkedList::new(),
            vars: HashMap::new(),
            cur_token_index: 0,
            cur_block: None,
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

    fn factor(&self) {
        // ident | number | "(" expression ")" | funcCall
        if self.current() == &Token::Symbol(Symbol::OpenParen) {
            if self.current() == &Token::Symbol(Symbol::CloseParen) {
            } else {
                panic!("Error, missing closed parentheses, \")\"");
            }
        } else if matches!(self.current(), &Token::Number(_)) {
        }
    }
    fn term(&mut self) {
        // factor { ("*" | "/") factor }
        self.factor();
        self.move_token();
        while self.current() == &Token::Op(MUL) {
            self.move_token();
            self.factor();
        }
        while self.current() == &Token::Op(DIV) {
            self.move_token();
            self.factor();
        }
    }
    fn expression(&mut self) {
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
    }
    fn relation(&mut self) {
        // expression relOp expression
        self.expression();
        self.move_token();
        match self.current() {
            &Token::RelOp(EQ) => (),
            &Token::RelOp(NE) => (),
            &Token::RelOp(GT) => (),
            &Token::RelOp(LT) => (),
            &Token::RelOp(GE) => (),
            &Token::RelOp(LE) => (),
            _ => {
                panic!("Error, missing relOp: ==, !=, >, <, >=, <=");
            }
        }
        self.move_token();
        self.expression();
    }
    fn assignment(&mut self) {
        // "let" ident "<-"  expression
        self.move_token();
        if self.current() != &Token::Ident("".to_string()) {
            panic!("Error: Invalid Assignment Format, Missing Identifier");
        }
        let var = self.current().clone();
        self.move_token();
        if self.current() != &Token::Symbol(Symbol::Init) {
            panic!("Error: Invalid Assignment Format, Missing \"<-\"");
        }
        self.move_token();
        self.expression(); // TODO: Return value (Identify the value)
        self.vars.insert(var, -1); // just for now
    }
    fn funcCall(&mut self) {
        // "call" ident [ "(" [expression {"," expression}] ")"]
    }
    fn ifStatement(&mut self) {
        // "if" relation "then" statSequence ["else" statSequence] "fi"
        // new block: if, then, else, fi
        self.move_token();
        self.relation(); // TODO: Return value 
        self.move_token();
        if self.current() != &Token::Then {
            panic!("Error: Invalid If Statement Format, Missing \"then\"");
        }
        self.move_token();
        self.statSequence();
        self.move_token();
        if self.current() != &Token::Else {
            panic!("Error: Invalid If Statement Format, Missing \"else\"");
        }
        self.move_token();
        self.statSequence();
        self.move_token();
        if self.current() != &Token::Fi {
            panic!("Error: Invalid If Statement Format, Missing \"fi\"");
        }
    }
    fn whileStatement(&self) {
        // "while" relaton "do" StatSequence "od"
        // new block: while, then
    }
    fn returnStatement(&self) {
        // assignment | funcCall | ifStatement | whileStatement | returnStatement
    }
    fn statement(&mut self) {
        // statement {";" statement } [";"]
        self.move_token();
        match self.current() {
            &Token::Let => self.assignment(),
            &Token::Call => self.funcCall(),
            &Token::If => self.ifStatement(),
            &Token::While => self.whileStatement(),
            &Token::Return => self.returnStatement(),
            _ => panic!("Error: Invalid Statement format"),
        }
    }
    fn statSequence(&mut self) {
        // statement {";" statement } [";"]
        self.move_token();
        self.statement();
        self.move_token();
        while self.current() == &Token::Symbol(Symbol::SemiColon) {
            self.move_token();
            self.statement();
        }
    }
    fn varDecl(&mut self) {
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
    fn funcDecl(&mut self) {
        // ["void"] "function" ident formalParam ";" funcBody ";"
        // new block
        self.move_token();
        if self.current() == &Token::Function {
            self.move_token();
            if matches!(self.current(), &Token::Ident(_)) {}
        }
    }
    fn formalParam(&mut self) {
        // "( [ident {"," ident}] ")"
        self.move_token();
        if self.current() == &Token::Symbol(Symbol::OpenParen) {
            self.move_token();
            if matches!(self.current(), &Token::Ident(_)) {
                // do something with identifier
            }
            self.move_token();
            while self.current() == &Token::Symbol(Symbol::SemiColon) {
                self.move_token();
                if self.current() == &Token::Ident("".to_string()) {
                    // do something with identifier
                }
                self.move_token();
            }
            if self.current() == &Token::Symbol(Symbol::CloseParen) {
                self.move_token();
            } else {
                panic!("Error: Missing Closed Parenthesis: \")\"");
            }
        }
    }
    fn funcBody(&mut self) {
        // [varDecl] "{" [statSequence] "}"
        self.varDecl();
    }
    fn computation(&mut self) {
        // "main" [varDecl] {funcDecl} "{" statSequence "}" "."
        while self.current() == &Token::Main {
            self.move_token();
            if self.current() == &Token::Symbol(Symbol::Period) {
                break;
            }
            if self.current() == &Token::Var {
                // Var Decl Path
                self.varDecl();
            }
            if self.current() == &Token::Void {
                // Func Decl Path
                self.funcDecl();
            }
            if self.current() == &Token::Symbol(Symbol::OpenBrace) {
                self.statSequence(); // Stat Sequence Path
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
}
