//
// stack.rs - Stack data structures
//

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::borrow::BorrowMut;

use crate::primitives::{BOOL, NONE, TYPES};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElemType {
    Instruction = 0,
    Integer = 1,
    Floating = 2,
    Boolean = 3,
    String = 4,
    Type = 5,
    None = 6,
    InnerStack = 7,
}

impl ElemType {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        TYPES[*self as usize]
    }
    
    pub fn from_index(idx: usize) -> Option<ElemType> {
        match idx {
            0 => Some(ElemType::Instruction),
            1 => Some(ElemType::Integer),
            2 => Some(ElemType::Floating),
            3 => Some(ElemType::Boolean),
            4 => Some(ElemType::String),
            5 => Some(ElemType::Type),
            6 => Some(ElemType::None),
            7 => Some(ElemType::InnerStack),
            _ => None,
        }
    }
}

impl fmt::Display for ElemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub enum StackElem {
    Instruction(Rc<str>),
    Integer(i64),
    Floating(f64),
    Boolean(bool),
    String(Rc<str>),
    Type(ElemType),
    None,
    InnerStack(Rc<RefCell<Stack>>),
}

impl StackElem {
    #[inline]
    pub fn elem_type(&self) -> ElemType {
        match self {
            StackElem::Instruction(_) => ElemType::Instruction,
            StackElem::Integer(_) => ElemType::Integer,
            StackElem::Floating(_) => ElemType::Floating,
            StackElem::Boolean(_) => ElemType::Boolean,
            StackElem::String(_) => ElemType::String,
            StackElem::Type(_) => ElemType::Type,
            StackElem::None => ElemType::None,
            StackElem::InnerStack(_) => ElemType::InnerStack,
        }
    }
    
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            StackElem::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    #[inline]
    pub fn as_int(&self) -> Option<i64> {
        match self {
            StackElem::Integer(i) => Some(*i),
            _ => None,
        }
    }
    
    #[inline]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            StackElem::Floating(f) => Some(*f),
            _ => None,
        }
    }
    
    #[inline]
    pub fn to_numeric(&self) -> Option<f64> {
        match self {
            StackElem::Integer(i) => Some(*i as f64),
            StackElem::Floating(f) => Some(*f),
            _ => None,
        }
    }
    
    #[inline]
    pub fn as_instruction(&self) -> Option<&Rc<str>> {
        match self {
            StackElem::Instruction(s) => Some(s),
            _ => None,
        }
    }
    
    #[inline]
    pub fn as_string(&self) -> Option<&Rc<str>> {
        match self {
            StackElem::String(s) => Some(s),
            _ => None,
        }
    }
    
    #[inline]
    pub fn as_inner_stack(&self) -> Option<&Rc<RefCell<Stack>>> {
        match self {
            StackElem::InnerStack(s) => Some(s),
            _ => None,
        }
    }
}

impl PartialEq for StackElem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StackElem::Integer(a), StackElem::Integer(b)) => a == b,
            (StackElem::Floating(a), StackElem::Floating(b)) => a == b,
            (StackElem::Integer(a), StackElem::Floating(b)) => (*a as f64) == *b,
            (StackElem::Floating(a), StackElem::Integer(b)) => *a == (*b as f64),
            (StackElem::Boolean(a), StackElem::Boolean(b)) => a == b,
            (StackElem::String(a), StackElem::String(b)) => a == b,
            (StackElem::Instruction(a), StackElem::Instruction(b)) => a == b,
            (StackElem::Type(a), StackElem::Type(b)) => a == b,
            (StackElem::None, StackElem::None) => true,
            (StackElem::InnerStack(a), StackElem::InnerStack(b)) => {
                *a.borrow() == *b.borrow()
            }
            _ => false,
        }
    }
}

impl fmt::Display for StackElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackElem::Instruction(s) => write!(f, "[ {} ]", s),
            StackElem::Integer(i) => write!(f, "{}", i),
            StackElem::Floating(fl) => write!(f, "{}", fl),
            StackElem::Boolean(b) => write!(f, "{}", BOOL[*b as usize]),
            StackElem::String(s) => write!(f, "\"{}\"", s),
            StackElem::Type(t) => write!(f, "{}", t),
            StackElem::None => write!(f, "{}", NONE),
            StackElem::InnerStack(stack) => {
                write!(f, "{{ ")?;
                let stack = stack.borrow();
                for (i, elem) in stack.content.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, " }}")
            }
        }
    }
}

