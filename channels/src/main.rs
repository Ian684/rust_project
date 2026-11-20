use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        for value in [10, 20, 30] {
            sender.send(value).expect("receiver dropped");
        }
    });

    let total: i32 = receiver.iter().sum();
    println!("total={total}");
}
