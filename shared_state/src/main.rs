use std::sync::{Arc,Mutex};fn main(){let n=Arc::new(Mutex::new(0));*n.lock().unwrap()+=7;println!("{}",*n.lock().unwrap());}
