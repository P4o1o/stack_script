//
// interpreter.rs - Virtual machine that executes compiled instructions
//

use std::cell::RefCell;
use std::fs;
use std::io::Write;
use std::rc::Rc;

use crate::bool_op::*;
use crate::compiler::compile;
use crate::environment::Environment;
use crate::error::{Result, SscriptError};
use crate::instruction::Instruction;
use crate::math_op::*;
use crate::stack::{Stack, StackElem};
use crate::stack_op::*;
use crate::types_op::*;

/// Program state containing the stack and environment
pub struct ProgramState {
    pub stack: Stack,
    pub env: Environment,
}

impl ProgramState {
    pub fn new() -> Self {
        ProgramState {
            stack: Stack::new(),
            env: Environment::new(),
        }
    }
    
    pub fn with_capacity(stack_capacity: usize, env_capacity: usize) -> Self {
        ProgramState {
            stack: Stack::with_capacity(stack_capacity),
            env: Environment::with_capacity(env_capacity),
        }
    }
}

impl Default for ProgramState {
    fn default() -> Self {
        Self::new()
    }
}

/// Interpreter that executes compiled instructions
pub struct Interpreter<'a> {
    state: &'a mut ProgramState,
}

impl<'a> Interpreter<'a> {
    pub fn new(state: &'a mut ProgramState) -> Self {
        Interpreter { state }
    }
    
    /// Execute a slice of instructions
    #[inline]
    pub fn execute(&mut self, instructions: &[Instruction]) -> Result<()> {
        for instr in instructions {
            self.execute_one(instr)?;
        }
        Ok(())
    }
    
