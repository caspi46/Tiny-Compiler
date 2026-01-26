use crate::frontend::fsm::{
    token::{Op, RelOp, Symbol, Token},
    tokenizer::Tokenizer,
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new(input: String) -> Self {
        let tokenizer = Tokenizer::new(input);
        tokenizer.generate_token();
        Self {
            tokens: tokenizer.get_token(),
        }
    }
    fn factor(&self) {}
    fn term(&self) {}
    fn expression(&self) {}
    fn relation(&self) {}
    fn assignment(&self) {}
    fn funcCall(&self) {}
    fn ifStatement(&self) {}
    fn whileStatement(&self) {}
    fn returnStatement(&self) {}
    fn statement(&self) {}
    fn statSequence(&self) {}
    fn varDecl(&self) {}
    fn funcDecl(&self) {}
    fn formalParam(&self) {}
    fn funcBody(&self) {}
    fn computation(&self) {}
}
