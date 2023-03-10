use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    let handle1 = thread::spawn(move || {
        println!("{:?}", v);
    });
    
    handle1.join().unwrap();

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned therad!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("he number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1))
    }
}