    /// Execute a single instruction
    fn execute_one(&mut self, instr: &Instruction) -> Result<()> {
        match instr {
            // === Literal push operations ===
            Instruction::PushInt(i) => {
                self.state.stack.push(StackElem::Integer(*i));
            }
            Instruction::PushFloat(f) => {
                self.state.stack.push(StackElem::Floating(*f));
            }
            Instruction::PushBool(b) => {
                self.state.stack.push(StackElem::Boolean(*b));
            }
            Instruction::PushNone => {
                self.state.stack.push(StackElem::None);
            }
            Instruction::PushString(s) => {
                self.state.stack.push(StackElem::String(s.clone()));
            }
            Instruction::PushQuoted(code) => {
                let code_str = instructions_to_string(code);
                self.state.stack.push(StackElem::Instruction(Rc::from(code_str)));
            }
            Instruction::PushStackLiteral(code) => {
                let mut inner = Stack::new();
                let mut inner_state = ProgramState {
                    stack: std::mem::take(&mut inner),
                    env: std::mem::take(&mut self.state.env),
                };
                {
                    let mut inner_interp = Interpreter::new(&mut inner_state);
                    inner_interp.execute(code)?;
                }
                inner = std::mem::take(&mut inner_state.stack);
                self.state.env = std::mem::take(&mut inner_state.env);
                self.state.stack.push(StackElem::InnerStack(Rc::new(RefCell::new(inner))));
            }
            
            // === Math operations ===
            Instruction::Add => op_add(&mut self.state.stack)?,
            Instruction::Sub => op_sub(&mut self.state.stack)?,
            Instruction::Mul => op_mul(&mut self.state.stack)?,
            Instruction::Div => op_div(&mut self.state.stack)?,
            Instruction::Mod => op_mod(&mut self.state.stack)?,
            Instruction::Pow => op_pow(&mut self.state.stack)?,
            Instruction::Sqrt => op_sqrt(&mut self.state.stack)?,
            Instruction::Exp => op_exp(&mut self.state.stack)?,
            Instruction::Log => op_log(&mut self.state.stack)?,
            Instruction::Log2 => op_log2(&mut self.state.stack)?,
            Instruction::Log10 => op_log10(&mut self.state.stack)?,
            Instruction::Opposite => op_opposite(&mut self.state.stack)?,
            Instruction::Factorial => op_factorial(&mut self.state.stack)?,
            Instruction::Gamma => op_gamma(&mut self.state.stack)?,
            Instruction::ToInt => op_int(&mut self.state.stack)?,
            Instruction::Size => {
                let size = self.state.stack.len() as i64;
                self.state.stack.push(StackElem::Integer(size));
            }
            
            // === Trig operations ===
            Instruction::Sin => op_sin(&mut self.state.stack)?,
            Instruction::Cos => op_cos(&mut self.state.stack)?,
            Instruction::Tan => op_tan(&mut self.state.stack)?,
            Instruction::ArcSin => op_arcsin(&mut self.state.stack)?,
            Instruction::ArcCos => op_arccos(&mut self.state.stack)?,
            Instruction::ArcTan => op_arctan(&mut self.state.stack)?,
            Instruction::Sinh => op_sinh(&mut self.state.stack)?,
            Instruction::Cosh => op_cosh(&mut self.state.stack)?,
            Instruction::Tanh => op_tanh(&mut self.state.stack)?,
            Instruction::ArcSinh => op_arcsinh(&mut self.state.stack)?,
            Instruction::ArcCosh => op_arccosh(&mut self.state.stack)?,
            Instruction::ArcTanh => op_arctanh(&mut self.state.stack)?,
            
            // === Boolean operations ===
            Instruction::And => op_and(&mut self.state.stack)?,
            Instruction::Or => op_or(&mut self.state.stack)?,
            Instruction::Xor => op_xor(&mut self.state.stack)?,
            Instruction::Not => op_not(&mut self.state.stack)?,
            Instruction::True => {
                self.state.stack.push(StackElem::Boolean(true));
            }
            Instruction::False => {
                self.state.stack.push(StackElem::Boolean(false));
            }
            
            // === Comparison operations ===
            Instruction::Eq => op_equal(&mut self.state.stack)?,
            Instruction::NotEq => op_notequal(&mut self.state.stack)?,
            Instruction::Gt => op_greater(&mut self.state.stack)?,
            Instruction::GtEq => op_greatereq(&mut self.state.stack)?,
            Instruction::Lt => op_less(&mut self.state.stack)?,
            Instruction::LtEq => op_lesseq(&mut self.state.stack)?,
            Instruction::Empty => {
                let is_empty = self.state.stack.is_empty();
                self.state.stack.push(StackElem::Boolean(is_empty));
            }
            Instruction::Last => {
                // NEW: checks if current element is the last (stack has exactly 1 element)
                let is_last = self.state.stack.len() == 1;
                self.state.stack.push(StackElem::Boolean(is_last));
            }
            
            // === Stack operations ===
            Instruction::Dup => op_dup(&mut self.state.stack)?,
            Instruction::DupN(n) => op_dup_n(&mut self.state.stack, *n)?,
            Instruction::Swap => op_swap(&mut self.state.stack)?,
            Instruction::SwapN(n) => op_swap_n(&mut self.state.stack, *n)?,
            Instruction::Drop => op_drop(&mut self.state.stack)?,
            Instruction::Clear => self.state.stack.clear(),
            Instruction::Roll => op_roll(&mut self.state.stack),
            Instruction::Top => op_top(&mut self.state.stack)?,
            Instruction::DigN(n) => op_dig(&mut self.state.stack, *n)?,
            Instruction::Nop => {}
            
            // === Quote/Compose operations ===
            Instruction::Quote => op_quote(&mut self.state.stack)?,
            Instruction::Compose => op_compose(&mut self.state.stack)?,
            
            // === Type operations ===
            Instruction::GetType => op_type(&mut self.state.stack)?,
            Instruction::TypeInstr => self.state.stack.push(StackElem::Type(crate::stack::ElemType::Instruction)),
            Instruction::TypeInt => self.state.stack.push(StackElem::Type(crate::stack::ElemType::Integer)),
            Instruction::TypeFloat => self.state.stack.push(StackElem::Type(crate::stack::ElemType::Floating)),
            Instruction::TypeBool => self.state.stack.push(StackElem::Type(crate::stack::ElemType::Boolean)),
            Instruction::TypeStr => self.state.stack.push(StackElem::Type(crate::stack::ElemType::String)),
            Instruction::TypeType => self.state.stack.push(StackElem::Type(crate::stack::ElemType::Type)),
            Instruction::TypeNone => self.state.stack.push(StackElem::Type(crate::stack::ElemType::None)),
            Instruction::TypeStack => self.state.stack.push(StackElem::Type(crate::stack::ElemType::InnerStack)),
            
            // === Control flow ===
            Instruction::Apply => {
                let code = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match code {
                    StackElem::Instruction(s) => {
                        let instructions = compile(&s)?;
                        self.execute(&instructions)?;
                    }
                    _ => {
                        self.state.stack.push(code);
                        return Err(SscriptError::InvalidOperands("apply: expected instruction".into()));
                    }
                }
            }
            
            Instruction::If => {
                let false_branch = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let true_branch = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let cond = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                
                let cond_bool = match cond {
                    StackElem::Boolean(b) => b,
                    _ => {
                        self.state.stack.push(cond);
                        self.state.stack.push(true_branch);
                        self.state.stack.push(false_branch);
                        return Err(SscriptError::InvalidOperands("if: expected boolean".into()));
                    }
                };
                
                let branch = if cond_bool { true_branch } else { false_branch };
                match branch {
                    StackElem::Instruction(s) => {
                        let instructions = compile(&s)?;
                        self.execute(&instructions)?;
                    }
                    _ => {
                        return Err(SscriptError::InvalidOperands("if: expected instruction".into()));
                    }
                }
            }
            
            Instruction::IfCond(cond) => {
                let false_branch = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let true_branch = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                
                // Execute condition
                self.execute(cond)?;
                
                let cond_val = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let cond_bool = match cond_val {
                    StackElem::Boolean(b) => b,
                    _ => {
                        return Err(SscriptError::InvalidOperands("if: expected boolean from condition".into()));
                    }
                };
                
                let branch = if cond_bool { true_branch } else { false_branch };
                match branch {
                    StackElem::Instruction(s) => {
                        let instructions = compile(&s)?;
                        self.execute(&instructions)?;
                    }
                    _ => {
                        return Err(SscriptError::InvalidOperands("if: expected instruction".into()));
                    }
                }
            }
            
            Instruction::Loop => {
                let code = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match code {
                    StackElem::Instruction(s) => {
                        let instructions = compile(&s)?;
                        loop {
                            self.execute(&instructions)?;
                            let cond = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                            match cond {
                                StackElem::Boolean(b) => {
                                    if !b { break; }
                                }
                                _ => {
                                    return Err(SscriptError::InvalidOperands("loop: expected boolean".into()));
                                }
                            }
                        }
                    }
                    _ => {
                        self.state.stack.push(code);
                        return Err(SscriptError::InvalidOperands("loop: expected instruction".into()));
                    }
                }
            }
            
            Instruction::LoopCond(cond) => {
                let body = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match body {
                    StackElem::Instruction(s) => {
                        let body_instructions = compile(&s)?;
                        loop {
                            // Execute condition first
                            self.execute(cond)?;
                            let cond_val = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                            match cond_val {
                                StackElem::Boolean(b) => {
                                    if !b { break; }
                                }
                                _ => {
                                    return Err(SscriptError::InvalidOperands("loop: expected boolean from condition".into()));
                                }
                            }
                            // Execute body
                            self.execute(&body_instructions)?;
                        }
                    }
                    _ => {
                        self.state.stack.push(body);
                        return Err(SscriptError::InvalidOperands("loop: expected instruction".into()));
                    }
                }
            }
            
            Instruction::Times(count_code) => {
                let body = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match body {
                    StackElem::Instruction(s) => {
                        // Execute code to get count
                        self.execute(count_code)?;
                        let count = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                        match count {
                            StackElem::Integer(n) => {
                                let body_instructions = compile(&s)?;
                                for _ in 0..n {
                                    self.execute(&body_instructions)?;
                                }
                            }
                            _ => {
                                return Err(SscriptError::InvalidOperands("times: expected integer".into()));
                            }
                        }
                    }
                    _ => {
                        self.state.stack.push(body);
                        return Err(SscriptError::InvalidOperands("times: expected instruction".into()));
                    }
                }
            }
            
            Instruction::Dip => {
                let code = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let temp = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                
                match code {
                    StackElem::Instruction(s) => {
                        let instructions = compile(&s)?;
                        self.execute(&instructions)?;
                        self.state.stack.push(temp);
                    }
                    _ => {
                        self.state.stack.push(temp);
                        self.state.stack.push(code);
                        return Err(SscriptError::InvalidOperands("dip: expected instruction".into()));
                    }
                }
            }
            
            Instruction::Try => {
                let code = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match code {
                    StackElem::Instruction(s) => {
                        let instructions = compile(&s)?;
                        // Save stack state for potential rollback
                        let stack_backup = self.state.stack.deep_clone();
                        match self.execute(&instructions) {
                            Ok(()) => {
                                self.state.stack.push(StackElem::Boolean(true));
                            }
                            Err(_) => {
                                // Restore stack and push false
                                self.state.stack = stack_backup;
                                self.state.stack.push(StackElem::Boolean(false));
                            }
                        }
                    }
                    _ => {
                        self.state.stack.push(code);
                        return Err(SscriptError::InvalidOperands("try: expected instruction".into()));
                    }
                }
            }
            
            Instruction::Exit => {
                return Err(SscriptError::ProgramExit);
            }
            
            // === Inner stack operations ===
            Instruction::NewStack => {
                self.state.stack.push(StackElem::InnerStack(Rc::new(RefCell::new(Stack::new()))));
            }
            
            Instruction::Push => {
                let elem = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let stack_elem = self.state.stack.last_mut().ok_or(SscriptError::StackUnderflow)?;
                
                match stack_elem {
                    StackElem::InnerStack(s) => {
                        s.borrow_mut().push(elem);
                    }
                    _ => {
                        self.state.stack.push(elem);
                        return Err(SscriptError::InvalidOperands("push: expected inner stack".into()));
                    }
                }
            }
            
            Instruction::Pop => {
                let stack_elem = self.state.stack.last().ok_or(SscriptError::StackUnderflow)?;
                
                match stack_elem {
                    StackElem::InnerStack(s) => {
                        let elem = s.borrow_mut().pop().unwrap_or(StackElem::None);
                        self.state.stack.push(elem);
                    }
                    _ => {
                        return Err(SscriptError::InvalidOperands("pop: expected inner stack".into()));
                    }
                }
            }
            
            Instruction::Inject => {
                let code = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let stack_elem = self.state.stack.last().ok_or(SscriptError::StackUnderflow)?;
                
                match (&code, stack_elem) {
                    (StackElem::Instruction(s), StackElem::InnerStack(inner_rc)) => {
                        let instructions = compile(s)?;
                        let mut inner_stack = inner_rc.borrow_mut();
                        let mut inner_state = ProgramState {
                            stack: std::mem::take(&mut *inner_stack),
                            env: std::mem::take(&mut self.state.env),
                        };
                        {
                            let mut inner_interp = Interpreter::new(&mut inner_state);
                            inner_interp.execute(&instructions)?;
                        }
                        *inner_stack = std::mem::take(&mut inner_state.stack);
                        self.state.env = std::mem::take(&mut inner_state.env);
                    }
                    _ => {
                        self.state.stack.push(code);
                        return Err(SscriptError::InvalidOperands("inject: expected instruction and inner stack".into()));
                    }
                }
            }
            
            Instruction::InjectN(n) => {
                let code = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                
                match &code {
                    StackElem::Instruction(s) => {
                        let len = self.state.stack.len();
                        if *n > len {
                            self.state.stack.push(code);
                            return Err(SscriptError::StackUnderflow);
                        }
                        
                        let instructions = compile(s)?;
                        let mut errors = Vec::new();
                        
                        for i in (len - n)..len {
                            match self.state.stack.get(i) {
                                Some(StackElem::InnerStack(stack_rc)) => {
                                    let mut inner_stack = stack_rc.borrow_mut();
                                    let mut inner_state = ProgramState {
                                        stack: std::mem::take(&mut *inner_stack),
                                        env: std::mem::take(&mut self.state.env),
                                    };
                                    let mut inner_interp = Interpreter::new(&mut inner_state);
                                    if let Err(e) = inner_interp.execute(&instructions) {
                                        errors.push(e);
                                    }
                                    *inner_stack = std::mem::take(&mut inner_state.stack);
                                    self.state.env = std::mem::take(&mut inner_state.env);
                                }
                                _ => {
                                    errors.push(SscriptError::InvalidOperands(
                                        "inject: expected inner stacks".into(),
                                    ));
                                }
                            }
                        }
                        
                        if !errors.is_empty() {
                            return Err(SscriptError::InjectError(errors));
                        }
                    }
                    _ => {
                        self.state.stack.push(code);
                        return Err(SscriptError::InvalidOperands(
                            "inject: expected instruction".into(),
                        ));
                    }
                }
            }
            
            // PInjectN - sequential execution (parallel would require Arc instead of Rc)
            Instruction::PInjectN(n) => {
                // Same as InjectN - true parallelism requires architectural changes
                let code = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                
                match &code {
                    StackElem::Instruction(s) => {
                        let len = self.state.stack.len();
                        if *n > len {
                            self.state.stack.push(code);
                            return Err(SscriptError::StackUnderflow);
                        }
                        
                        let instructions = compile(s)?;
                        let mut errors = Vec::new();
                        
                        for i in (len - n)..len {
                            match self.state.stack.get(i) {
                                Some(StackElem::InnerStack(stack_rc)) => {
                                    let mut inner_stack = stack_rc.borrow_mut();
                                    let mut inner_state = ProgramState {
                                        stack: std::mem::take(&mut *inner_stack),
                                        env: std::mem::take(&mut self.state.env),
                                    };
                                    let mut inner_interp = Interpreter::new(&mut inner_state);
                                    if let Err(e) = inner_interp.execute(&instructions) {
                                        errors.push(e);
                                    }
                                    *inner_stack = std::mem::take(&mut inner_state.stack);
                                    self.state.env = std::mem::take(&mut inner_state.env);
                                }
                                _ => {
                                    errors.push(SscriptError::InvalidOperands(
                                        "pinject: expected inner stacks".into(),
                                    ));
                                }
                            }
                        }
                        
                        if !errors.is_empty() {
                            return Err(SscriptError::InjectError(errors));
                        }
                    }
                    _ => {
                        self.state.stack.push(code);
                        return Err(SscriptError::InvalidOperands(
                            "pinject: expected instruction".into(),
                        ));
                    }
                }
            }
            
            Instruction::Compress => {
                let mut inner = Stack::new();
                std::mem::swap(&mut inner, &mut self.state.stack);
                self.state.stack.push(StackElem::InnerStack(Rc::new(RefCell::new(inner))));
            }
            
            Instruction::Split => {
                let top = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match top {
                    StackElem::InnerStack(s) => {
                        let inner = s.borrow();
                        for elem in inner.content.iter() {
                            self.state.stack.push(elem.clone());
                        }
                    }
                    StackElem::Instruction(s) => {
                        // Split instruction into tokens
                        let tokens = split_instruction(&s);
                        for token in tokens {
                            self.state.stack.push(StackElem::Instruction(Rc::from(token)));
                        }
                    }
                    StackElem::String(s) => {
                        // Split string by whitespace
                        for word in s.split_whitespace() {
                            self.state.stack.push(StackElem::String(Rc::from(word)));
                        }
                    }
                    _ => {
                        self.state.stack.push(top);
                        return Err(SscriptError::InvalidOperands("split: expected inner stack, instruction, or string".into()));
                    }
                }
            }
            
            Instruction::SplitCond(cond) => {
                // Execute condition to get delimiter
                self.execute(cond)?;
                let delim = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let string = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                
                match (&string, &delim) {
                    (StackElem::String(s), StackElem::String(d)) => {
                        for part in s.split(d.as_ref()) {
                            if !part.is_empty() {
                                self.state.stack.push(StackElem::String(Rc::from(part)));
                            }
                        }
                    }
                    _ => {
                        self.state.stack.push(string);
                        self.state.stack.push(delim);
                        return Err(SscriptError::InvalidOperands("split: expected strings".into()));
                    }
                }
            }
            
            Instruction::ComposeCond(cond) => {
                // Execute condition to get delimiter
                self.execute(cond)?;
                let delim = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let second = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                let first = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                
                match (&first, &second, &delim) {
                    (StackElem::String(a), StackElem::String(b), StackElem::String(d)) => {
                        let composed = format!("{}{}{}", a, d, b);
                        self.state.stack.push(StackElem::String(Rc::from(composed)));
                    }
                    _ => {
                        self.state.stack.push(first);
                        self.state.stack.push(second);
                        self.state.stack.push(delim);
                        return Err(SscriptError::InvalidOperands("compose: expected strings".into()));
                    }
                }
            }
            
            // === Bracket operations for dynamic dup/swap/dig ===
            Instruction::DupCond(cond) => {
                self.execute(cond)?;
                let idx = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match idx {
                    StackElem::Integer(n) => {
                        op_dup_n(&mut self.state.stack, n as usize)?;
                    }
                    _ => {
                        return Err(SscriptError::InvalidOperands("dup: expected integer".into()));
                    }
                }
            }
            
            Instruction::SwapCond(cond) => {
                self.execute(cond)?;
                let idx = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match idx {
                    StackElem::Integer(n) => {
                        op_swap_n(&mut self.state.stack, n as usize)?;
                    }
                    _ => {
                        return Err(SscriptError::InvalidOperands("swap: expected integer".into()));
                    }
                }
            }
            
            Instruction::DigCond(cond) => {
                self.execute(cond)?;
                let idx = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match idx {
                    StackElem::Integer(n) => {
                        op_dig(&mut self.state.stack, n as usize)?;
                    }
                    _ => {
                        return Err(SscriptError::InvalidOperands("dig: expected integer".into()));
                    }
                }
            }
            
            // === Environment operations ===
            Instruction::Define(name) => {
                let code = self.state.stack.pop().ok_or(SscriptError::StackUnderflow)?;
                match code {
                    StackElem::Instruction(s) => {
                        let instructions = compile(&s)?;
                        self.state.env.set(name.clone(), Rc::from(instructions.as_slice()));
                    }
                    _ => {
                        self.state.stack.push(code);
                        return Err(SscriptError::InvalidOperands("define: expected instruction".into()));
                    }
                }
            }
            
            Instruction::Delete(name) => {
                self.state.env.remove(name);
            }
            
            Instruction::IsDef(name) => {
                let is_defined = self.state.env.contains(name);
                self.state.stack.push(StackElem::Boolean(is_defined));
            }
            
            Instruction::Call(name) => {
                if let Some(instructions) = self.state.env.get(name) {
                    let instructions = instructions.clone();
                    self.execute(&instructions)?;
                } else {
                    return Err(SscriptError::InvalidInstruction(name.to_string()));
                }
            }
            
            // === I/O operations ===
            Instruction::Print => {
                if let Some(elem) = self.state.stack.last() {
                    println!("{}", elem);
                }
            }
            
            Instruction::PrintAll => {
                for elem in self.state.stack.content.iter().rev() {
                    println!("{}", elem);
                }
            }
            
            Instruction::Load(filename) => {
                let content = fs::read_to_string(filename.as_ref())
                    .map_err(|_| SscriptError::FileNotFound(filename.to_string()))?;
                let instructions = compile(&content)?;
                self.execute(&instructions)?;
            }
            
            Instruction::Save(filename) => {
                let mut file = fs::File::create(filename.as_ref())
                    .map_err(|_| SscriptError::FileNotCreatable(filename.to_string()))?;
                
                for elem in &self.state.stack.content {
                    let s = match elem {
                        StackElem::Instruction(s) => format!("[{}] ", s),
                        StackElem::String(s) => format!("\"{}\" ", s),
                        StackElem::Integer(i) => format!("{} ", i),
                        StackElem::Floating(f) => format!("{} ", f),
                        StackElem::Boolean(b) => format!("{} ", if *b { "true" } else { "false" }),
                        StackElem::None => "none ".to_string(),
                        StackElem::Type(t) => format!("{} ", t),
                        StackElem::InnerStack(_) => continue, // Skip inner stacks
                    };
                    file.write_all(s.as_bytes())
                        .map_err(|_| SscriptError::IOError("write failed".into()))?;
                }
            }
        }
        
        Ok(())
    }
}

