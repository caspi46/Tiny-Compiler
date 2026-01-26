# TINY COMPILER 
###  Language (Current Language: Rust)
- Currently, I'm building this project in Rust.
- However, if I find any difficulty due to the characteristics of language, I may change it to Python or Java. 


# Design:
## Phase 1: Parser 
### Tokens:
- At this stage, all the token enums contains String type. The type can be changed at the later stage. 
- Token Types:
    - Number
    - Ident => Identifier 
    - Let
    - If 
    - Then 
    - Else 
    - Fi 
    - While 
    - Do 
    - Od 
    - Return 
    - Var 
    - Void 
    - Function 
    - Main 
    - RelOp 
    - Op
    - Symbol 

- Sub Token Types: 
    - RelOp: 
        - EQ 
        - NE 
        - GT 
        - LT
        - GE 
        - LE
    
    - Op: 
        - ADD
        - SUB
        - DIV
        - MUL
    
    - Symbol: 
        - OpenParen => "("
        - CloseParen => ")"
        - OpenBrace  => "{"
        - CloseBrace => "}"
        - Init => "<-"
        - SemiColon
        - Period
        - Comma

### Tokenizer : 
- Tokenizer generates Token from the user input.

## Future Plan (TODO): 
- Study Copy propagation & Common subexpression elimination
- Parser 
- IR
- Tree 
...
- Operation ...
- Optimization Optimization Optimization (Of course, if possible)