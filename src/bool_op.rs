//
// bool_op.rs - Boolean and comparison operations
//

use crate::error::{Result, SscriptError};
use crate::stack::{Stack, StackElem};

pub fn op_true(stack: &mut Stack) {
    stack.push(StackElem::Boolean(true));
}

pub fn op_false(stack: &mut Stack) {
    stack.push(StackElem::Boolean(false));
}

pub fn op_empty(stack: &mut Stack) {
    stack.push(StackElem::Boolean(stack.is_empty()));
}

pub fn op_not(stack: &mut Stack) -> Result<()> {
    let elem = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    match elem {
        StackElem::Boolean(b) => {
            stack.push(StackElem::Boolean(!b));
            Ok(())
        }
        _ => {
            stack.push(elem);
            Err(SscriptError::InvalidOperands("not: expected boolean".into()))
        }
    }
}

pub fn op_and(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&a, &b) {
        (StackElem::Boolean(ab), StackElem::Boolean(bb)) => {
            stack.push(StackElem::Boolean(*ab && *bb));
            Ok(())
        }
        _ => {
            stack.push(a);
            stack.push(b);
            Err(SscriptError::InvalidOperands("and: expected boolean operands".into()))
        }
    }
}

pub fn op_or(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&a, &b) {
        (StackElem::Boolean(ab), StackElem::Boolean(bb)) => {
            stack.push(StackElem::Boolean(*ab || *bb));
            Ok(())
        }
        _ => {
            stack.push(a);
            stack.push(b);
            Err(SscriptError::InvalidOperands("or: expected boolean operands".into()))
        }
    }
}

pub fn op_xor(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&a, &b) {
        (StackElem::Boolean(ab), StackElem::Boolean(bb)) => {
            stack.push(StackElem::Boolean(*ab ^ *bb));
            Ok(())
        }
        _ => {
            stack.push(a);
            stack.push(b);
            Err(SscriptError::InvalidOperands("xor: expected boolean operands".into()))
        }
    }
}

pub fn op_equal(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    let result = a == b;
    stack.push(StackElem::Boolean(result));
    Ok(())
}

pub fn op_notequal(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    let result = a != b;
    stack.push(StackElem::Boolean(result));
    Ok(())
}

fn compare_numeric(a: &StackElem, b: &StackElem) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (StackElem::Integer(ai), StackElem::Integer(bi)) => ai.partial_cmp(bi),
        (StackElem::Floating(af), StackElem::Floating(bf)) => af.partial_cmp(bf),
        (StackElem::Integer(ai), StackElem::Floating(bf)) => (*ai as f64).partial_cmp(bf),
        (StackElem::Floating(af), StackElem::Integer(bi)) => af.partial_cmp(&(*bi as f64)),
        _ => None,
    }
}

pub fn op_greater(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match compare_numeric(&a, &b) {
        Some(ord) => {
            stack.push(StackElem::Boolean(ord == std::cmp::Ordering::Greater));
            Ok(())
        }
        None => {
            stack.push(a);
            stack.push(b);
            Err(SscriptError::InvalidOperands(">: expected numeric operands".into()))
        }
    }
}

pub fn op_greatereq(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match compare_numeric(&a, &b) {
        Some(ord) => {
            stack.push(StackElem::Boolean(ord != std::cmp::Ordering::Less));
            Ok(())
        }
        None => {
            stack.push(a);
            stack.push(b);
            Err(SscriptError::InvalidOperands(">=: expected numeric operands".into()))
        }
    }
}

pub fn op_less(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match compare_numeric(&a, &b) {
        Some(ord) => {
            stack.push(StackElem::Boolean(ord == std::cmp::Ordering::Less));
            Ok(())
        }
        None => {
            stack.push(a);
            stack.push(b);
            Err(SscriptError::InvalidOperands("<: expected numeric operands".into()))
        }
    }
}

pub fn op_lesseq(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match compare_numeric(&a, &b) {
        Some(ord) => {
            stack.push(StackElem::Boolean(ord != std::cmp::Ordering::Greater));
            Ok(())
        }
        None => {
            stack.push(a);
            stack.push(b);
            Err(SscriptError::InvalidOperands("<=: expected numeric operands".into()))
        }
    }
}