/// Convert instructions back to string representation (for quoted code)
fn instructions_to_string(instructions: &[Instruction]) -> String {
    let mut parts = Vec::new();
    
    for instr in instructions {
        let s = match instr {
            Instruction::PushInt(i) => i.to_string(),
            Instruction::PushFloat(f) => f.to_string(),
            Instruction::PushBool(true) => "true".to_string(),
            Instruction::PushBool(false) => "false".to_string(),
            Instruction::PushNone => "none".to_string(),
            Instruction::PushString(s) => format!("\"{}\"", s),
            Instruction::PushQuoted(code) => format!("[{}]", instructions_to_string(code)),
            Instruction::PushStackLiteral(code) => format!("{{{}}}", instructions_to_string(code)),
            
            // Math
            Instruction::Add => "+".to_string(),
            Instruction::Sub => "-".to_string(),
            Instruction::Mul => "*".to_string(),
            Instruction::Div => "/".to_string(),
            Instruction::Mod => "%".to_string(),
            Instruction::Pow => "pow".to_string(),
            Instruction::Sqrt => "sqrt".to_string(),
            Instruction::Exp => "exp".to_string(),
            Instruction::Log => "log".to_string(),
            Instruction::Log2 => "log2".to_string(),
            Instruction::Log10 => "log10".to_string(),
            Instruction::Opposite => "--".to_string(),
            Instruction::Factorial => "!".to_string(),
            Instruction::Gamma => "gamma".to_string(),
            Instruction::ToInt => "int".to_string(),
            Instruction::Size => "size".to_string(),
            
            // Trig
            Instruction::Sin => "sin".to_string(),
            Instruction::Cos => "cos".to_string(),
            Instruction::Tan => "tan".to_string(),
            Instruction::ArcSin => "arcsin".to_string(),
            Instruction::ArcCos => "arccos".to_string(),
            Instruction::ArcTan => "arctan".to_string(),
            Instruction::Sinh => "sinh".to_string(),
            Instruction::Cosh => "cosh".to_string(),
            Instruction::Tanh => "tanh".to_string(),
            Instruction::ArcSinh => "arcsinh".to_string(),
            Instruction::ArcCosh => "arccosh".to_string(),
            Instruction::ArcTanh => "arctanh".to_string(),
            
            // Boolean
            Instruction::And => "and".to_string(),
            Instruction::Or => "or".to_string(),
            Instruction::Xor => "xor".to_string(),
            Instruction::Not => "not".to_string(),
            Instruction::True => "true".to_string(),
            Instruction::False => "false".to_string(),
            
            // Comparison
            Instruction::Eq => "==".to_string(),
            Instruction::NotEq => "!=".to_string(),
            Instruction::Lt => "<".to_string(),
            Instruction::Gt => ">".to_string(),
            Instruction::LtEq => "<=".to_string(),
            Instruction::GtEq => ">=".to_string(),
            Instruction::Empty => "empty".to_string(),
            Instruction::Last => "last".to_string(),  // NEW
            
            // Stack
            Instruction::Dup => "dup".to_string(),
            Instruction::Swap => "swap".to_string(),
            Instruction::Drop => "drop".to_string(),
            Instruction::Clear => "clear".to_string(),
            Instruction::Roll => "roll".to_string(),
            Instruction::Top => "top".to_string(),
            Instruction::Quote => "quote".to_string(),
            Instruction::Compose => "compose".to_string(),
            Instruction::Apply => "apply".to_string(),
            Instruction::Split => "split".to_string(),
            Instruction::Compress => "compress".to_string(),
            Instruction::Dip => "dip".to_string(),
            Instruction::Nop => "nop".to_string(),
            
            // Numbered
            Instruction::DupN(n) => format!("dup{}", n),
            Instruction::SwapN(n) => format!("swap{}", n),
            Instruction::DigN(n) => format!("dig{}", n),
            Instruction::InjectN(n) => format!("inject{}", n),
            Instruction::PInjectN(n) => format!("pinject{}", n),
            
            // Control
            Instruction::If => "if".to_string(),
            Instruction::Loop => "loop".to_string(),
            Instruction::Exit => "exit".to_string(),
            Instruction::Try => "try".to_string(),
            
            // Inner stack
            Instruction::NewStack => "stack".to_string(),
            Instruction::Push => "push".to_string(),
            Instruction::Pop => "pop".to_string(),
            Instruction::Inject => "inject".to_string(),
            
            // Type
            Instruction::GetType => "type".to_string(),
            Instruction::TypeInstr => "INSTR".to_string(),
            Instruction::TypeInt => "INT".to_string(),
            Instruction::TypeFloat => "FLOAT".to_string(),
            Instruction::TypeBool => "BOOL".to_string(),
            Instruction::TypeStr => "STR".to_string(),
            Instruction::TypeType => "TYPE".to_string(),
            Instruction::TypeNone => "NONE".to_string(),
            Instruction::TypeStack => "STACK".to_string(),
            
            // I/O
            Instruction::Print => "print".to_string(),
            Instruction::PrintAll => "printall".to_string(),
            
            // Bracket operations
            Instruction::IfCond(code) => format!("if({})", instructions_to_string(code)),
            Instruction::LoopCond(code) => format!("loop({})", instructions_to_string(code)),
            Instruction::Times(code) => format!("times({})", instructions_to_string(code)),
            Instruction::DupCond(code) => format!("dup({})", instructions_to_string(code)),
            Instruction::SwapCond(code) => format!("swap({})", instructions_to_string(code)),
            Instruction::DigCond(code) => format!("dig({})", instructions_to_string(code)),
            Instruction::SplitCond(code) => format!("split({})", instructions_to_string(code)),
            Instruction::ComposeCond(code) => format!("compose({})", instructions_to_string(code)),
            
            // String operations
            Instruction::Define(name) => format!("define({})", name),
            Instruction::Delete(name) => format!("delete({})", name),
            Instruction::IsDef(name) => format!("isdef({})", name),
            Instruction::Load(name) => format!("load({})", name),
            Instruction::Save(name) => format!("save({})", name),
            Instruction::Call(name) => name.to_string(),
        };
        parts.push(s);
    }
    
    parts.join(" ")
}

