# Frontend 
## Structure (Current version)
### FSM: 
 - Token: 
    - Role: Defines Token types 
- Tokenizer:
    - Role: Generate Tokens 
### Parser (WIP):
- Parser: (WIP)
    - Role: Parser thing. 
    - Current stage: skeleton 
    - varDecl: adding variable in variables' hash map (ex: var1 : 0 as default)
- Return Values for functions: 
    - The return value type will be i32 because it returns the inst #.
        - TODO: Need to figure out which inst# should be return
    

### Operator (WIP): TODO: Test! 
- Operator: (WIP) 
    - Role: Define Operator types 
- Inst
- InstStorage: Instruction Storage for OP 
    - This is for optimization: common subexpression elimination (for now)
    - I'll think about the copy propagation later 

## Performance: (WIP: STUDY!!!)
### [Copy propagation](https://en.wikipedia.org/wiki/Copy_propagation):
```rust
// Example
let a = 1; 
let b = a; 
let c = 3 * b;

// After Copy propagation 
let a = 1; 
let c = 3 * b;
```
### [Common subexpression elimination](https://en.wikipedia.org/wiki/Common_subexpression_elimination):
```rust 
// Example
let a = 2; 
let b = a + 2 + 5; 
let c = a + 2 * 4; 

// After Common subexpression elimination 
let a = 2; 
let a_add_two = a + 2; 
let b = a_add_two + 5; 
let c = a_add_two * 4;
```