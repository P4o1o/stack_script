use std::io::{self, Write};
use crate::stack::Errors;
use crate::interpreter::BaseInterpreter;

pub fn base_shell(){
    let mut stack = BaseInterpreter::new();
    loop{
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match stack.execute(&input){
            Ok(_) => {}
            Err(Errors::ExecutionEnd) => { println!("Program End"); break; },
            Err(e) => println!("{}", e.msg())
        };
    }
}