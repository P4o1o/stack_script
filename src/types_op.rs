//
// types_op.rs - Type operations
//

use crate::error::{Result, SscriptError};
use crate::stack::{ElemType, Stack, StackElem};

pub fn op_type(stack: &mut Stack) -> Result<()> {
    let elem = stack.peek().ok_or(SscriptError::StackUnderflow)?;
    let t = elem.elem_type();
    stack.push(StackElem::Type(t));
    Ok(())
}

pub fn op_type_instr(stack: &mut Stack) {
    stack.push(StackElem::Type(ElemType::Instruction));
}

pub fn op_type_int(stack: &mut Stack) {
    stack.push(StackElem::Type(ElemType::Integer));
}

pub fn op_type_float(stack: &mut Stack) {
    stack.push(StackElem::Type(ElemType::Floating));
}

pub fn op_type_bool(stack: &mut Stack) {
    stack.push(StackElem::Type(ElemType::Boolean));
}

pub fn op_type_str(stack: &mut Stack) {
    stack.push(StackElem::Type(ElemType::String));
}

pub fn op_type_type(stack: &mut Stack) {
    stack.push(StackElem::Type(ElemType::Type));
}

pub fn op_type_none(stack: &mut Stack) {
    stack.push(StackElem::Type(ElemType::None));
}

pub fn op_type_stack(stack: &mut Stack) {
    stack.push(StackElem::Type(ElemType::InnerStack));
}
