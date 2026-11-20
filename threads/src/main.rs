use std::thread;

fn main() {
    let workers: Vec<_> = (0..4)
        .map(|id| thread::spawn(move || id * id))
        .collect();

    for worker in workers {
        println!("worker result={}", worker.join().expect("worker panicked"));
    }
}
