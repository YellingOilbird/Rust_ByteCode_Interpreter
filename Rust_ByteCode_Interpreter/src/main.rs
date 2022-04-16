mod parser;
mod virtual_machine;



use crate::virtual_machine::VirtualMachine;

fn main() {
    let mut vm = VirtualMachine::new();
    parser::parse_code();
    let result = vm.interpretate_code();
    println!("{:?}", result);
}
