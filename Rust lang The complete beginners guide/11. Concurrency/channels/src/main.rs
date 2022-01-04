use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const NUM_THREADS: usize = 20;

fn start_thread(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("Setting Timer {}", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("Sending {}", d);
        tx.send(d).unwrap();
    });
}

fn main() {
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    //     tx.send(42).unwrap();
    // });
    // println!("Received {}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_THREADS {
        start_thread(i, tx.clone());
    }

    for i in rx.iter().take(NUM_THREADS) {
        println!("Received {}", i);
    }
}
