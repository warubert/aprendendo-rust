use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting main thread");

    let handle1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("hi number {} from the spawned thread ONE!", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("hi number {} from the spawned thread TWO!", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    println!("Waiting for threads to finish...");

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("All threads have finished");
}
