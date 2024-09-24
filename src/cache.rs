use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    cache: HashMap<String, String>,
    attempts: HashMap<String, u32>,
}

impl Cache {
    pub fn new() -> Cache {
        let mut cache = Cache {
            cache: HashMap::new(),
            attempts: HashMap::new(),
        };
        cache.restore();
        cache
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        if self.cache.contains_key(key) {
            let attempts = self.attempts.get(key).or_else(|| Some(&0)).unwrap().clone();
            self.attempts.insert(key.to_string(), attempts + 1);
            self.save();
            if attempts > 2 {
                self.cache.remove(key);
                self.attempts.remove(key);
                self.save();
                println!("To many attempts cache removed");
                println!("================");
                return None;
            }
        }
        self.cache.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.cache.insert(key, value);
        self.save();
    }

    fn save(&mut self) {
        let json = serde_json::to_string(&self.cache).unwrap();
        std::fs::write("cache.json", json).unwrap();
    }

    fn restore(&mut self) {
        if !std::path::Path::new("cache.json").exists() {
            return;
        }
        let json = std::fs::read_to_string("cache.json").unwrap();
        self.cache = serde_json::from_str(&json).unwrap();
    }
}

mod tests {
    #[test]
    fn test_cache() {
        let mut cache = super::Cache::new();
        cache.set("key".to_string(), "value".to_string());
        assert_eq!(cache.get("key").unwrap(), "value");
        assert_eq!(cache.get("key").unwrap(), "value");
        assert_eq!(cache.get("key").unwrap(), "value");
        assert_eq!(cache.get("key"), None);
    }
}
