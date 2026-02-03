//
// stack_op.rs - Stack manipulation operations
//

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::{Result, SscriptError};
use crate::primitives::{BOOL, NONE, TYPES};
use crate::stack::{Stack, StackElem};

pub fn op_dup(stack: &mut Stack) -> Result<()> {
    let elem = stack.peek().ok_or(SscriptError::StackUnderflow)?;
    let copy = elem.clone();
    stack.push(copy);
    Ok(())
}

pub fn op_dup_n(stack: &mut Stack, n: usize) -> Result<()> {
    let len = stack.len();
    if n >= len {
        return Err(SscriptError::StackUnderflow);
    }
    let elem = stack.get(len - 1 - n).unwrap().clone();
    stack.push(elem);
    Ok(())
}

pub fn op_top(stack: &mut Stack) -> Result<()> {
    if stack.is_empty() {
        return Err(SscriptError::StackUnderflow);
    }
    let elem = stack.get(0).unwrap().clone();
    stack.push(elem);
    Ok(())
}

pub fn op_swap(stack: &mut Stack) -> Result<()> {
    if stack.len() < 2 {
        return Err(SscriptError::StackUnderflow);
    }
    let len = stack.len();
    stack.content.swap(len - 1, len - 2);
    Ok(())
}

pub fn op_swap_n(stack: &mut Stack, n: usize) -> Result<()> {
    if !stack.swap_top(n) {
        return Err(SscriptError::StackUnderflow);
    }
    Ok(())
}

pub fn op_drop(stack: &mut Stack) -> Result<()> {
    stack.pop().ok_or(SscriptError::StackUnderflow)?;
    Ok(())
}

pub fn op_clear(stack: &mut Stack) {
    stack.clear();
}

pub fn op_roll(stack: &mut Stack) {
    stack.roll();
}

pub fn op_dig(stack: &mut Stack, n: usize) -> Result<()> {
    if !stack.dig(n) {
        return Err(SscriptError::StackUnderflow);
    }
    Ok(())
}

pub fn op_quote(stack: &mut Stack) -> Result<()> {
    let elem = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    let quoted = match elem {
        StackElem::String(s) => {
            // Quote string: "hello" -> ["hello"]
            format!("\"{}\"", s)
        }
        StackElem::Instruction(s) => {
            // Quote instruction: [code] -> [[code]]
            format!("[{}]", s)
        }
        StackElem::Integer(i) => i.to_string(),
        StackElem::Floating(f) => format!("{}", f),
        StackElem::Boolean(b) => BOOL[b as usize].to_string(),
        StackElem::None => NONE.to_string(),
        StackElem::Type(t) => TYPES[t as usize].to_string(),
        StackElem::InnerStack(_) => {
            stack.push(elem);
            return Err(SscriptError::InvalidOperands("quote: cannot quote InnerStack".into()));
        }
    };
    
    stack.push(StackElem::Instruction(quoted.into()));
    Ok(())
}

pub fn op_compose(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&a, &b) {
        (StackElem::Instruction(ai), StackElem::Instruction(bi)) => {
            let composed = format!("{} {}", ai, bi);
            stack.push(StackElem::Instruction(composed.into()));
            Ok(())
        }
        (StackElem::String(ai), StackElem::String(bi)) => {
            let composed = format!("{}{}", ai, bi);
            stack.push(StackElem::String(composed.into()));
            Ok(())
        }
        _ => {
            stack.push(a);
            stack.push(b);
            Err(SscriptError::InvalidOperands(
                "compose: expected two instructions or two strings".into(),
            ))
        }
    }
}

pub fn op_compose_with_delimiter(stack: &mut Stack) -> Result<()> {
    let delim = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&a, &b, &delim) {
        (StackElem::String(ai), StackElem::String(bi), StackElem::String(di)) => {
            let composed = format!("{}{}{}", ai, di, bi);
            stack.push(StackElem::String(composed.into()));
            Ok(())
        }
        _ => {
            stack.push(a);
            stack.push(b);
            stack.push(delim);
            Err(SscriptError::InvalidOperands(
                "compose: expected three strings".into(),
            ))
        }
    }
}

pub fn op_nop() {
    // Do nothing
}

pub fn op_print(stack: &Stack) {
    if let Some(elem) = stack.peek() {
        println!("{}", elem);
    }
}

pub fn op_printall(stack: &Stack) {
    for elem in stack.content.iter().rev() {
        println!("{}", elem);
    }
}

pub fn print_stack(stack: &Stack, n: usize) {
    let len = stack.len();
    let start = if n > len { 0 } else { len - n };
    for i in start..len {
        println!("{}", stack.get(i).unwrap());
    }
}

