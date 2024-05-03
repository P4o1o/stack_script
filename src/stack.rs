use std::borrow::ToOwned;
use std::clone::Clone;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::convert::Into;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::iter::Iterator;
use crate::environments::Environment;
use std::ops::{Add, Div, Mul, Rem, Sub, BitAnd, BitOr, BitXor, Not, Fn};
use std::option::Option;
use std::option::Option::{Some, None};
use std::result::Result;
use std::result::Result::{Ok, Err};
use std::string::{String, ToString};
use crate::stack::Errors::{StackOverflow, StackUnderflow};

pub(crate) const INSTRUCTIONS: [&str; 38] = [
    "+", "-", "*", "/", "%", "pow", "sqrt",
    "size", "maxsize", "bottom", "last", "empty",
    "true", "false",
    "not", "and", "or", "xor",
    "==", "!=", ">", ">=", "<", "<=",
    "nop", "if", "loop",
    "quote", "dup", "swap", "compose", "apply", "clear", "drop",
    "exit", "int", "print", "printall",
];
pub(crate) const BRACKETS_INSTR: [&str; 10] = [
    "if(", "loop(",
    "dup(", "swap(",
    "load(", "save(",
    "define(", "isdef(", "delete(",
    ")"
];
pub(crate) const STACK_SIZE: usize = 256;

pub(crate) enum Errors {
    StackUnderflow(String),
    StackOverflow(String),
    InvalidOperands(String),
    InvalidCharacter(char),
    ParenthesisError,
    ExecutionEnd,
    ZeroDivision,
    DefineInvalidName(String),
    InvalidInstruction(String),
    FileNotExists(String),
    IOError(String),
    FileNotCreatable(String),
}

impl Errors {
    pub(crate) fn msg(&self) -> String{
        match self {
            Errors::StackUnderflow(x) => "StackUnderflow by operation ".to_string() + x,
            Errors::StackOverflow(x) => "StackUnderflow by operation ".to_string() + x,
            Errors::InvalidOperands(x) => "InvalidOperands for operation ".to_string() + x,
            Errors::InvalidCharacter(x) => "InvalidCharacter ".to_string() + &x.to_string(),
            Errors::ParenthesisError => "ParenthesisError".to_string(),
            Errors::ExecutionEnd => "ExecutionEnd".to_string(),
            Errors::ZeroDivision => "ZeroDivision".to_string(),
            Errors::DefineInvalidName(x) => "DefineInvalidName ".to_string() + x,
            Errors::InvalidInstruction(x) => "InvalidInstruction ".to_string() + x,
            Errors::FileNotExists(x) => "FileNotExists ".to_string() + x,
            Errors::IOError(x) => "IOError ".to_string() + x,
            Errors::FileNotCreatable(x) => "FileNotCreatable ".to_string() + x,
        }
    }
}

pub(crate) enum Values {
    Float(f32),
    Int(i32),
    True,
    False
}

pub(crate) enum StackElem {
    Instruction(String),
    Value(Values),
}

pub(crate) struct Stack {
    content: [StackElem; STACK_SIZE],
    next: usize,
}

fn match_maths_type<T, S>(x: &StackElem, y: &StackElem, func_int: T, func_float: S) -> Option<StackElem>
    where T: Fn(i32, i32) -> i32, S: Fn(f32, f32) -> f32 {
    match (x, y) {
        (StackElem::Value(a), StackElem::Value(b)) => match (a, b) {
            (Values::Int(x), Values::Int(y)) => Some(StackElem::Value(Values::Int(func_int(*x, *y)))),
            (Values::Int(x), Values::Float(y)) => Some(StackElem::Value(Values::Float(func_float(*x as f32, *y)))),
            (Values::Float(x), Values::Int(y)) => Some(StackElem::Value(Values::Float(func_float(*x, *y as f32)))),
            (Values::Float(x), Values::Float(y)) => Some(StackElem::Value(Values::Float(func_float(*x, *y)))),
            _ => None
        },
        _ => None,
    }
}

fn correct_define_name(name: &str) -> bool{
    if name.is_empty() {
        return false;
    }

    if let Some(first_char) = name.chars().next() {
        if (!first_char.is_alphabetic()) || first_char.is_whitespace() {
            return false;
        }
    } else {
        // Se non c'è un primo carattere, la stringa è vuota o contiene solo spazi
        return false;
    }
    for c in name.chars().skip(1) {
        if (!c.is_alphanumeric()) || c.is_whitespace() {
            return false;
        }
    }
    if INSTRUCTIONS.contains(&name) {
        return false
    }
    true
}

impl Add for StackElem {
    type Output = Result<StackElem, Errors>;

    fn add(self, rhs: Self) -> Self::Output {
        match match_maths_type(&self, &rhs, |a, b| a + b, |x, y| x + y){
            None => Err(Errors::InvalidOperands(INSTRUCTIONS[0].to_string())),
            Some(x) => Ok(x)
        }
    }
}

impl Sub for StackElem {
    type Output = Result<StackElem, Errors>;

    fn sub(self, rhs: Self) -> Self::Output {
        match match_maths_type(&self, &rhs, |a, b| a - b, |x, y| x - y){
            None => Err(Errors::InvalidOperands(INSTRUCTIONS[1].to_string())),
            Some(x) => Ok(x)
        }
    }
}

