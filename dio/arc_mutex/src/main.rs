use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Hello, world!");

    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let contador_clone = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            let mut num = contador_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Contador: {}", *contador.lock().unwrap());
}
