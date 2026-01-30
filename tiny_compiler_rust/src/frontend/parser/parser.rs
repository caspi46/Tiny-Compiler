use crate::frontend::fsm::{
    token::{Op, RelOp, Symbol, Token},
    tokenizer::Tokenizer,
};

pub struct Parser {
    tokens: Vec<Token>,
    cur_token: Toekn,
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
            cur_token: tokens[0],
        }
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
    }
    fn varDecl(&self) {
        // ["void"] "function" ident formalParam ";" funcBody ";"
    }
    fn funcDecl(&self) {
        // "(" [ident { "," ident }] ")"
        // new block
    }
    fn formalParam(&self) {
        // "( [ident {"," ident}] ")"
    }
    fn funcBody(&self) {
        // [varDecl] "{" [statSequence] "}"
    }
    fn computation(&self) {
        // "main" [varDecl] {funcDecl} "{" statSequence "}" "."
    }
}