pub fn op_new_stack(stack: &mut Stack) {
    stack.push(StackElem::InnerStack(Rc::new(RefCell::new(Stack::new()))));
}

pub fn op_push_inner(stack: &mut Stack) -> Result<()> {
    if stack.len() < 2 {
        return Err(SscriptError::StackUnderflow);
    }
    
    let elem = stack.pop().unwrap();
    let inner = stack.peek_mut().ok_or(SscriptError::StackUnderflow)?;
    
    match inner {
        StackElem::InnerStack(s) => {
            s.borrow_mut().push(elem);
            Ok(())
        }
        _ => {
            stack.push(elem);
            Err(SscriptError::InvalidOperands("push: expected InnerStack".into()))
        }
    }
}

pub fn op_pop_inner(stack: &mut Stack) -> Result<()> {
    let inner = stack.peek().ok_or(SscriptError::StackUnderflow)?;
    
    match inner {
        StackElem::InnerStack(s) => {
            let elem = s.borrow_mut().pop().unwrap_or(StackElem::None);
            stack.push(elem);
            Ok(())
        }
        _ => Err(SscriptError::InvalidOperands("pop: expected InnerStack".into())),
    }
}

pub fn op_compress(stack: &mut Stack) {
    let mut new_inner = Stack::new();
    std::mem::swap(&mut new_inner.content, &mut stack.content);
    stack.push(StackElem::InnerStack(Rc::new(RefCell::new(new_inner))));
}

pub fn op_split_instruction(stack: &mut Stack) -> Result<()> {
    let elem = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match elem {
        StackElem::Instruction(s) => {
            split_instruction_string(&s, stack)?;
            Ok(())
        }
        StackElem::String(s) => {
            // Split string by spaces
            for part in s.split_whitespace() {
                stack.push(StackElem::String(part.into()));
            }
            Ok(())
        }
        StackElem::InnerStack(inner) => {
            // Split inner stack onto main stack
            let inner_ref = inner.borrow();
            for elem in inner_ref.content.iter() {
                stack.push(elem.clone());
            }
            Ok(())
        }
        _ => {
            stack.push(elem);
            Err(SscriptError::InvalidOperands(
                "split: expected instruction, string, or stack".into(),
            ))
        }
    }
}

fn split_instruction_string(s: &str, stack: &mut Stack) -> Result<()> {
    let mut bracket_depth = 0;
    let mut paren_depth = 0;
    let mut in_string = false;
    let mut start = 0;
    let chars: Vec<char> = s.chars().collect();
    
    for (i, &c) in chars.iter().enumerate() {
        match c {
            '[' if !in_string => bracket_depth += 1,
            ']' if !in_string => {
                bracket_depth -= 1;
                if bracket_depth == 0 && paren_depth == 0 {
                    let part: String = chars[start..=i].iter().collect();
                    if !part.trim().is_empty() {
                        stack.push(StackElem::Instruction(part.trim().into()));
                    }
                    start = i + 1;
                }
            }
            '(' if !in_string => paren_depth += 1,
            ')' if !in_string => {
                paren_depth -= 1;
                if bracket_depth == 0 && paren_depth == 0 {
                    let part: String = chars[start..=i].iter().collect();
                    if !part.trim().is_empty() {
                        stack.push(StackElem::Instruction(part.trim().into()));
                    }
                    start = i + 1;
                }
            }
            '"' => {
                in_string = !in_string;
                if !in_string && bracket_depth == 0 && paren_depth == 0 {
                    let part: String = chars[start..=i].iter().collect();
                    if !part.trim().is_empty() {
                        stack.push(StackElem::Instruction(part.trim().into()));
                    }
                    start = i + 1;
                }
            }
            ' ' | '\t' | '\n' | '\r' if bracket_depth == 0 && paren_depth == 0 && !in_string => {
                let part: String = chars[start..i].iter().collect();
                if !part.trim().is_empty() {
                    stack.push(StackElem::Instruction(part.trim().into()));
                }
                start = i + 1;
            }
            _ => {}
        }
    }
    
    // Handle remaining
    if start < chars.len() {
        let part: String = chars[start..].iter().collect();
        if !part.trim().is_empty() {
            stack.push(StackElem::Instruction(part.trim().into()));
        }
    }
    
    Ok(())
}

pub fn op_split_with_delimiter(stack: &mut Stack) -> Result<()> {
    let delim = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let string = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&string, &delim) {
        (StackElem::String(s), StackElem::String(d)) => {
            for part in s.split(d.as_ref()) {
                if !part.is_empty() {
                    stack.push(StackElem::String(part.into()));
                }
            }
            Ok(())
        }
        _ => {
            stack.push(string);
            stack.push(delim);
            Err(SscriptError::InvalidOperands(
                "split: expected two strings".into(),
            ))
        }
    }
}
