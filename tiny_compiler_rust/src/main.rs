mod frontend;
use crate::frontend::fsm::{
    token::{Op, RelOp, Symbol, Token},
    tokenizer::Tokenizer,
};

use crate::frontend::parsing::{block, parser::Parser};
use std::env;
use std::error::Error;
use std::fs;
// use std::process;
fn main() {
    // cargo run -- FILE_PATH
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let source_code = fs::read_to_string(file_path).expect("INVALID FILE PATH");

    let mut parse = Parser::new(source_code);
    parse.run();
}
