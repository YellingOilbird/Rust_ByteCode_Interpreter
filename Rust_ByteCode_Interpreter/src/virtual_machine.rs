use std::collections::HashMap;
use crate::parser::parse_code;
use crate::channel::{Sender, Receiver, channel, ChannelData};
    
pub(crate) type Val = u8;
pub(crate) type Var = char;
pub(crate) type Lbl = u8; 
pub(crate) type Data = u8; 
pub(crate) type Channel = char;        

#[derive (Clone, Copy, PartialEq, Eq, Debug)]
#[allow(non_camel_case_types, dead_code)]
pub enum Operation {
    LOAD_VAL (Val),
    WRITE_VAR(Var),
    READ_VAR (Var),
    ADD,
    MULTIPLY,
    CMP,
    INCREMENT,
    JUMP (Lbl),
    JUMP_GREATER (Lbl),
    JUMP_LESS (Lbl),
    JUMP_EQUALS (Lbl),
    SEND_CHANNEL (Channel),
    RECV_CHANNEL (Channel),
    SPAWN,
    RETURN_VALUE
}
#[derive (Debug, Clone)]
pub enum VirtualMachineError {
    MismatchType,
    MissingVariable(String),
    EmptyStack,
    ReceiveChannelError
}
#[derive (Debug, Clone)]
pub struct VirtualMachine {
    flag: [bool; 2],
    operation_pointer: u8,
    stack: Vec<Val>,
    vars: HashMap<Var, Val>,
    operations: Vec<Operation>,
    channels: ChannelData<Data>
}

#[allow(unreachable_patterns)]
impl VirtualMachine {
    // initialization
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            flag : [false; 2],
            operation_pointer: 0,
            stack: vec![],
            vars: HashMap::new(),
            operations: vec![],
            channels: ChannelData::default()
        }
    }

    fn pop(&mut self) -> Result<u8, VirtualMachineError> {
        let result = self.stack.pop();
        match result {
            Some(v) => Ok(v),
            None => return Err(VirtualMachineError::EmptyStack),
        }
    }
    
    // FUNCTION for matching parsed code to operations and declares what each Operation do
    // -----------------------------------------------------------------------------------------  
    // You can testing this function :
    // Lock this code line >>> (_,self.functions) = parse_code(); <<<
    pub fn interpretate_code(&mut self) -> Result<Val,VirtualMachineError> {
        let mut output = Ok(Val::MIN);
        let mut output_stack:Vec<Val>= Vec::default();
        // parser.rs gives us (modificators, operations)
        (_, self.operations) = parse_code();

        while self.operation_pointer < self.operations.len() as u8 {
                match self.operations[self.operation_pointer as usize] {
                    Operation::LOAD_VAL(value)=> {
                        self.stack.push(value.clone());
                    }
                    Operation::WRITE_VAR(name) => {
                        let value = self.pop();
                        self.vars.insert(name.into(), value.unwrap());
                    }
                    Operation::READ_VAR(name) => match self.vars.get(&name) {
                        Some(&value) => self.stack.push(*&value),
                        None => return  Err(VirtualMachineError::MissingVariable(name.into()))
                        
                    }
                    Operation::RETURN_VALUE => {
                        output = self.pop();
                    }
                    Operation::MULTIPLY  => match (self.pop()?, self.pop()?) {
                        (right, left) => self.stack.push(left * right),
                        _ => return Err(VirtualMachineError::MismatchType)
                    }
                    Operation::ADD  => match (self.pop()?, self.pop()?) {
                        (right, left) => self.stack.push(left + right),
                        _ => return Err(VirtualMachineError::MismatchType)
                    }
                    Operation::CMP  => match (self.pop()?,self.pop()?) {
                        (right, left) => {
                            self.flag[0] = if left < right {true} else {false};
                            self.flag[1] = if left != right {true} else {false};
                        }
                        _ => return Err(VirtualMachineError::MismatchType)
                    }
                    Operation::INCREMENT =>  match self.pop()? {
                        value => self.stack.push(value + 1),
                        _ => return Err(VirtualMachineError::MismatchType)
                    }
                    Operation::JUMP(label) => {
                        self.operation_pointer = label;
                        continue;
                    }
                    Operation::JUMP_GREATER(label) => {
                        if self.flag[0] {
                            self.operation_pointer = label;
                            continue;
                        }
                    }
                    Operation::JUMP_LESS(label) => {
                        if !self.flag[0] {
                            self.operation_pointer = label;
                            continue;
                        }
                    }
                    Operation::JUMP_EQUALS(label) => {
                        if !self.flag[1] {
                            self.operation_pointer = label;
                            continue; 
                        }
                    }
                    Operation::SEND_CHANNEL(name) => {
                        let value = self.pop()?;
                        
                        self.channels = ChannelData::new(name);
                        self.channels.tx.send(value);

                        //self.stack.push(data)
                    }
                    Operation::RECV_CHANNEL(name) => match self.channels.rx.receive() {
                        Some(data) => self.stack.push(data),
                        None => return Err(VirtualMachineError::ReceiveChannelError),
                    },
                    Operation::SPAWN => todo!(),
                }
                self.operation_pointer += 1;
            }
            output
        }
}

#[test]
fn test_function1() -> Result<(), VirtualMachineError>{
  
    let mut vm = VirtualMachine::new();
    //(_, vm.operations) = parse_code("./code_examples/loop1.language");

    let right:u8 = 6;
    let result = vm.interpretate_code()?;

    //assert_eq!(result, right);

    Ok(())
}
#[test]
fn test_cmp() -> Result<(), VirtualMachineError>{

    let mut vm = VirtualMachine::new();

    let right:u8 = 2;
    let result = vm.interpretate_code()?;

    //assert_eq!(result, right);

    Ok(())
}
#[test]
fn test_loop() -> Result<(), VirtualMachineError>{

    let mut vm = VirtualMachine::new();
    //(_, vm.operations) = parse_code("./code_examples/loop.language");

    let right:u8 = 6;
    let result = vm.interpretate_code()?;

    //assert_eq!(result, right);

    Ok(())
}
