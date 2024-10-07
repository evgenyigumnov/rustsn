use bincode::{config, Decode, Encode};
use std::collections::HashMap;
use std::fs;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Cache {
    cache: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Cache {
        let mut cache = Cache {
            cache: HashMap::new(),
        };
        cache.restore();
        cache
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
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
