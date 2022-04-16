use crate::virtual_machine::{Var, Val, Lbl, Operation};
use std::fs::File;
use std::io::Read;
use std::{env, collections::HashMap};

#[allow(dead_code)]
// parse from sliced var 'x' -> x:Var    
fn parse_var(var: &str) -> Var {
    if var.starts_with('\'') && var.ends_with('\'') && var.len() == 3 {
        let s = var[1..(var.len() - 1)].to_ascii_lowercase();
        let x = s.chars().next().unwrap();
        x
    } else {
        panic!("Wrong VAR declaration")
        }
    }
#[allow(dead_code)]
fn parse_val(value: &str) -> Val {
    value.parse().unwrap()
}

// FUNCTION for parsing from "filename" to return modificators and code operations 
// ----------------------------------------------------------------------------------------- 
// #[modificators] - Mapped labels in code which allows to jump on it declared lines numbers
// operations      - Parsed Vector of commands from file 
// ----------------------------------------------------------------------------------------- 
// You can testing this function :
// 1). Insert filename var parse_code(filename:&str)
// 2). Mute args and path lines (first two in this function)
// 3). Unlock this code line >>> let path = filename; <<<


#[allow(dead_code)]
pub(crate) fn parse_code() -> (HashMap<String, u8>, Vec<Operation>) {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    // create Hashmap for #[modificator] and Vector for operations
    let mut modificators: HashMap<String,u8> = HashMap::default();
    let mut operations: Vec<Operation> = vec![];
    // read file to buffer String
    // use this case for virtual_machine.rs tests >>>
    // let path = filename; <<< 
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // split buffer to instructions
    let instruction: Vec<&str> = buf.split('\n')
                        // split lines
                         .map(|i| i.trim())
                         // clear spaces from lines
                         .filter(|i| !i.starts_with("//") && i.len()>1)
                         // clear commentaries
                         .collect();
                         // collect all to Vector

    // parse and collect modificators
    for (index, item) in instruction.iter().enumerate() {
        // modificators starts with '#[' and ends with ']'
        if item.starts_with("#[") && item.ends_with("]") {
            // line number = index of current line - amount of certain modificators
            let label = index  - modificators.len();
                // add to instructions modificator with label
                modificators.insert(item[2..item.len() - 1].to_string(), label as Lbl);
            }
        }
    // split instructions to blocks for each instruction and drop NaN
    // match all operations to this block slice and push them into operations map
    for item in instruction { 
        let block: Vec<&str> = item.split(|c| c == ' ' || c == ',')
                                    .filter(|c| c.len() > 0)
                                    .collect();
        let operation = match &block[..] {
            &["LOAD_VAL", value] => Operation::LOAD_VAL(parse_val(value)),
            &["WRITE_VAR", value] => Operation::WRITE_VAR(parse_var(value)),
            &["READ_VAR", value] => Operation::READ_VAR(parse_var(value)),
            &["ADD"] => Operation::ADD,
            &["MULTIPLY"] => Operation::MULTIPLY,
            &["INCREMENT"] => Operation::INCREMENT,
            &["#[BREAK]"] => continue,
            &["#[LOOP]"] => continue,
            &["CMP"] => Operation::CMP,
            // match Operation using labels to modificators map
            &["JUMP", label] => match modificators.get(label) {
                Some(label) => Operation::JUMP(*label),
                None => panic!("label `{}` not found!", label)
            },
            &["JUMP_EQUALS", label] => match modificators.get(label) {
                Some(label) => Operation::JUMP_EQUALS(*label),
                None => panic!("label `{}` not found!", label)
            },
            &["JUMP_GREATER", label] => match modificators.get(label) {
                Some(label) => Operation::JUMP_GREATER(*label),
                None => panic!("label `{}` not found!", label)
            }, 
            &["JUMP_LESS", label] => match modificators.get(label) {
                Some(label) => Operation::JUMP_LESS(*label),
                None => panic!("label `{}` not found!", label)
            },
            &["SEND_CHANNEL", data] => Operation::SEND_CHANNEL(str::parse(data).unwrap()),
            &["RECV_CHANNEL"] => Operation::RECV_CHANNEL,
            &["SPAWN"] => Operation::SPAWN,
            &["RETURN_VALUE"] => Operation::RETURN_VALUE,
            _ => panic!("Wrong Operation")
        };
        operations.push(operation);
    }

    (modificators, operations)
   
}

#[test]
fn test_parse_modificators() {

    let mut codemap: HashMap<String,u8> = HashMap::default();
    codemap.insert("LOOP".into(), 2);
    codemap.insert("BREAK".into(), 10);

    //let (modificators, _) = parse_code("./code_examples/loop1.language");
    //assert_eq!(modificators, codemap);
}
#[test]
#[allow(dead_code)]
fn test_parse_operations() {

    let _operations_vec = vec![
        Operation::LOAD_VAL(0),
        Operation::WRITE_VAR('x'),
        Operation::LOAD_VAL(5),
        Operation::READ_VAR('x'),
        Operation::CMP,
        Operation::JUMP_GREATER(10),
        Operation::READ_VAR('x'),
        Operation::INCREMENT,
        Operation::WRITE_VAR('x'),
        Operation::JUMP(2),
        Operation::READ_VAR('x'),
        Operation::RETURN_VALUE
    ];

    //let (_, operations) = parse_code("./code_examples/loop1.language");
    //assert_eq!(operations, _operations_vec);

}