impl Mul for StackElem {
    type Output = Result<StackElem, Errors>;

    fn mul(self, rhs: Self) -> Self::Output {
        match match_maths_type(&self, &rhs, |a, b| a * b, |x, y| x * y){
            None => Err(Errors::InvalidOperands(INSTRUCTIONS[2].to_string())),
            Some(x) => Ok(x)
        }
    }
}

impl Div for StackElem {
    type Output = Result<StackElem, Errors>;

    fn div(self, rhs: Self) -> Self::Output {
        match &rhs {
            StackElem::Value(x) => match x{
                Values::Float(a) => if *a == 0.0 {
                    return Err(Errors::ZeroDivision);
                }
                Values::Int(a) => if *a == 0 {
                    return Err(Errors::ZeroDivision);
                },
                _ => return Err(Errors::InvalidOperands(INSTRUCTIONS[3].to_string()))
            },
            _ => return Err(Errors::InvalidOperands(INSTRUCTIONS[3].to_string()))
        };
        match match_maths_type(&self, &rhs, |a, b| a / b, |x, y| x / y){
            None => Err(Errors::InvalidOperands(INSTRUCTIONS[3].to_string())),
            Some(x) => Ok(x)
        }
    }
}

impl Rem for StackElem {
    type Output = Result<StackElem, Errors>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StackElem::Value(a), StackElem::Value(b)) => match (a, b) {
                (Values::Int(x), Values::Int(y)) => match y {
                    0 => Err(Errors::ZeroDivision),
                    _ => Ok(StackElem::Value(Values::Int(x % y)))
                },
                _ => Err(Errors::InvalidOperands(INSTRUCTIONS[4].to_string())),
            },
            (_, _) => Err(Errors::InvalidOperands(INSTRUCTIONS[4].to_string())),
        }
    }
}

impl Not for StackElem {
    type Output = Result<StackElem, Errors>;

    fn not(self) -> Self::Output {
        match self {
            StackElem::Value(a) => match a {
                Values::True => Ok(StackElem::Value(Values::False)),
                Values::False => Ok(StackElem::Value(Values::True)),
                _ => Err(Errors::InvalidOperands("not".to_string()))
            }
            _ => Err(Errors::InvalidOperands("not".to_string()))
        }
    }
}

impl BitAnd for StackElem{
    type Output = Result<StackElem, Errors>;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StackElem::Value(a), StackElem::Value(b)) => match (a, b) {
                (Values::True, Values::True) => Ok(StackElem::Value(Values::True)),
                (Values::False, Values::True) => Ok(StackElem::Value(Values::False)),
                (Values::True, Values::False) => Ok(StackElem::Value(Values::False)),
                (Values::False, Values::False) => Ok(StackElem::Value(Values::False)),
                _ => Err(Errors::InvalidOperands("and".to_string()))
            }
            _ => Err(Errors::InvalidOperands("and".to_string()))
        }
    }
}

impl BitOr for StackElem{
    type Output = Result<StackElem, Errors>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StackElem::Value(a), StackElem::Value(b)) => match (a, b) {
                (Values::True, Values::True) => Ok(StackElem::Value(Values::True)),
                (Values::False, Values::True) => Ok(StackElem::Value(Values::True)),
                (Values::True, Values::False) => Ok(StackElem::Value(Values::True)),
                (Values::False, Values::False) => Ok(StackElem::Value(Values::False)),
                _ => Err(Errors::InvalidOperands("or".to_string()))
            }
            _ => Err(Errors::InvalidOperands("or".to_string()))
        }
    }
}

impl BitXor for StackElem{
    type Output = Result<StackElem, Errors>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StackElem::Value(a), StackElem::Value(b)) => match (a, b) {
                (Values::True, Values::True) => Ok(StackElem::Value(Values::False)),
                (Values::False, Values::True) => Ok(StackElem::Value(Values::True)),
                (Values::True, Values::False) => Ok(StackElem::Value(Values::True)),
                (Values::False, Values::False) => Ok(StackElem::Value(Values::False)),
                _ => Err(Errors::InvalidOperands("xor".to_string()))
            }
            _ => Err(Errors::InvalidOperands("xor".to_string()))
        }
    }
}

impl PartialEq for StackElem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StackElem::Instruction(a), StackElem::Instruction(b)) => *a == *b,
            (StackElem::Value(a), StackElem::Value(b)) => match (a, b) {
                (Values::Int(x), Values::Int(y)) => *x == *y,
                (Values::Float(x), Values::Float(y)) => *x == *y,
                (Values::Float(x), Values::Int(y)) => *x == (*y as f32),
                (Values::Int(x), Values::Float(y)) => (*x as f32) == *y,
                (Values::True, Values::True) => true,
                (Values::False, Values::False) => true,
                _ => false
            }
            _ => false
        }
    }
}

impl PartialOrd for StackElem {
    fn partial_cmp(&self, other: &StackElem) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (StackElem::Value(a), StackElem::Value(b)) => match (a, b) {
                (Values::Int(x), Values::Int(y)) => Some(x.cmp(y)),
                (Values::Float(x), Values::Float(y)) => x.partial_cmp(y),
                (Values::Float(x), Values::Int(y)) => x.partial_cmp(&(*y as f32).into()),
                (Values::Int(x), Values::Float(y)) => (*x as f32).partial_cmp(y),
                _ => None,
            },
            _ => None,
        }
    }
}


