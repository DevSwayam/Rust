use core::time::Duration;
use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 0..16 {
            println!("Hi from thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap();

    for i in 0..10 {
        println!("Hi from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
