use std::thread;
use std::time::Duration;

fn main() {
    let mut threads = vec![];

    for i in 0..10 {
        let th = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 1000));
            println!("New Thread {}", i);
        });

        threads.push(th);
    }

    for t in threads {
        t.join();
    }

    println!("Main Thread");
}
