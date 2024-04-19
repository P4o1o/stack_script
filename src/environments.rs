use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) trait Environment{
    fn get(&self, key: &str) -> Option<String>;

    fn set(&mut self, key: &str, val: String);

    fn contains(&self, key: &str) -> bool;

    fn remove(&mut self, key: &str);

    fn clear(&mut self);

}

pub(crate) struct BaseEnv{
    map: HashMap<String, String>
}

impl BaseEnv{
    pub fn new() -> Self{
        BaseEnv{ map: HashMap::new()}
    }
}

impl Environment for BaseEnv{
    fn get(&self, key: &str) -> Option<String> {
        match self.map.get(key){
            None => None,
            Some(val) => Some(val.clone())
        }
    }

    fn set(&mut self, key: &str, val: String) {
        self.map.insert(key.to_string(), val);
    }

    fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    fn remove(&mut self, key: &str){
        self.map.remove(key);
    }

    fn clear(&mut self) {
        self.map.clear()
    }
}

pub(crate) struct ThreadSafeEnv{
    map: Arc<Mutex<HashMap<String, String>>>
}

impl ThreadSafeEnv{
    pub fn new() -> Self{
        ThreadSafeEnv{ map: Arc::new(Mutex::new(HashMap::new()))}
    }
}

impl Environment for ThreadSafeEnv{
    fn get(&self, key: &str) -> Option<String> {
        let guard = self.map.lock().unwrap();
        match guard.get(key){
            None => None,
            Some(val) => Some(val.clone())
        }
    }

    fn set(&mut self, key: &str, val: String) {
        self.map.lock().unwrap().insert(key.to_string(), val);
    }

    fn contains(&self, key: &str) -> bool {
        self.map.lock().unwrap().contains_key(key)
    }

    fn remove(&mut self, key: &str){
        self.map.lock().unwrap().remove(key);
    }

    fn clear(&mut self) {
        self.map.lock().unwrap().clear()
    }
}