/// Split an instruction string into tokens (for split operation)
fn split_instruction(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_string = false;
    
    for ch in s.chars() {
        match ch {
            '"' if depth == 0 => {
                in_string = !in_string;
                current.push(ch);
                if !in_string && !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            '[' | '(' | '{' if !in_string => {
                depth += 1;
                current.push(ch);
            }
            ']' | ')' | '}' if !in_string => {
                depth -= 1;
                current.push(ch);
                if depth == 0 && !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            ' ' | '\t' | '\n' | '\r' if depth == 0 && !in_string => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }
    
    if !current.is_empty() {
        tokens.push(current);
    }
    
    tokens
}

/// Execute code from a string
pub fn execute_string(state: &mut ProgramState, code: &str) -> Result<()> {
    let instructions = compile(code)?;
    let mut interp = Interpreter::new(state);
    interp.execute(&instructions)
}

/// Execute code from a string (alias for execute_string)
pub fn execute(state: &mut ProgramState, code: &str) -> Result<()> {
    execute_string(state, code)
}

/// Load and execute code from a file
pub fn load_file(state: &mut ProgramState, filename: &str) -> Result<()> {
    let content = fs::read_to_string(filename)
        .map_err(|_| SscriptError::FileNotFound(filename.to_string()))?;
    execute_string(state, &content)
}
