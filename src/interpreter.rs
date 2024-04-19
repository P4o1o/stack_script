use crate::stack::{Stack, Errors};
use crate::environments::{BaseEnv, ThreadSafeEnv};

pub(crate) struct BaseInterpreter{
    stack: Stack,
    env: BaseEnv
}

impl BaseInterpreter {
    pub(crate) fn new() -> Self{
        BaseInterpreter{
            stack: Stack::new(),
            env: BaseEnv::new(),
        }
    }

    pub(crate) fn execute(&mut self, program: String) -> Result<(), Errors>{
        self.stack.execute::<BaseEnv>(program, &mut self.env)
    }
}

use std::sync::{Arc, Mutex};

pub(crate) struct ThreadSafeInterpreter {
    stack: Arc<Mutex<Stack>>,
    env: ThreadSafeEnv,
}

impl ThreadSafeInterpreter {
    pub(crate) fn new() -> Self {
        ThreadSafeInterpreter {
            stack: Arc::new(Mutex::new(Stack::new())),
            env: ThreadSafeEnv::new(),
        }
    }

    pub(crate) fn execute(&mut self, program: String) -> Result<(), Errors> {
        let mut stack = self.stack.lock().unwrap();
        stack.execute::<ThreadSafeEnv>(program, &mut self.env)
    }
}
