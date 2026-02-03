//
// compiler.rs - Tokenizer and compiler
//

use crate::error::{Result, SscriptError};
use crate::instruction::Instruction;

/// Check if character is whitespace/indent
#[inline]
fn is_indent(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

/// Check if character is reserved
#[inline]
fn is_reserved(c: char) -> bool {
    is_indent(c) || matches!(c, '[' | ']' | '{' | '}' | '(' | ')' | '"')
}

/// Compiler that converts source code to instructions
pub struct Compiler<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Compiler { source, pos: 0 }
    }
    
    /// Compile source code into a vector of instructions
    pub fn compile(&mut self) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();
        
        while self.pos < self.source.len() {
            self.skip_whitespace();
            if self.pos >= self.source.len() {
                break;
            }
            
            let c = self.current_char();
            
            match c {
                '"' => {
                    let s = self.parse_string()?;
                    instructions.push(Instruction::PushString(s.into()));
                }
                '[' => {
                    let inner = self.parse_bracketed('[', ']')?;
                    let mut inner_compiler = Compiler::new(&inner);
                    let inner_instructions = inner_compiler.compile()?;
                    instructions.push(Instruction::PushQuoted(inner_instructions.into()));
                }
                '{' => {
                    let inner = self.parse_bracketed('{', '}')?;
                    let mut inner_compiler = Compiler::new(&inner);
                    let inner_instructions = inner_compiler.compile()?;
                    instructions.push(Instruction::PushStackLiteral(inner_instructions.into()));
                }
                '-' if self.peek_char().map_or(false, |c| c.is_ascii_digit()) => {
                    let instr = self.parse_number()?;
                    instructions.push(instr);
                }
                c if c.is_ascii_digit() => {
                    let instr = self.parse_number()?;
                    instructions.push(instr);
                }
                _ => {
                    let instr = self.parse_word()?;
                    instructions.push(instr);
                }
            }
        }
        
        Ok(instructions)
    }
    
    #[inline]
    fn current_char(&self) -> char {
        self.source[self.pos..].chars().next().unwrap()
    }
    
    #[inline]
    fn peek_char(&self) -> Option<char> {
        self.source[self.pos..].chars().nth(1)
    }
    
    fn skip_whitespace(&mut self) {
        while self.pos < self.source.len() {
            let c = self.current_char();
            if is_indent(c) {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }
    
    fn parse_string(&mut self) -> Result<String> {
        self.pos += 1; // skip opening "
        let start = self.pos;
        
        while self.pos < self.source.len() {
            let c = self.current_char();
            if c == '"' {
                let s = self.source[start..self.pos].to_string();
                self.pos += 1; // skip closing "
                return Ok(s);
            }
            self.pos += c.len_utf8();
        }
        
        Err(SscriptError::StringQuotingError)
    }
    
    fn parse_bracketed(&mut self, open: char, close: char) -> Result<String> {
        self.pos += 1; // skip opening bracket
        let start = self.pos;
        let mut depth = 1;
        
        while self.pos < self.source.len() {
            let c = self.current_char();
            if c == close {
                depth -= 1;
                if depth == 0 {
                    let s = self.source[start..self.pos].to_string();
                    self.pos += 1; // skip closing bracket
                    return Ok(s);
                }
            } else if c == open {
                depth += 1;
            }
            self.pos += c.len_utf8();
        }
        
        match close {
            ']' => Err(SscriptError::SquaredParenthesisError),
            '}' => Err(SscriptError::CurlyParenthesisError),
            ')' => Err(SscriptError::RoundParenthesisError),
            _ => unreachable!(),
        }
    }
    
    fn parse_number(&mut self) -> Result<Instruction> {
        let start = self.pos;
        let mut has_dot = false;
        
        // Handle negative sign
        if self.current_char() == '-' {
            self.pos += 1;
        }
        
        while self.pos < self.source.len() {
            let c = self.current_char();
            if c.is_ascii_digit() {
                self.pos += 1;
            } else if (c == '.' || c == ',') && !has_dot {
                has_dot = true;
                self.pos += 1;
            } else {
                break;
            }
        }
        
        let num_str = &self.source[start..self.pos];
        let num_str = num_str.replace(',', ".");
        
        if has_dot {
            let f: f64 = num_str.parse().map_err(|_| {
                SscriptError::ValueError(format!("Invalid float: {}", num_str))
            })?;
            Ok(Instruction::PushFloat(f))
        } else {
            let i: i64 = num_str.parse().map_err(|_| {
                SscriptError::ValueError(format!("Invalid integer: {}", num_str))
            })?;
            Ok(Instruction::PushInt(i))
        }
    }
    
    fn parse_word(&mut self) -> Result<Instruction> {
        let start = self.pos;
        let mut bracket_start = None;
        let mut num_start = None;
        
        while self.pos < self.source.len() {
            let c = self.current_char();
            
            if is_indent(c) {
                break;
            }
            
            if c == '(' {
                bracket_start = Some(self.pos);
                self.pos += 1;
                // Parse until closing paren
                while self.pos < self.source.len() && self.current_char() != ')' {
                    self.pos += self.current_char().len_utf8();
                }
                if self.pos >= self.source.len() {
                    return Err(SscriptError::RoundParenthesisError);
                }
                self.pos += 1; // skip )
                break;
            }
            
            // Check for numeric suffix (dup3, swap2, etc.)
            if c.is_ascii_digit() && num_start.is_none() {
                num_start = Some(self.pos);
            }
            
            self.pos += c.len_utf8();
        }
        
        let word = &self.source[start..self.pos];
        
        // Handle bracket instructions: instr(args)
        if let Some(bstart) = bracket_start {
            let name = &self.source[start..bstart];
            let args = &self.source[bstart + 1..self.pos - 1];
            return self.compile_bracket_instruction(name, args);
        }
        
        // Handle numbered instructions: dup3, swap2, dig1
        if let Some(nstart) = num_start {
            let name = &self.source[start..nstart];
            let num_str = &self.source[nstart..self.pos];
            if let Ok(num) = num_str.parse::<usize>() {
                if let Some(instr) = self.compile_numbered_instruction(name, num) {
                    return Ok(instr);
                }
            }
        }
        
        // Regular instruction
        self.compile_simple_instruction(word)
    }
    
    fn compile_simple_instruction(&self, word: &str) -> Result<Instruction> {
        match word {
            // Math
            "+" => Ok(Instruction::Add),
            "-" => Ok(Instruction::Sub),
            "*" => Ok(Instruction::Mul),
            "/" => Ok(Instruction::Div),
            "%" => Ok(Instruction::Mod),
            "pow" => Ok(Instruction::Pow),
            "sqrt" => Ok(Instruction::Sqrt),
            "exp" => Ok(Instruction::Exp),
            "log" => Ok(Instruction::Log),
            "log2" => Ok(Instruction::Log2),
            "log10" => Ok(Instruction::Log10),
            "--" => Ok(Instruction::Opposite),
            "!" => Ok(Instruction::Factorial),
            "gamma" => Ok(Instruction::Gamma),
            "sin" => Ok(Instruction::Sin),
            "cos" => Ok(Instruction::Cos),
            "tan" => Ok(Instruction::Tan),
            "arcsin" => Ok(Instruction::ArcSin),
            "arccos" => Ok(Instruction::ArcCos),
            "arctan" => Ok(Instruction::ArcTan),
            "sinh" => Ok(Instruction::Sinh),
            "cosh" => Ok(Instruction::Cosh),
            "tanh" => Ok(Instruction::Tanh),
            "arcsinh" => Ok(Instruction::ArcSinh),
            "arccosh" => Ok(Instruction::ArcCosh),
            "arctanh" => Ok(Instruction::ArcTanh),
            "int" => Ok(Instruction::ToInt),
            
            // Boolean
            "and" => Ok(Instruction::And),
            "or" => Ok(Instruction::Or),
            "xor" => Ok(Instruction::Xor),
            "not" => Ok(Instruction::Not),
            "true" => Ok(Instruction::True),
            "false" => Ok(Instruction::False),
            
            // Comparison
            "==" => Ok(Instruction::Eq),
            "!=" => Ok(Instruction::NotEq),
            "<" => Ok(Instruction::Lt),
            ">" => Ok(Instruction::Gt),
            "<=" => Ok(Instruction::LtEq),
            ">=" => Ok(Instruction::GtEq),
            
            // Stack
            "dup" | "dup0" => Ok(Instruction::Dup),
            "swap" | "swap1" => Ok(Instruction::Swap),
            "drop" => Ok(Instruction::Drop),
            "clear" => Ok(Instruction::Clear),
            "roll" => Ok(Instruction::Roll),
            "top" => Ok(Instruction::Top),
            "quote" => Ok(Instruction::Quote),
            "compose" => Ok(Instruction::Compose),
            "apply" => Ok(Instruction::Apply),
            "split" => Ok(Instruction::Split),
            "size" => Ok(Instruction::Size),
            "empty" => Ok(Instruction::Empty),
            "last" => Ok(Instruction::Last),  // NEW
            "compress" => Ok(Instruction::Compress),
            "dip" => Ok(Instruction::Dip),
            
            // Inner stack
            "stack" => Ok(Instruction::NewStack),
            "push" => Ok(Instruction::Push),
            "pop" => Ok(Instruction::Pop),
            "inject" => Ok(Instruction::Inject),
            
            // Type
            "type" => Ok(Instruction::GetType),
            "INSTR" => Ok(Instruction::TypeInstr),
            "INT" => Ok(Instruction::TypeInt),
            "FLOAT" => Ok(Instruction::TypeFloat),
            "BOOL" => Ok(Instruction::TypeBool),
            "STR" => Ok(Instruction::TypeStr),
            "TYPE" => Ok(Instruction::TypeType),
            "NONE" => Ok(Instruction::TypeNone),
            "STACK" => Ok(Instruction::TypeStack),
            
            // Control
            "if" => Ok(Instruction::If),
            "loop" => Ok(Instruction::Loop),
            "nop" => Ok(Instruction::Nop),
            "exit" => Ok(Instruction::Exit),
            "try" => Ok(Instruction::Try),
            
            // I/O
            "print" => Ok(Instruction::Print),
            "printall" => Ok(Instruction::PrintAll),
            
            // None value
            "none" => Ok(Instruction::PushNone),
            
            // swap0 is nop
            "swap0" => Ok(Instruction::Nop),
            
            // User-defined or unknown
            _ => Ok(Instruction::Call(word.into())),
        }
    }
    
    fn compile_numbered_instruction(&self, name: &str, num: usize) -> Option<Instruction> {
        match name {
            "dup" => Some(Instruction::DupN(num)),
            "swap" => Some(Instruction::SwapN(num)),
            "dig" => Some(Instruction::DigN(num)),
            "inject" => Some(Instruction::InjectN(num)),
            "pinject" => Some(Instruction::PInjectN(num)),
            _ => None,
        }
    }
    
    fn compile_bracket_instruction(&mut self, name: &str, args: &str) -> Result<Instruction> {
        match name {
            "define" => {
                // Validate name
                for c in args.chars() {
                    if is_reserved(c) {
                        return Err(SscriptError::InvalidNameDefine(args.to_string()));
                    }
                }
                Ok(Instruction::Define(args.into()))
            }
            "delete" => Ok(Instruction::Delete(args.into())),
            "isdef" => Ok(Instruction::IsDef(args.into())),
            "load" => Ok(Instruction::Load(args.into())),
            "save" => Ok(Instruction::Save(args.into())),
            "if" => {
                let mut compiler = Compiler::new(args);
                let cond = compiler.compile()?;
                Ok(Instruction::IfCond(cond.into()))
            }
            "loop" => {
                let mut compiler = Compiler::new(args);
                let cond = compiler.compile()?;
                Ok(Instruction::LoopCond(cond.into()))
            }
            "times" => {
                let mut compiler = Compiler::new(args);
                let count = compiler.compile()?;
                Ok(Instruction::Times(count.into()))
            }
            "dup" => {
                let mut compiler = Compiler::new(args);
                let idx = compiler.compile()?;
                Ok(Instruction::DupCond(idx.into()))
            }
            "swap" => {
                let mut compiler = Compiler::new(args);
                let idx = compiler.compile()?;
                Ok(Instruction::SwapCond(idx.into()))
            }
            "dig" => {
                let mut compiler = Compiler::new(args);
                let idx = compiler.compile()?;
                Ok(Instruction::DigCond(idx.into()))
            }
            "split" => {
                let mut compiler = Compiler::new(args);
                let delim = compiler.compile()?;
                Ok(Instruction::SplitCond(delim.into()))
            }
            "compose" => {
                let mut compiler = Compiler::new(args);
                let delim = compiler.compile()?;
                Ok(Instruction::ComposeCond(delim.into()))
            }
            _ => {
                // Unknown bracket instruction - could be user-defined
                // Try to compile args and make a call
                let mut compiler = Compiler::new(args);
                let arg_instrs = compiler.compile()?;
                
                // For user-defined functions with brackets, we push the compiled args
                // as a quoted instruction and then call the function
                if arg_instrs.is_empty() {
                    Ok(Instruction::Call(name.into()))
                } else {
                    // This is a bracket call to a user-defined function
                    // We need to handle this specially in the interpreter
                    Err(SscriptError::InvalidInstruction(format!(
                        "{}({})",
                        name, args
                    )))
                }
            }
        }
    }
}

/// Convenience function to compile source code
pub fn compile(source: &str) -> Result<Vec<Instruction>> {
    let mut compiler = Compiler::new(source);
    compiler.compile()
}
