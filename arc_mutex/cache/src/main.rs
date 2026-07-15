use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Cache {
    data: HashMap<String, i32>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }
    fn insert(&mut self, key: String, value: i32) {
        self.data.insert(key, value);
    }
    fn get(&self, key: &str) -> Option<i32> {
        self.data.get(key).copied()
    }
}

fn main() {
    let cache = Arc::new(Mutex::new(Cache::new()));
    let mut handles = vec![];

    // Threads de escrita
    for i in 0..3 {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            let mut cache = cache.lock().unwrap();
            cache.insert(format!("key{}", i), i);
            println!("Thread {} escreveu key{}", i, i);
        }));
    } 
    // Thread de leitura
    let cache_reader = Arc::clone(&cache);
    handles.push(thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(100));
        // Aguarda escritas
        let cache = cache_reader.lock().unwrap();
        println!("Leitura: key1 = {:?}", cache.get("key1"));
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    let cache = cache.lock().unwrap();
    println!("Cache final: {:?}", cache.data);
}