impl StackElem {

    fn boolean(val :bool) -> StackElem{
        if val{
            return StackElem::Value(Values::True)
        }
        StackElem::Value(Values::False)
    }

    fn pow(self, exp: StackElem) -> Result<StackElem, Errors> {
        match (self, exp) {
            (StackElem::Value(a), StackElem::Value(b)) => match (a, b) {
                (Values::Int(x), Values::Int(y)) => {
                    if y < 0 {
                        return Err(Errors::InvalidOperands("pow".to_string()));
                    }
                    Ok(StackElem::Value(Values::Int(x.pow(y as u32))))
                },
                (Values::Int(x), Values::Float(y)) => Ok(StackElem::Value(Values::Float((x as f32).powf(y)))),
                (Values::Float(x), Values::Int(y)) => Ok(StackElem::Value(Values::Float(x.powf(y as f32)))),
                (Values::Float(x), Values::Float(y)) => Ok(StackElem::Value(Values::Float(x.powf(y)))),
                _ => Err(Errors::InvalidOperands("pow".to_string()))
            },
            _ => Err(Errors::InvalidOperands("pow".to_string())),
        }
    }
    fn sqrt(self) -> Result<StackElem, Errors> {
        match self {
            StackElem::Value(a) => match a {
                Values::Float(x) => Ok(StackElem::Value(Values::Float(x.sqrt()))),
                Values::Int(x) => Ok(StackElem::Value(Values::Float((x as f32).sqrt()))),
                _ => Err(Errors::InvalidOperands("sqrt".to_string()))
            },
            _ => Err(Errors::InvalidOperands("sqrt".to_string())),
        }
    }

    fn get_instructions(self) -> Option<String> {
        match self {
            StackElem::Instruction(a) => Some(a[1..a.len() - 1].to_owned()),
            _ => None
        }
    }

    fn to_string(&self) -> String {
        match self {
            StackElem::Instruction(a) => (*a).clone(),
            StackElem::Value(a) => match a {
                Values::Float(x) => x.to_string(),
                Values::Int(x) => x.to_string(),
                Values::True => "true".to_string(),
                Values::False => "false".to_string(),
            }
        }
    }

    fn from_string(instr: &String) -> Option<StackElem>{
        if instr.starts_with('[') {
            return Some(StackElem::Instruction(instr.clone()))
        } else {
            return match instr.parse::<i32>() {
                Ok(i) => Some(StackElem::Value(Values::Int(i))),
                Err(_) => match instr.parse::<f32>() {
                    Ok(f) => Some(StackElem::Value(Values::Float(f))),
                    Err(_) => None
                }
            }
        }
    }

    fn extend(self, other: Self) -> Result<StackElem, Errors> {
        match (self, other) {
            (StackElem::Instruction(a), StackElem::Instruction(b)) => {
                Ok(StackElem::Instruction(a[0..a.len() - 1].to_owned() + " " + &b[1..b.len()]))
            }
            _ => Err(Errors::InvalidOperands("compose".to_string()))
        }
    }

    fn clone(&self) -> Self {
        match self {
            StackElem::Instruction(a) => StackElem::Instruction((*a).clone()),
            StackElem::Value(a) => match a {
                Values::Float(x) => StackElem::Value(Values::Float(*x)),
                Values::Int(x) => StackElem::Value(Values::Int(*x)),
                Values::True => StackElem::Value(Values::True),
                Values::False => StackElem::Value(Values::False)
            }
        }
    }
}

enum StackErrors{
    Overflow,
    Underflow
}

impl StackErrors{
    fn blame(self, operation : String) -> Errors{
        match self {
            StackErrors::Overflow => Errors::StackOverflow(operation),
            StackErrors::Underflow => Errors::StackUnderflow(operation)
        }
    }
}

const DEFAULT_STACK_ELEM: StackElem = StackElem::Value(Values::False);

impl Stack {
    pub(crate) fn new() -> Self {
        Stack {
            content: [DEFAULT_STACK_ELEM; STACK_SIZE],
            next: 0,
        }
    }

