## BYTECODE INTERPRETER

This is Virtual Stack Machine which interpretate code files from ./code_examples folder
Machine have stack for values from operations and vars map for declared variables

#### Usage:

```bash
$ cargo run "./code_examples/your_file"
```

#### How to use tests ####

##### So, we have implementation with parse args from terminal to read code files. If you want to made some tests, you can do simple changes which detailed describe in code commentaries

Example via [parser.rs](https://github.com/YellingOilbird/Rust_ByteCode_Interpreter/blob/main/Rust_ByteCode_Interpreter/src/parser.rs)

Unlock this in ```rust #[test] ```:

```rust

//let (_, operations) = parse_code("./code_examples/loop1.language");
//assert_eq!(operations, _operations_vec);
```
To this:

```rust
let (_, operations) = parse_code("./code_examples/loop1.language");
assert_eq!(operations, _operations_vec);
```
And made this changes in fn parse_code() from this

```rust
pub(crate) fn parse_code() -> (HashMap<String, u8>, Vec<Operation>) {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    // create Hashmap for #[modificator] and Vector for operations
    let mut modificators: HashMap<String,u8> = HashMap::default();
    let mut operations: Vec<Operation> = vec![];
    // read file to buffer String
    // use this case for virtual_machine.rs tests >>>
    // let path = filename; <<< 
```
To this:

```rust
pub(crate) fn parse_code(filename: &str) -> (HashMap<String, u8>, Vec<Operation>) {
    //let args: Vec<String> = env::args().collect();
    //let path = &args[1];
    // create Hashmap for #[modificator] and Vector for operations
    let mut modificators: HashMap<String,u8> = HashMap::default();
    let mut operations: Vec<Operation> = vec![];
    // read file to buffer String
    // use this case for virtual_machine.rs tests >>>
    let path = filename;
```

---

#### Operations:
```LOAD_VAL Value```     - push Value to Virtual Machine (VM) stack  
```WRITE_VAR Var```      - pop Value from VM stack and write it to Var  
```READ_VAR Var```       - push Value from Var to VM stack  
```ADD```                - try pop two values from VM stack and add them...  
```MULTIPLY```           - ...or multiply  
```CMP```                - ...compare. VM flags switch when Value 1 > Value 2, or when Value 1 != Value 2  
```INCREMENT```          - after pop READ_VAR Value made it +1    
```JUMP Label```         - jump unconditionally to place in code        
```JUMP_GREATER Label``` - jump after ```CMP``` with Value 1 > Value 2 to place in code      
```JUMP_LESS Label```    - jump after ```CMP``` with Value 1 < Value 2 to place in code      
```JUMP_EQUALS Label```  - jump after ```CMP``` with Value 1 = Value 2 to place in code      
```SEND_CHANNEL Data```  - [todo!()]    
```RECV_CHANNEL```,      - [todo!()]    
```SPAWN```,             - [todo!()]    
```RETURN_VALUE```       - pop Value from VM stack    

Also, in code we have:  

// ```commentaries```    - ability to add commentaries and skip lines in code for make it more readable!     
```#[modificators]```    - markers for Loop starts and ends   

Enjoy!  



## License
[MIT](https://choosealicense.com/licenses/mit/)
