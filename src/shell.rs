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
            Err(e) => match e {
                Errors::StackUnderflow => println!("StackUnderflow"),
                Errors::StackOverflow => println!("StackOverflow"),
                Errors::InvalidOperands => println!("InvalidOperation"),
                Errors::InvalidCharacter => println!("InvalidCharacter"),
                Errors::ParenthesisError => println!("ParenthesisError"),
                Errors::ExecutionEnd => { println!("Program end"); break; },
                Errors::ZeroDivision => println!("ZeroDivision"),
                Errors::DefineInvalidName => println!("DefineInvalidName"),
                Errors::InvalidInstruction => println!("InvalidInstruction"),
            }
        };
    }
}