    fn push(&mut self, val: StackElem) -> Result<(), StackErrors> {
        if self.next >= STACK_SIZE {
            return Err(StackErrors::Overflow)
        }
        self.content[self.next] = val;
        self.next += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<StackElem, StackErrors> {
        if self.next == 0 {
            return Err(StackErrors::Underflow)
        }
        self.next -= 1;
        Ok(self.content[self.next].clone())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.next == 0
    }

    pub(crate) fn is_full(&self) -> bool {
        self.next >= STACK_SIZE
    }

    pub(crate) fn clear(&mut self) {
        self.next = 0;
    }

    fn swap(&mut self) -> Result<(), StackErrors> {
        if self.next < 2 {
            return Err(StackErrors::Underflow);
        }
        self.content.swap(self.next - 2, self.next - 1);
        Ok(())
    }

    fn precise_swap(&mut self, from_first: usize) -> Result<(), StackErrors> {
        if self.next < 1 + from_first {
            return Err(StackErrors::Underflow);
        }
        self.content.swap(self.next - 1 - from_first, self.next - 1);
        Ok(())
    }

    fn dig(&mut self, i: usize) -> Result<(), StackErrors> {
        if self.next <= i {
            return Err(StackErrors::Underflow);
        }
        self.push(self.content[self.next - i - 1].clone())
    }

    fn dup(&mut self) -> Result<(), StackErrors> {
        if self.next == 0 {
            return Err(StackErrors::Underflow)
        } else if self.next >= STACK_SIZE {
            return Err(StackErrors::Overflow)
        }
        self.push(self.content[self.next - 1].clone())
    }

    fn bottom(&mut self) -> Result<(), StackErrors> {
        if self.next == 0 {
            return Err(StackErrors::Underflow)
        }
        self.push(self.content[0].clone())
    }

    pub(crate) fn iterate<T>(&self, func: T) where T: Fn(&StackElem) -> () {
        for i in 0..self.next {
            func(&self.content[i])
        }
    }

    fn execute_instruction<T>(&mut self, instr: &String, env: &mut T) -> Result<(), Errors> where T: Environment{
        if !instr.is_empty() {
            // Quoted Instructions
            if instr.starts_with('[') {
                return match self.push(StackElem::Instruction(instr.clone())){
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.blame(instr.clone()))
                }
            } else {
                // Numerical Values
                return match instr.parse::<i32>() {
                    Ok(i) => match self.push(StackElem::Value(Values::Int(i))){
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.blame(instr.clone()))
                    }
                    Err(_) => match instr.parse::<f32>() {
                        Ok(f) => match self.push(StackElem::Value(Values::Float(f))){
                            Ok(_) => Ok(()),
                            Err(e) => Err(e.blame(instr.clone()))
                        }
                        // Mathematical Operations
                        Err(_) => {
                            // +
                            if instr.eq(INSTRUCTIONS[0]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[0].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[0].to_string()));
                                    }
                                };
                                let res = match arg0 + arg1 {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[0].to_string()));
                                    }
                                }
                            // -
                            } else if instr.eq(INSTRUCTIONS[1]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[1].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[1].to_string()));
                                    }
                                };
                                let res = match arg0 - arg1 {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[1].to_string()));
                                    }
                                }
                            // *
                            } else if instr.eq(INSTRUCTIONS[2]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[2].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[2].to_string()));
                                    }
                                };
                                let res = match arg0 * arg1 {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[2].to_string()));
                                    }
                                }
                            // /
                            } else if instr.eq(INSTRUCTIONS[3]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[3].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[3].to_string()));
                                    }
                                };
                                let res = match arg0 / arg1 {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[3].to_string()));
                                    }
                                }
                                // %
                            } else if instr.eq(INSTRUCTIONS[4]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[4].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[4].to_string()));
                                    }
                                };
                                let res = match arg0 % arg1 {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[4].to_string()));
                                    }
                                }
                                // pow
                            } else if instr.eq(INSTRUCTIONS[5]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[5].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[5].to_string()));
                                    }
                                };
                                let res = match arg0.pow(arg1) {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[5].to_string()));
                                    }
                                }
                                // sqrt
                            } else if instr.eq(INSTRUCTIONS[6]) {
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[6].to_string()))
                                };
                                let res = match arg0.sqrt() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[6].to_string()));
                                    }
                                }

                                // Values
                            // size
                            } else if instr.eq(INSTRUCTIONS[7]) {
                                return match self.push(StackElem::Value(Values::Int(self.next as i32))){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(INSTRUCTIONS[7].to_string()))
                                }
                            // maxsize
                            } else if instr.eq(INSTRUCTIONS[8]) {
                                return match self.push(StackElem::Value(Values::Int(STACK_SIZE as i32))){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(INSTRUCTIONS[8].to_string()))
                                }
                                // bottom
                            } else if instr.eq(INSTRUCTIONS[9]){
                                return match self.bottom(){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(INSTRUCTIONS[9].to_string()))
                                }
                                // last
                            } else if instr.eq(INSTRUCTIONS[10]){
                                return match self.next == STACK_SIZE - 1 {
                                    true => match self.push(StackElem::Value(Values::True)){
                                        Ok(_) => Ok(()),
                                        Err(e) => Err(e.blame(INSTRUCTIONS[10].to_string()))
                                    }
                                    false => match self.push(StackElem::Value(Values::False)){
                                        Ok(_) => Ok(()),
                                        Err(e) => Err(e.blame(INSTRUCTIONS[10].to_string()))
                                    }
                                };
                            // empty
                            } else if instr.eq(INSTRUCTIONS[11]) {
                                return match self.is_empty() {
                                    true => match self.push(StackElem::Value(Values::True)){
                                        Ok(_) => Ok(()),
                                        Err(e) => Err(e.blame(INSTRUCTIONS[11].to_string()))
                                    }
                                    false => match self.push(StackElem::Value(Values::False)){
                                        Ok(_) => Ok(()),
                                        Err(e) => Err(e.blame(INSTRUCTIONS[11].to_string()))
                                    }
                                };
                                // Boolean Costants
                                // true
                            } else if instr.eq(INSTRUCTIONS[12]) {
                                return match self.push(StackElem::Value(Values::True)){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(INSTRUCTIONS[12].to_string()))
                                }
                                // false
                            } else if instr.eq(INSTRUCTIONS[13]) {
                                return match self.push(StackElem::Value(Values::False)){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(INSTRUCTIONS[13].to_string()))
                                }

                                // Boolean Operations
                            // not
                            } else if instr.eq(INSTRUCTIONS[14]) {
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[14].to_string()))
                                };
                                let res = match !arg0 {
                                    Ok(val) => val,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[14].to_string()));
                                    }
                                }
                                // and
                            } else if instr.eq(INSTRUCTIONS[15]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[15].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[15].to_string()))
                                    }
                                };
                                let res = match arg0 & arg1 {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e)
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[15].to_string()))
                                    }
                                }// or
                            } else if instr.eq(INSTRUCTIONS[16]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[16].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[16].to_string()));
                                    }
                                };
                                let res = match arg0 | arg1 {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[16].to_string()));
                                    }
                                }
                            // xor
                            } else if instr.eq(INSTRUCTIONS[17]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[17].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[17].to_string()));
                                    }
                                };
                                let res = match arg0 ^ arg1 {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[17].to_string()));
                                    }
                                }
                                // Confront Operations
                                // ==
                            } else if instr.eq(INSTRUCTIONS[18]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[18].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[18].to_string()));
                                    }
                                };
                                let res = match arg0 == arg1 {
                                    true => StackElem::Value(Values::True),
                                    false => StackElem::Value(Values::False)
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[18].to_string()));
                                    }
                                }
                                // !=
                            } else if instr.eq(INSTRUCTIONS[19]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[19].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[19].to_string()));
                                    }
                                };
                                let res = match arg0 == arg1 {
                                    true => StackElem::Value(Values::False),
                                    false => StackElem::Value(Values::True)
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[19].to_string()));
                                    }
                                }
                                // >
                            }else if instr.eq(INSTRUCTIONS[20]){
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[20].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[20].to_string()));
                                    }
                                };
                                let res = match arg0.partial_cmp(&arg1) {
                                    Some(x) => x == Ordering::Greater,
                                    None => {
                                        self.next += 2;
                                        return Err(Errors::InvalidOperands(INSTRUCTIONS[20].to_string()));
                                    }
                                };
                                match self.push(StackElem::boolean(res)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[20].to_string()));
                                    }
                                }
                                // >=
                            }else if instr.eq(INSTRUCTIONS[21]){
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[21].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[21].to_string()));
                                    }
                                };
                                let res = match arg0.partial_cmp(&arg1) {
                                    Some(x) => x == Ordering::Greater || x == Ordering::Equal,
                                    None => {
                                        self.next += 2;
                                        return Err(Errors::InvalidOperands(INSTRUCTIONS[21].to_string()));
                                    }
                                };
                                match self.push(StackElem::boolean(res)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[21].to_string()));
                                    }
                                }
                            // <
                            }else if instr.eq(INSTRUCTIONS[22]){
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[22].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[22].to_string()));
                                    }
                                };
                                let res = match arg0.partial_cmp(&arg1) {
                                    Some(x) => x == Ordering::Less,
                                    None => {
                                        self.next += 2;
                                        return Err(Errors::InvalidOperands(INSTRUCTIONS[22].to_string()));
                                    }
                                };
                                match self.push(StackElem::boolean(res)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[22].to_string()));
                                    }
                                }
                                // <=
                            }else if instr.eq(INSTRUCTIONS[23]){
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[23].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[23].to_string()));
                                    }
                                };
                                let res = match arg0.partial_cmp(&arg1) {
                                    Some(x) => x == Ordering::Less || x == Ordering::Equal,
                                    None => {
                                        self.next += 2;
                                        return Err(Errors::InvalidOperands(INSTRUCTIONS[23].to_string()));
                                    }
                                };
                                match self.push(StackElem::boolean(res)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[23].to_string()));
                                    }
                                }
                                // Flow Control
                            // nop
                            } else if instr.eq(INSTRUCTIONS[24]) {
                                return Ok(());
                            // if
                            } else if instr.eq(INSTRUCTIONS[25]) {
                                let condfalse = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[25].to_string()))
                                };
                                let condtrue = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[25].to_string()));
                                    }
                                };
                                return match self.pop() {
                                    Ok(x) => match x {
                                        StackElem::Value(x) => match x {
                                            Values::False => {
                                                match condfalse.get_instructions() {
                                                    Some(ins) => self.execute(&ins, env),
                                                    None => {
                                                        self.next += 2;
                                                        return Err(Errors::InvalidOperands(INSTRUCTIONS[25].to_string()));
                                                    }
                                                }
                                            }
                                            Values::True => {
                                                match condtrue.get_instructions() {
                                                    Some(ins) => self.execute(&ins, env),
                                                    None => {
                                                        self.next += 2;
                                                        return Err(Errors::InvalidOperands(INSTRUCTIONS[25].to_string()));
                                                    }
                                                }
                                            }
                                            _ => {
                                                self.next += 2;
                                                Err(Errors::InvalidOperands(INSTRUCTIONS[25].to_string()))
                                            }
                                        }
                                        _ => {
                                            self.next += 2;
                                            Err(Errors::InvalidOperands(INSTRUCTIONS[25].to_string()))
                                        }
                                    }
                                    Err(e) => Err(e.blame(INSTRUCTIONS[25].to_string()))
                                };
                                // if(
                            }else if instr.starts_with(BRACKETS_INSTR[0]) && instr.ends_with(BRACKETS_INSTR[BRACKETS_INSTR.len() - 1]){
                                let cond = &instr[3..instr.len() - 1];
                                let condfalse = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(instr.clone()))
                                };
                                let condtrue = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(instr.clone()));
                                    }
                                };
                                match self.execute(cond, env){
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                }
                                return match self.pop() {
                                    Ok(x) => match x {
                                        StackElem::Value(x) => match x {
                                            Values::False => {
                                                match condfalse.get_instructions() {
                                                    Some(ins) => self.execute(&ins, env),
                                                    None => {
                                                        self.next += 2;
                                                        return Err(Errors::InvalidOperands(instr.clone()));
                                                    }
                                                }
                                            }
                                            Values::True => {
                                                match condtrue.get_instructions() {
                                                    Some(ins) => self.execute(&ins, env),
                                                    None => {
                                                        self.next += 2;
                                                        return Err(Errors::InvalidOperands(instr.clone()));
                                                    }
                                                }
                                            }
                                            _ => {
                                                self.next += 2;
                                                Err(Errors::InvalidOperands(instr.clone()))
                                            }
                                        }
                                        _ => {
                                            self.next += 2;
                                            Err(Errors::InvalidOperands(instr.clone()))
                                        }
                                    }
                                    Err(e) => Err(e.blame(instr.clone()))
                                };
                                // loop
                            }else if instr.eq(INSTRUCTIONS[26]){
                                let opers = match self.pop() {
                                    Ok(x) => match x.get_instructions(){
                                        Some(cont) => cont,
                                        None => return Err(Errors::InvalidOperands(INSTRUCTIONS[26].to_string()))
                                    },
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[26].to_string()))
                                };
                                loop{
                                    match self.execute(&opers, env){
                                        Ok(_) => {}
                                        Err(e) => return Err(e)
                                    }
                                    match self.pop() {
                                        Ok(x) => match x{
                                            StackElem::Value(val) => match val{
                                                Values::True => continue,
                                                Values::False => break,
                                                _ => return Err(Errors::InvalidOperands(INSTRUCTIONS[26].to_string()))
                                            }
                                            _ => return Err(Errors::InvalidOperands(INSTRUCTIONS[26].to_string()))
                                        }
                                        Err(e) => return Err(e.blame(INSTRUCTIONS[26].to_string()))
                                    }
                                }
                                // loop(
                            }else if instr.starts_with(BRACKETS_INSTR[1]) && instr.ends_with(BRACKETS_INSTR[BRACKETS_INSTR.len() - 1]){
                                let cond = &instr[5..instr.len() - 1];
                                let opers = match self.pop() {
                                    Ok(x) => match x.get_instructions(){
                                        Some(cont) => cont,
                                        None => return Err(Errors::InvalidOperands(instr.clone()))
                                    },
                                    Err(e) => return Err(e.blame(instr.clone()))
                                };
                                loop{
                                    match self.execute(cond, env){
                                        Ok(_) => match self.pop() {
                                            Ok(x) => match x{
                                                StackElem::Value(val) => match val{
                                                    Values::True => match self.execute(&opers, env){
                                                        Ok(_) => continue,
                                                        Err(e) => return Err(e)
                                                    },
                                                    Values::False => break,
                                                    _ => return Err(Errors::InvalidOperands(instr.clone()))
                                                }
                                                _ => return Err(Errors::InvalidOperands(instr.clone()))
                                            }
                                            Err(e) => return Err(e.blame(instr.clone()))
                                        }
                                        Err(e) => return Err(e)
                                    }
                                }
                                // Stack Operations
                            // quote
                            } else if instr.eq(INSTRUCTIONS[27]) {
                                let val = match self.pop() {
                                    Ok(a) => match a {
                                        StackElem::Instruction(x) => StackElem::Instruction(format!("[{}]", x)),
                                        StackElem::Value(x) => match x {
                                            Values::Float(v) => StackElem::Instruction(format!("[{}]", v)),
                                            Values::Int(v) => StackElem::Instruction(format!("[{}]", v)),
                                            Values::True => StackElem::Instruction(format!("[{}]", INSTRUCTIONS[12])),
                                            Values::False => StackElem::Instruction(format!("[{}]", INSTRUCTIONS[13]))
                                        }
                                    },
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[27].to_string()))
                                };
                                match self.push(val) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[27].to_string()));
                                    }
                                }
                                // dup
                            } else if instr.eq(INSTRUCTIONS[28]) {
                                return match self.dup(){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(INSTRUCTIONS[28].to_string()))
                                }
                                // dup(
                            }else if instr.starts_with(BRACKETS_INSTR[2]) && instr.ends_with(BRACKETS_INSTR[BRACKETS_INSTR.len() - 1]){
                                let strindex = &instr[4..instr.len() - 1];
                                let index = match self.execute(strindex, env){
                                    Ok(()) => match self.pop() {
                                        Ok(x) => match x{
                                            StackElem::Value(val) => match val{
                                                Values::Int(x) => x as usize,
                                                Values::Float(x) => x as usize,
                                                _ => return Err(Errors::InvalidOperands(instr.clone()))
                                            }
                                            _ => return Err(Errors::InvalidOperands(instr.clone()))
                                        }
                                        Err(e) => return Err(e.blame(instr.clone()))
                                    }
                                    Err(e) => return Err(e)
                                };
                                return match self.dig(index){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(instr.clone()))
                                }
                                // dup_
                            } else if instr.starts_with(INSTRUCTIONS[30]) {
                                let strindex = &instr[3..instr.len()];
                                let index = match strindex.parse::<usize>(){
                                    Ok(x) => x,
                                    Err(_) => return Err(Errors::InvalidInstruction(instr.clone()))
                                };
                                return match self.dig(index){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(instr.clone()))
                                }
                                // swap
                            } else if instr.eq(INSTRUCTIONS[29]) {
                                return match self.swap(){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(INSTRUCTIONS[29].to_string()))
                                }
                                // swap(
                            }else if instr.starts_with(BRACKETS_INSTR[3]) && instr.ends_with(BRACKETS_INSTR[BRACKETS_INSTR.len() - 1]){
                                let strindex = &instr[5..instr.len() - 1];
                                let index = match self.execute(strindex, env){
                                    Ok(()) => match self.pop() {
                                        Ok(x) => match x{
                                            StackElem::Value(val) => match val{
                                                Values::Int(x) => x as usize,
                                                Values::Float(x) => x as usize,
                                                _ => return Err(Errors::InvalidOperands(instr.clone()))
                                            }
                                            _ => return Err(Errors::InvalidOperands(instr.clone()))
                                        }
                                        Err(e) => return Err(e.blame(instr.clone()))
                                    }
                                    Err(e) => return Err(e)
                                };
                                return match self.precise_swap(index){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(instr.clone()))
                                }
                                // swap_
                            }else if instr.starts_with(INSTRUCTIONS[29]){
                                let strindex = &instr[4..instr.len()];
                                let index = match strindex.parse::<usize>(){
                                    Ok(x) => if x == 0 {return Ok(())} else {x},
                                    Err(_) => return Err(Errors::InvalidInstruction(instr.clone()))
                                };
                                return match self.precise_swap(index){
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.blame(instr.clone()))
                                }
                                // compose
                            } else if instr.eq(INSTRUCTIONS[30]) {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[30].to_string()))
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[30].to_string()));
                                    }
                                };
                                let res = match arg0.extend(arg1) {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                };
                                match self.push(res) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e.blame(INSTRUCTIONS[30].to_string()));
                                    }
                                }
                                // apply
                            } else if instr.eq(INSTRUCTIONS[31]) {
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[31].to_string()))
                                };
                                let res = match arg0.get_instructions() {
                                    Some(x) => x,
                                    None => {
                                        self.next += 1;
                                        return Err(Errors::InvalidOperands(INSTRUCTIONS[31].to_string()));
                                    }
                                };
                                return self.execute(&res, env)
                                // clear
                            } else if instr.eq(INSTRUCTIONS[32]) {
                                self.next = 0;
                                // drop
                            } else if instr.eq(INSTRUCTIONS[33]) {
                                match self.pop() {
                                    Ok(_) => {}
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[33].to_string()))
                                }
                                // System Operations
                                // exit
                            } else if instr.eq(INSTRUCTIONS[34]) {
                                return Err(Errors::ExecutionEnd);
                                // int
                            } else if instr.eq(INSTRUCTIONS[35]) {
                                let val = match self.pop() {
                                    Ok(a) => match a {
                                        StackElem::Instruction(_) => return Err(Errors::InvalidOperands(INSTRUCTIONS[35].to_string())),
                                        StackElem::Value(x) => match x {
                                            Values::Float(v) => StackElem::Value(Values::Int(v as i32)),
                                            Values::Int(_) => return Ok(()),
                                            _ => return Err(Errors::InvalidOperands(INSTRUCTIONS[35].to_string()))
                                        }
                                    },
                                    Err(e) => return Err(e.blame(INSTRUCTIONS[35].to_string()))
                                };
                                match self.push(val) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e.blame(INSTRUCTIONS[35].to_string()));
                                    }
                                }
                                // load(
                            } else if instr.starts_with(BRACKETS_INSTR[4]) && instr.ends_with(BRACKETS_INSTR[BRACKETS_INSTR.len() - 1]){
                                let path = &instr[5..instr.len() - 1];
                                let mut file = match File::open(path){
                                    Ok(x) => x,
                                    Err(_) => return Err(Errors::FileNotExists(path.to_string()))
                                };
                                let mut cont = String::new();
                                match file.read_to_string(&mut cont){
                                        Ok(_) => {}
                                        Err(_) => return Err(Errors::IOError(instr.clone()))
                                    }
                                return self.execute(&cont, env)
                                // save(
                            }else if instr.starts_with(BRACKETS_INSTR[5]) && instr.ends_with(BRACKETS_INSTR[BRACKETS_INSTR.len() - 1]){
                                let path = &instr[5..instr.len() - 1];
                                let mut file = match OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(path){
                                    Ok(x) => x,
                                    Err(_) => return Err(Errors::FileNotCreatable(path.to_string()))
                                };
                                for i in 0..self.next {
                                    match file.write_all(&(self.content[i].to_string() + " ").into_bytes()){
                                        Ok(_) => {}
                                        Err(_) => return Err(Errors::IOError(instr.clone()))
                                    }
                                }
                                // print
                            } else if instr.eq(INSTRUCTIONS[36]) {
                                if self.next == 0 {
                                    return Err(Errors::StackUnderflow(INSTRUCTIONS[36].to_string()));
                                }
                                println!("{}", match &self.content[self.next - 1] {
                                    StackElem::Instruction(i) => i.to_string(),
                                    StackElem::Value(v) => match v {
                                        Values::Float(f) => f.to_string(),
                                        Values::Int(i) => i.to_string(),
                                        Values::True => INSTRUCTIONS[12].to_string(),
                                        Values::False => INSTRUCTIONS[13].to_string()
                                    }
                                });
                                // printall
                            } else if instr.eq(INSTRUCTIONS[37]) {
                                self.print();
                                // Keywords
                            } else if instr.starts_with(BRACKETS_INSTR[6]) && instr.ends_with(BRACKETS_INSTR[BRACKETS_INSTR.len() - 1]) {
                                let name = &instr[7..instr.len() - 1];
                                match correct_define_name(name.trim()) {
                                    true => {
                                        let body = match self.pop() {
                                            Ok(val) => {
                                                match val.get_instructions() {
                                                    Some(a) => a,
                                                    None => return Err(Errors::InvalidOperands(instr.clone()))
                                                }
                                            }
                                            Err(e) => return Err(e.blame(instr.clone()))
                                        };
                                        env.set(name, body);
                                    }
                                    false => return Err(Errors::DefineInvalidName(name.to_string()))
                                }
                            } else if instr.starts_with(BRACKETS_INSTR[7]) && instr.ends_with(BRACKETS_INSTR[BRACKETS_INSTR.len() - 1]) {
                                let name = &instr[6..instr.len() - 1];
                                match correct_define_name(name) {
                                    true => {
                                        let val = match env.contains(name) {
                                            true => StackElem::Value(Values::True),
                                            false => StackElem::Value(Values::False)
                                        };
                                        return match self.push(val){
                                            Ok(_) => Ok(()),
                                            Err(e) => Err(e.blame(instr.clone()))
                                        }
                                    }
                                    false => return Err(Errors::DefineInvalidName(name.to_string()))
                                }
                            } else if instr.starts_with(BRACKETS_INSTR[8]) && instr.ends_with(BRACKETS_INSTR[BRACKETS_INSTR.len() - 1]) {
                                let name = &instr[7..instr.len() - 1];
                                match correct_define_name(name) {
                                    true => env.remove(name),
                                    false => return Err(Errors::DefineInvalidName(name.to_string()))
                                }
                            } else {
                                return match env.get(instr) {
                                    None => Err(Errors::InvalidInstruction(instr.clone())),
                                    Some(body) => self.execute(&body, env)
                                };
                            }
                            return Ok(())
                        }
                    }
                };
            }
        }
        Ok(())
    }

    pub(crate) fn execute<T>(&mut self, program: &str, env: &mut T) -> Result<(), Errors> where T: Environment{
        let mut instr = String::new();
        let mut quote: i32 = 0;
        let mut roundpar: i32 = 0;
        for charact in program.chars() {
            match charact {
                ']' => {
                    quote -= 1;
                    instr.push(charact);
                },
                '[' => {
                    quote += 1;
                    instr.push(charact);
                },
                '(' => {
                    roundpar += 1;
                    instr.push(charact);
                }
                ')' => {
                    roundpar -= 1;
                    instr.push(charact);
                }
                'a'..='z' | 'A'..='Z' | '+' | '-' | '*' | '/' | '%' | '0'..='9' | '.' | '=' | '!' | '<' | '>' | '\\' => instr.push(charact),
                ' ' | '\n' | '\t' | '\r' => {
                    match (quote, roundpar) {
                        (0, 0) => {
                            match self.execute_instruction(&instr, env) {
                                Ok(_) => {}
                                Err(e) => return Err(e)
                            };
                            instr.clear();
                        }
                        (1..=i32::MAX, _) => instr.push(charact),
                        (_, 1..=i32::MAX) => instr.push(charact),
                        _ => return Err(Errors::ParenthesisError)
                    };
                }
                x => return Err(Errors::InvalidCharacter(x))
            };
        }
        match (quote, roundpar) {
            (0, 0) => {
                self.execute_instruction(&instr, env)
            }
            _ => return Err(Errors::ParenthesisError)
        }
    }

    pub(crate) fn print(&self) {
        for i in 0..self.next {
            println!("{}", match &self.content[i] {
                StackElem::Instruction(i) => i.to_string(),
                StackElem::Value(v) => match v {
                    Values::Float(f) => f.to_string(),
                    Values::Int(i) => i.to_string(),
                    Values::True => "true".to_string(),
                    Values::False => "false".to_string()
                }
            })
        }
    }
}
