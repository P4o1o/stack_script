//
// environment.rs - Environment for user-defined instructions
//

use rustc_hash::FxHashMap;
use std::rc::Rc;

use crate::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Environment {
    content: FxHashMap<Rc<str>, Rc<[Instruction]>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            content: FxHashMap::default(),
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Environment {
            content: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }
    
    #[inline]
    pub fn set(&mut self, key: Rc<str>, value: Rc<[Instruction]>) {
        self.content.insert(key, value);
    }
    
    #[inline]
    pub fn get(&self, key: &str) -> Option<&Rc<[Instruction]>> {
        self.content.get(key)
    }
    
    #[inline]
    pub fn remove(&mut self, key: &str) -> bool {
        self.content.remove(key).is_some()
    }
    
    #[inline]
    pub fn contains(&self, key: &str) -> bool {
        self.content.contains_key(key)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
