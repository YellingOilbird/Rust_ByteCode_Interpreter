## BYTECODE INTERPRETER

This is Virtual Stack Machine which interpretate code files from ./code_examples folder
Machine have stack for values from operations and vars map for declared variables
##### Run machine:

###### ```$ cargo run "your_path_to_file"``` ######
---
Unlock commands and options in parser.rs and virtual_mashine.rs for tests
Instructions for testing included in code behind both of main functions

#### Operations:
```LOAD_VAL Value```     - push Value to Virtual Machine (VM) stack  
```WRITE_VAR Var```      - pop Value from VM stack and write it to Var  
```READ_VAR Var```       - push Value from Var to VM stack  
```ADD```                - try pop two values from VM stack and add them...  
```MULTIPLY```           - ...or multiply  
```CMP```                - ...compare. VM flags switch when Value 1 ``` Value 2, or when Value 1 != Value 2  
```INCREMENT```          - after pop READ_VAR Value made it +1  
```JUMP Label```         - jump unconditionally to place in code  
```JUMP_GREATER Label``` - jump after ```CMP``` with Value 1 ``` Value 2 to place in code  
```JUMP_LESS Label```    - jump after ```CMP``` with Value 1 ``` Value 2 to place in code  
```JUMP_EQUALS Label```  - jump after ```CMP``` with Value 1 = Value 2 to place in code  
```SEND_CHANNEL Data```  - [todo!()]  
```RECV_CHANNEL```,      - [todo!()]  
```SPAWN```,             - [todo!()]  
```RETURN_VALUE```       - pop Value from VM stack  

Also, in code we have:  

// ```commentaries```    - ability to add commentaries and skip lines in code for make it more readable!     
```#[modificators]```    - markers for Loop starts and ends   

Enjoy!  
