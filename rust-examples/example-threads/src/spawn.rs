use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("From spawned thread {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for _i in 0..5 {
        println!("Before...");
        thread::sleep(Duration::from_millis(500));
        println!("After...");
    }
    let _res = handle.join().unwrap();
    println!("End of program");
}
