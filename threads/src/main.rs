use std::thread;fn main(){let h=thread::spawn(||(1..=5).sum::<i32>());println!("sum={}",h.join().unwrap());}
