use crate::frontend::fsm::token::{Op, PreDefFunc, RelOp, Symbol, Token};

pub struct Tokenizer {
    input: Vec<char>,
    tokens: Vec<Token>,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        if input.len() == 0 {
            panic!("Error from Tokenizer: empty input");
        }
        let input_chars: Vec<char> = input.chars().collect();
        Self {
            input: input_chars,
            tokens: Vec::<Token>::new(),
        }
    }

    // fn is_letter(&self) -> bool {
    //     self.cur_ch.is_alphabetic()
    // }

    // fn is_digit(&self) -> bool {
    //     self.cur_ch.is_digit(10)
    // }

    // arugment: cur_token = token currently checking
    // check if the current value is relOp
    // NOTE: should be careful when I handle <- and <
    fn is_rel_op(&mut self, cur_token: &str) -> bool {
        match cur_token {
            "==" => self
                .tokens
                .push(Token::RelOp(RelOp::EQ(String::from("==")))),

            "!=" => self
                .tokens
                .push(Token::RelOp(RelOp::NE(String::from("!=")))),

            ">" => self.tokens.push(Token::RelOp(RelOp::GT(String::from(">")))),

            "<" => self.tokens.push(Token::RelOp(RelOp::LT(String::from("<")))),

            ">=" => self
                .tokens
                .push(Token::RelOp(RelOp::GE(String::from(">=")))),

            "<=" => self
                .tokens
                .push(Token::RelOp(RelOp::LE(String::from("<=")))),

            _ => {
                println!("Not relOp");
                return false;
            }
        }
        true
    }

    fn is_op(&mut self, cur_token: &str) -> bool {
        match cur_token {
            "+" => self.tokens.push(Token::Op(Op::ADD(String::from("+")))),

            "-" => self.tokens.push(Token::Op(Op::SUB(String::from("-")))),

            "*" => self.tokens.push(Token::Op(Op::MUL(String::from("*")))),

            "/" => self.tokens.push(Token::Op(Op::DIV(String::from("/")))),

            _ => {
                println!("Not op");
                return false;
            }
        }
        true
    }

    fn is_symbol(&mut self, cur_token: &str) -> bool {
        println!("We're in Symbol section: {}", cur_token);
        match cur_token {
            "(" => self
                .tokens
                .push(Token::Symbol(Symbol::OpenParen(String::from("(")))),

            ")" => self
                .tokens
                .push(Token::Symbol(Symbol::CloseParen(String::from(")")))),

            "{" => self
                .tokens
                .push(Token::Symbol(Symbol::OpenBrace(String::from("{")))),

            "}" => self
                .tokens
                .push(Token::Symbol(Symbol::CloseBrace(String::from("}")))),

            "<-" => self
                .tokens
                .push(Token::Symbol(Symbol::Init(String::from("<-")))),

            ";" => self
                .tokens
                .push(Token::Symbol(Symbol::SemiColon(String::from(";")))),

            "." => self
                .tokens
                .push(Token::Symbol(Symbol::Period(String::from(".")))),

            "," => self
                .tokens
                .push(Token::Symbol(Symbol::Comma(String::from(",")))),

            _ => {
                println!("Not Symbol");
                return false;
            }
        }
        true
    }

    fn is_reserved(&mut self, cur_token: &str) -> bool {
        match cur_token {
            "let" => self.tokens.push(Token::Let(String::from("let"))),

            "if" => self.tokens.push(Token::If(String::from("if"))),

            "then" => self.tokens.push(Token::Then(String::from("then"))),

            "else" => self.tokens.push(Token::Else(String::from("else"))),

            "fi" => self.tokens.push(Token::Fi(String::from("fi"))),

            "while" => self.tokens.push(Token::While(String::from("while"))),

            "do" => self.tokens.push(Token::Do(String::from("do"))),

            "od" => self.tokens.push(Token::Od(String::from("od"))),

            "return" => self.tokens.push(Token::Return(String::from("return"))),

            "var" => self.tokens.push(Token::Var(String::from("var"))),

            "void" => self.tokens.push(Token::Void(String::from("void"))),

            "function" => self.tokens.push(Token::Function(String::from("function"))),

            "main" => self.tokens.push(Token::Main(String::from("main"))),

            _ => {
                println!("Not reserved");
                return false;
            }
        }
        true
    }

    fn is_number(&mut self, cur_token: &str) -> bool {
        if cur_token.parse::<i32>().is_ok() {
            self.tokens.push(Token::Number(String::from(cur_token)));
            return true;
        }
        false
    }

    fn is_ident(&mut self, cur_token: &str) -> bool {
        if cur_token.is_empty() {
            return false;
        }
        match cur_token.chars().next() {
            Some(c) => {
                if c.is_digit(10) {
                    panic!("No digit for first letter of variable");
                }
                self.tokens.push(Token::Ident(String::from(cur_token)));
                return true;
            }
            None => panic!("Error in is_ident"),
        }
    }

    fn is_predefined_func(&mut self, cur_token: &str) -> bool {
        println!("Hello, this is predefined function checker");
        match cur_token {
            "InputNum" => self
                .tokens
                .push(Token::PreDefFunc(PreDefFunc::InputNum(String::from(
                    "InputNum",
                )))),
            "OutputNum" => {
                self.tokens
                    .push(Token::PreDefFunc(PreDefFunc::OutputNum(String::from(
                        "OutputNum",
                    ))))
            }
            "OutputNewLine" => self
                .tokens
                .push(Token::PreDefFunc(PreDefFunc::OutputNewLine(String::from(
                    "OutputNewLine",
                )))),
            _ => return false,
        }
        true
    }

    // fn is_predefined_funcs(&mut self, cur_token: String) -> bool {
    //     cur_token == "InputNum()" || cur_token == "OutputNewLinet()" // skip OutputNum(x) for now
    // }

    fn is_ch_symbol(&self, ch: char) -> bool {
        ch == '+'
            || ch == '-'
            || ch == '*'
            || ch == '/'
            || ch == '('
            || ch == ')'
            || ch == '{'
            || ch == '}'
            || ch == ' '
            || ch == ';'
    }

    fn generate_token(&mut self) {
        let mut cur_token = match self.input[0] {
            ' ' => String::new(),
            n => String::from(n),
        };
        let mut i = 1;
        while i < self.input.len() {
            println!("Enter the while loop");
            let cur_str = cur_token.as_str();
            if !self.is_ch_symbol(self.input[i]) && self.is_symbol(cur_str) {
                cur_token = if self.input[i] != ' ' {
                    String::from(self.input[i])
                } else {
                    String::new()
                };
                i += 1;
                continue;
            } else if cur_str == "<" && self.input[i] == '-' {
                cur_token += &self.input[i].to_string();
                self.is_symbol(cur_token.as_str());
            } else if self.is_ch_symbol(self.input[i])
                && !cur_str.is_empty()
                && (self.is_op(cur_str)
                    || self.is_rel_op(cur_str)
                    || self.is_reserved(cur_str)
                    || self.is_symbol(cur_str)
                    || self.is_number(cur_str)
                    || self.is_predefined_func(cur_str)
                    || self.is_ident(cur_str))
            {
                cur_token = if self.input[i] != ' ' {
                    String::from(self.input[i])
                } else {
                    String::new()
                };
                i += 1;
                continue;
            }
            if self.input[i] != ' ' {
                cur_token += &self.input[i].to_string();
            }
            i += 1;
        }
        let cur_str = cur_token.as_str();
        if !cur_str.is_empty()
            && (self.is_op(cur_str)
                || self.is_rel_op(cur_str)
                || self.is_reserved(cur_str)
                || self.is_symbol(cur_str)
                || self.is_number(cur_str)
                || self.is_predefined_func(cur_str)
                || self.is_ident(cur_str))
        {}
        println!("Current token at the end: {}", cur_token);
    }

    fn get_token(&self) {
        println!("Tokens: {:?}", self.tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = String::from("1 <- + (var3) ; let v InputNum");
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.generate_token();
        tokenizer.get_token();
    }
}
