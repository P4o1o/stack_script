use std::cmp::Ordering;
use crate::environments::Environment;
use std::ops::{Add, Div, Mul, Rem, Sub, BitAnd, BitOr, BitXor, Not};

const INSTRUCTIONS: [&str; 30] = [
    "last", "empty", "sizemax", "size",
    "pow", "sqrt",
    "true", "false",
    "and", "or", "xor", "not",
    "drop", "dig", "dup", "swap", "top", "compose", "apply", "quote", "clear",
    "if", "nop",
    "exit", "int", "print", "printall",
    "define", "delete", "isdef"
];

const STACK_SIZE: usize = 256;

pub enum Errors {
    StackUnderflow,
    StackOverflow,
    InvalidOperands,
    InvalidCharacter,
    ParenthesisError,
    ExecutionEnd,
    ZeroDivision,
    DefineInvalidName,
    InvalidInstruction,
}

enum Values {
    Float(f32),
    Int(i32),
    True,
    False
}

enum StackElem {
    Instruction(String),
    Value(Values),
}

pub(crate) struct Stack {
    content: [StackElem; STACK_SIZE],
    next: usize,
}

fn match_maths_type<T, S>(x: &StackElem, y: &StackElem, func_int: T, func_float: S) -> Result<StackElem, Errors>
    where T: Fn(i32, i32) -> i32, S: Fn(f32, f32) -> f32 {
    match (x, y) {
        (StackElem::Value(a), StackElem::Value(b)) => match (a, b) {
            (Values::Int(x), Values::Int(y)) => Ok(StackElem::Value(Values::Int(func_int(*x, *y)))),
            (Values::Int(x), Values::Float(y)) => Ok(StackElem::Value(Values::Float(func_float(*x as f32, *y)))),
            (Values::Float(x), Values::Int(y)) => Ok(StackElem::Value(Values::Float(func_float(*x, *y as f32)))),
            (Values::Float(x), Values::Float(y)) => Ok(StackElem::Value(Values::Float(func_float(*x, *y)))),
            _ => Err(Errors::InvalidOperands)
        },
        _ => Err(Errors::InvalidOperands),
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
        match_maths_type(&self, &rhs, |a, b| a + b, |x, y| x + y)
    }
}

impl Sub for StackElem {
    type Output = Result<StackElem, Errors>;

    fn sub(self, rhs: Self) -> Self::Output {
        match_maths_type(&self, &rhs, |a, b| a - b, |x, y| x - y)
    }
}

impl Mul for StackElem {
    type Output = Result<StackElem, Errors>;

