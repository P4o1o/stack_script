//
// instruction.rs - Compiled instruction representation (bytecode)
//

use std::rc::Rc;

/// Compiled instruction representation
/// This is the "bytecode" that gets executed by the VM
#[derive(Debug, Clone)]
pub enum Instruction {
    // === Literals ===
    PushInt(i64),
    PushFloat(f64),
    PushBool(bool),
    PushNone,
    PushString(Rc<str>),
    PushQuoted(Rc<[Instruction]>),
    PushStackLiteral(Rc<[Instruction]>),
    
    // === Built-in operations (no arguments) ===
    // Math
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Sqrt,
    Exp,
    Log,
    Log2,
    Log10,
    Opposite,  // --
    Factorial, // !
    Gamma,
    Sin,
    Cos,
    Tan,
    ArcSin,
    ArcCos,
    ArcTan,
    Sinh,
    Cosh,
    Tanh,
    ArcSinh,
    ArcCosh,
    ArcTanh,
    ToInt,
    
    // Boolean
    And,
    Or,
    Xor,
    Not,
    True,
    False,
    
    // Comparison
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    
    // Stack operations
    Dup,
    Swap,
    Drop,
    Clear,
    Roll,
    Top,
    Quote,
    Compose,
    Apply,
    Split,
    Size,
    Empty,
    Last,  // NEW: checks if stack has exactly one element
    Compress,
    Dip,
    
    // Inner stack
    NewStack,
    Push,
    Pop,
    Inject,
    
    // Type operations
    GetType,
    TypeInstr,
    TypeInt,
    TypeFloat,
    TypeBool,
    TypeStr,
    TypeType,
    TypeNone,
    TypeStack,
    
    // Control
    If,
    Loop,
    Nop,
    Exit,
    Try,
    
    // I/O
    Print,
    PrintAll,
    
    // === Operations with numeric argument ===
    DupN(usize),
    SwapN(usize),
    DigN(usize),
    InjectN(usize),
    PInjectN(usize),
    
    // === Operations with code argument (bracket operations) ===
    IfCond(Rc<[Instruction]>),
    LoopCond(Rc<[Instruction]>),
    Times(Rc<[Instruction]>),
    DupCond(Rc<[Instruction]>),
    SwapCond(Rc<[Instruction]>),
    DigCond(Rc<[Instruction]>),
    SplitCond(Rc<[Instruction]>),
    ComposeCond(Rc<[Instruction]>),
    
    // === Operations with string argument ===
    Define(Rc<str>),
    Delete(Rc<str>),
    IsDef(Rc<str>),
    Load(Rc<str>),
    Save(Rc<str>),
    
    // === User-defined function call ===
    Call(Rc<str>),
}