/// The main stack data structure
#[derive(Debug, Clone)]
pub struct Stack {
    pub content: Vec<StackElem>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            content: Vec::with_capacity(256),
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Stack {
            content: Vec::with_capacity(capacity),
        }
    }
    
    #[inline]
    pub fn push(&mut self, elem: StackElem) {
        self.content.push(elem);
    }
    
    #[inline]
    pub fn pop(&mut self) -> Option<StackElem> {
        self.content.pop()
    }
    
    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    
    #[inline]
    pub fn clear(&mut self) {
        self.content.clear();
    }
    
    #[inline]
    pub fn get(&self, index: usize) -> Option<&StackElem> {
        self.content.get(index)
    }
    
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut StackElem> {
        self.content.get_mut(index)
    }
    
    #[inline]
    pub fn last(&self) -> Option<&StackElem> {
        self.content.last()
    }
    
    #[inline]
    pub fn last_mut(&mut self) -> Option<&mut StackElem> {
        self.content.last_mut()
    }
    
    #[inline]
    pub fn first(&self) -> Option<&StackElem> {
        self.content.first()
    }
    
    /// Peek at the top element without removing it (alias for last)
    #[inline]
    pub fn peek(&self) -> Option<&StackElem> {
        self.content.last()
    }
    
    /// Peek at the top element mutably without removing it (alias for last_mut)
    #[inline]
    pub fn peek_mut(&mut self) -> Option<&mut StackElem> {
        self.content.last_mut()
    }
    
    #[inline]
    pub fn swap(&mut self, i: usize, j: usize) {
        self.content.swap(i, j);
    }
    
    /// Swap the top element with the element at index n from the top
    /// swap_top(1) swaps top with second element (same as regular swap)
    /// swap_top(2) swaps top with third element
    /// Returns false if stack is too small
    #[inline]
    pub fn swap_top(&mut self, n: usize) -> bool {
        let len = self.content.len();
        if n == 0 || n >= len {
            return n == 0; // swap with self is always OK
        }
        self.content.swap(len - 1, len - 1 - n);
        true
    }
    
    /// Roll: move top element to bottom, shift everything else up
    #[inline]
    pub fn roll(&mut self) {
        if let Some(top) = self.content.pop() {
            self.content.insert(0, top);
        }
    }
    
    /// Dig: bring element at depth n to top, shifting others down
    /// dig(0) is a no-op
    /// dig(1) is equivalent to swap
    /// dig(2) brings the third element to top
    #[inline]
    pub fn dig(&mut self, n: usize) -> bool {
        let len = self.content.len();
        if n >= len {
            return false;
        }
        if n == 0 {
            return true;
        }
        let idx = len - 1 - n;
        let elem = self.content.remove(idx);
        self.content.push(elem);
        true
    }
    
    #[inline]
    pub fn remove(&mut self, index: usize) -> StackElem {
        self.content.remove(index)
    }
    
    #[inline]
    pub fn insert_front(&mut self, elem: StackElem) {
        self.content.insert(0, elem);
    }
    
    pub fn print_top(&self, count: usize) {
        let start = if count > self.len() { 0 } else { self.len() - count };
        for i in start..self.len() {
            println!("{}", self.content[i]);
        }
    }

    #[inline]
    pub fn append_stack_consume(&mut self, other: &mut Stack) {
        let s_borrow = self.borrow_mut();
        // Move all elements from other to self, preserving order
        s_borrow.content.extend(other.content.drain(..));
    }
    
    /// Deep clone for inner stacks (needed when duplicating stacks)
    pub fn deep_clone(&self) -> Stack {
        Stack {
            content: self.content.iter().map(|elem| {
                match elem {
                    StackElem::InnerStack(s) => {
                        StackElem::InnerStack(Rc::new(RefCell::new(s.borrow().deep_clone())))
                    }
                    other => other.clone(),
                }
            }).collect(),
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Stack {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}
