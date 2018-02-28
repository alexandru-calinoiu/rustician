use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let value = rx.recv().unwrap();
        println!("got: {:?}", value);
    });

    let numbers = vec![1, 2, 3];

    tx.send(numbers).unwrap();

    println!("numbers: {:?}", numbers);
}