    fn mul(self, rhs: Self) -> Self::Output {
        match_maths_type(&self, &rhs, |a, b| a * b, |x, y| x * y)
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
                _ => return Err(Errors::InvalidOperands)
            },
            _ => return Err(Errors::InvalidOperands)
        };
        match_maths_type(&self, &rhs, |a, b| a / b, |x, y| x / y)
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
                _ => Err(Errors::InvalidOperands),
            },
            (_, _) => Err(Errors::InvalidOperands),
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
                _ => Err(Errors::InvalidOperands)
            }
            _ => Err(Errors::InvalidOperands)
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
                _ => Err(Errors::InvalidOperands)
            }
            _ => Err(Errors::InvalidOperands)
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
                _ => Err(Errors::InvalidOperands)
            }
            _ => Err(Errors::InvalidOperands)
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
                _ => Err(Errors::InvalidOperands)
            }
            _ => Err(Errors::InvalidOperands)
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
                        return Err(Errors::InvalidOperands);
                    }
                    Ok(StackElem::Value(Values::Int(x.pow(y as u32))))
                },
                (Values::Int(x), Values::Float(y)) => Ok(StackElem::Value(Values::Float((x as f32).powf(y)))),
                (Values::Float(x), Values::Int(y)) => Ok(StackElem::Value(Values::Float(x.powf(y as f32)))),
                (Values::Float(x), Values::Float(y)) => Ok(StackElem::Value(Values::Float(x.powf(y)))),
                _ => Err(Errors::InvalidOperands)
            },
            _ => Err(Errors::InvalidOperands),
        }
    }
    fn sqrt(self) -> Result<StackElem, Errors> {
        match self {
            StackElem::Value(a) => match a {
                Values::Float(x) => Ok(StackElem::Value(Values::Float(x.sqrt()))),
                Values::Int(x) => Ok(StackElem::Value(Values::Float((x as f32).sqrt()))),
                _ => Err(Errors::InvalidOperands)
            },
            _ => Err(Errors::InvalidOperands),
        }
    }

    fn get_instructions(self) -> Result<String, Errors> {
        match self {
            StackElem::Instruction(a) => Ok(a[1..a.len() - 1].to_owned()),
            _ => Err(Errors::InvalidOperands)
        }
    }

    fn extend(self, other: Self) -> Result<StackElem, Errors> {
        match (self, other) {
            (StackElem::Instruction(a), StackElem::Instruction(b)) => {
                Ok(StackElem::Instruction(a[0..a.len() - 1].to_owned() + " " + &b[1..b.len()]))
            }
            _ => Err(Errors::InvalidOperands)
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

const DEFAULT_STACK_ELEM: StackElem = StackElem::Value(Values::False);

impl Stack {
    pub(crate) fn new() -> Self {
        Stack {
            content: [DEFAULT_STACK_ELEM; STACK_SIZE],
            next: 0,
        }
    }

    fn push(&mut self, val: StackElem) -> Result<(), Errors> {
        if self.next >= STACK_SIZE {
            return Err(Errors::StackOverflow)
        }
        self.content[self.next] = val;
        self.next += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<StackElem, Errors> {
        if self.next == 0 {
            return Err(Errors::StackUnderflow)
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

    fn swap(&mut self) -> Result<(), Errors> {
        if self.next < 2 {
            return Err(Errors::StackUnderflow);
        }
        self.content.swap(self.next - 2, self.next - 1);
        Ok(())
    }

    fn dig(&mut self, i: usize) -> Result<(), Errors> {
        if self.next <= i {
            return Err(Errors::StackUnderflow);
        }
        self.push(self.content[self.next - i - 1].clone())
    }

    fn dup(&mut self) -> Result<(), Errors> {
        if self.next == 0 {
            return Err(Errors::StackUnderflow)
        } else if self.next >= STACK_SIZE {
            return Err(Errors::StackOverflow)
        }
        self.push(self.content[self.next - 1].clone())
    }

    fn top(&mut self) -> Result<(), Errors> {
        if self.next == 0 {
            return Err(Errors::StackUnderflow)
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
            if instr.starts_with('[') {
                return self.push(StackElem::Instruction(instr.clone()))
            } else {
                return match instr.parse::<i32>() {
                    Ok(i) => self.push(StackElem::Value(Values::Int(i))),
                    Err(_) => match instr.parse::<f32>() {
                        Ok(f) => self.push(StackElem::Value(Values::Float(f))),
                        // Mathematical Operations
                        Err(_) => {
                            if instr.eq("+") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("-") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("*") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("/") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("%") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("pow") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("sqrt") {
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
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
                                        return Err(e);
                                    }
                                }

                                // Values
                            } else if instr.eq("size") {
                                return self.push(StackElem::Value(Values::Int(self.next as i32)));
                            } else if instr.eq("maxsize") {
                                return self.push(StackElem::Value(Values::Int(STACK_SIZE as i32)));
                            } else if instr.eq("top") {
                                return self.top();
                            } else if instr.eq("last") {
                                return match self.next == STACK_SIZE - 1 {
                                    true => self.push(StackElem::Value(Values::True)),
                                    false => self.push(StackElem::Value(Values::False))
                                };
                            } else if instr.eq("empty") {
                                return match self.is_empty() {
                                    true => self.push(StackElem::Value(Values::True)),
                                    false => self.push(StackElem::Value(Values::False)),
                                };
                                // Boolean Costants
                            } else if instr.eq("true") {
                                return self.push(StackElem::Value(Values::True));
                            } else if instr.eq("false") {
                                return self.push(StackElem::Value(Values::False));

                                // Boolean Operations
                            } else if instr.eq("not") {
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("and") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                };
                                let res = match arg0 & arg1 {
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("or") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("xor") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                                // Confront Operations
                            } else if instr.eq("==") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("!=") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            }else if instr.eq(">"){
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                };
                                let res = match arg0.partial_cmp(&arg1) {
                                    Some(x) => x == Ordering::Greater,
                                    None => {
                                        self.next += 2;
                                        return Err(Errors::InvalidOperands);
                                    }
                                };
                                match self.push(StackElem::boolean(res)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                }
                            }else if instr.eq(">="){
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                };
                                let res = match arg0.partial_cmp(&arg1) {
                                    Some(x) => x == Ordering::Greater || x == Ordering::Equal,
                                    None => {
                                        self.next += 2;
                                        return Err(Errors::InvalidOperands);
                                    }
                                };
                                match self.push(StackElem::boolean(res)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                }
                            }else if instr.eq("<"){
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                };
                                let res = match arg0.partial_cmp(&arg1) {
                                    Some(x) => x == Ordering::Less,
                                    None => {
                                        self.next += 2;
                                        return Err(Errors::InvalidOperands);
                                    }
                                };
                                match self.push(StackElem::boolean(res)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                }
                            }else if instr.eq("<="){
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                };
                                let res = match arg0.partial_cmp(&arg1) {
                                    Some(x) => x == Ordering::Less || x == Ordering::Equal,
                                    None => {
                                        self.next += 2;
                                        return Err(Errors::InvalidOperands);
                                    }
                                };
                                match self.push(StackElem::boolean(res)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 2;
                                        return Err(e);
                                    }
                                }
                                // Flow Control
                            } else if instr.eq("nop") {
                                return Ok(());
                            } else if instr.eq("if") {
                                let condfalse = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let condtrue = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                };
                                return match self.pop() {
                                    Ok(x) => match x {
                                        StackElem::Value(x) => match x {
                                            Values::False => {
                                                match condfalse.get_instructions() {
                                                    Ok(ins) => self.execute(&ins, env),
                                                    Err(e) => {
                                                        self.next += 2;
                                                        return Err(e);
                                                    }
                                                }
                                            }
                                            Values::True => {
                                                match condtrue.get_instructions() {
                                                    Ok(ins) => self.execute(&ins, env),
                                                    Err(e) => {
                                                        self.next += 2;
                                                        return Err(e);
                                                    }
                                                }
                                            }
                                            _ => {
                                                self.next += 2;
                                                Err(Errors::InvalidOperands)
                                            }
                                        }
                                        _ => {
                                            self.next += 2;
                                            Err(Errors::InvalidOperands)
                                        }
                                    }
                                    Err(e) => Err(e)
                                };
                            }else if instr.eq("loop"){
                                let opers = match self.pop() {
                                    Ok(x) => match x.get_instructions(){
                                        Ok(cont) => cont,
                                        Err(e) => return Err(e)
                                    },
                                    Err(e) => return Err(e)
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
                                                _ => return Err(Errors::InvalidOperands)
                                            }
                                            _ => return Err(Errors::InvalidOperands)
                                        }
                                        Err(e) => return Err(e)
                                    }
                                }

                                // Stack Operations
                            } else if instr.eq("quote") {
                                let val = match self.pop() {
                                    Ok(a) => match a {
                                        StackElem::Instruction(x) => StackElem::Instruction(format!("[{}]", x)),
                                        StackElem::Value(x) => match x {
                                            Values::Float(v) => StackElem::Instruction(format!("[{}]", v)),
                                            Values::Int(v) => StackElem::Instruction(format!("[{}]", v)),
                                            Values::True => StackElem::Instruction("[true]".to_string()),
                                            Values::False => StackElem::Instruction("[false]".to_string())
                                        }
                                    },
                                    Err(e) => return Err(e)
                                };
                                match self.push(val) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("dup") {
                                match self.dup() {
                                    Ok(_) => {}
                                    Err(e) => return Err(e)
                                }
                            } else if instr.eq("swap") {
                                match self.swap() {
                                    Ok(_) => {}
                                    Err(e) => return Err(e)
                                }
                            } else if instr.eq("compose") {
                                let arg1 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
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
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("apply") {
                                let arg0 = match self.pop() {
                                    Ok(x) => x,
                                    Err(e) => return Err(e)
                                };
                                let res = match arg0.get_instructions() {
                                    Ok(x) => x,
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                };
                                return self.execute(&res, env)
                            } else if instr.eq("clear") {
                                self.next = 0;
                            } else if instr.eq("dig") {
                                let dival = match self.pop() {
                                    Ok(val) => match val {
                                        StackElem::Instruction(_) => return Err(Errors::InvalidOperands),
                                        StackElem::Value(a) => match a {
                                            Values::Float(x) => x as usize,
                                            Values::Int(x) => x as usize,
                                            _ => return Err(Errors::InvalidOperands)
                                        }
                                    }
                                    Err(e) => return Err(e)
                                };
                                match self.dig(dival) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("drop") {
                                match self.pop() {
                                    Ok(_) => {}
                                    Err(e) => return Err(e)
                                }
                                // System Operations
                            } else if instr.eq("exit") {
                                return Err(Errors::ExecutionEnd);
                            } else if instr.eq("int") {
                                let val = match self.pop() {
                                    Ok(a) => match a {
                                        StackElem::Instruction(_) => return Err(Errors::InvalidOperands),
                                        StackElem::Value(x) => match x {
                                            Values::Float(v) => StackElem::Value(Values::Int(v as i32)),
                                            Values::Int(_) => return Ok(()),
                                            _ => return Err(Errors::InvalidOperands)
                                        }
                                    },
                                    Err(e) => return Err(e)
                                };
                                match self.push(val) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        self.next += 1;
                                        return Err(e);
                                    }
                                }
                            } else if instr.eq("print") {
                                if self.next == 0 {
                                    return Err(Errors::StackUnderflow);
                                }
                                println!("{}", match &self.content[self.next - 1] {
                                    StackElem::Instruction(i) => i.to_string(),
                                    StackElem::Value(v) => match v {
                                        Values::Float(f) => f.to_string(),
                                        Values::Int(i) => i.to_string(),
                                        Values::True => "true".to_string(),
                                        Values::False => "false".to_string()
                                    }
                                });
                            } else if instr.eq("printall") {
                                self.print();
                                // Keywords
                            } else if instr.starts_with("define(") && instr.ends_with(")") {
                                let name = &instr[7..instr.len() - 1];
                                match correct_define_name(name) {
                                    true => {
                                        let body = match self.pop() {
                                            Ok(val) => {
                                                match val.get_instructions() {
                                                    Ok(a) => a,
                                                    Err(e) => return Err(e)
                                                }
                                            }
                                            Err(e) => return Err(e)
                                        };
                                        env.set(name, body);
                                    }
                                    false => return Err(Errors::DefineInvalidName)
                                }
                            } else if instr.starts_with("isdef(") && instr.ends_with(")") {
                                let name = &instr[6..instr.len() - 1];
                                match correct_define_name(name) {
                                    true => {
                                        let val = match env.contains(name) {
                                            true => StackElem::Value(Values::True),
                                            false => StackElem::Value(Values::False)
                                        };
                                        return self.push(val);
                                    }
                                    false => return Err(Errors::DefineInvalidName)
                                }
                            } else if instr.starts_with("delete(") && instr.ends_with(")") {
                                let name = &instr[7..instr.len() - 1];
                                match correct_define_name(name) {
                                    true => env.remove(name),
                                    false => return Err(Errors::DefineInvalidName)
                                }
                            } else {
                                return match env.get(instr) {
                                    None => Err(Errors::InvalidInstruction),
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

    pub(crate) fn execute<T>(&mut self, program: &String, env: &mut T) -> Result<(), Errors> where T: Environment{
        let mut instr = String::new();
        let mut quote: i32 = 0;
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
                'a'..='z' | 'A'..='Z' | '+' | '-' | '*' | '/' | '%' | '0'..='9' | '.' | '=' | '!' | '(' | ')' | '<' | '>' => instr.push(charact),
                ' ' | '\n' | '\t' | '\r' => {
                    match quote {
                        0 => {
                            match self.execute_instruction(&instr, env) {
                                Ok(_) => {}
                                Err(e) => return Err(e)
                            };
                            instr.clear();
                        }
                        1..=i32::MAX => instr.push(charact),
                        _ => return Err(Errors::ParenthesisError)
                    };
                }
                _ => return Err(Errors::InvalidCharacter)
            };
        }
        match quote {
            0 => {
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
