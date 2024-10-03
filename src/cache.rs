use bincode::{config, Decode, Encode};
use std::collections::HashMap;
use std::fs;

#[derive(Encode, Decode, PartialEq, Debug)]
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
            let attempts = self.attempts.get(key).unwrap_or(&0).clone();
            self.attempts.insert(key.to_string(), attempts + 1);
            self.save();
            if attempts > 2 {
                self.cache.remove(key);
                self.attempts.remove(key);
                self.save();
                println!("Too many attempts, cache removed");
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
        let config = config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(&*self, config).unwrap();
        fs::write("cache.bin", encoded).unwrap();
    }

    fn restore(&mut self) {
        if !std::path::Path::new("cache.bin").exists() {
            return;
        }
        let config = config::standard();
        let encoded = fs::read("cache.bin").unwrap();
        let (decoded, _): (Cache, usize) = bincode::decode_from_slice(&encoded, config).unwrap();
        *self = decoded;
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
