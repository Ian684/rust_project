use std::sync::{Arc,Mutex};fn main(){let value=Arc::new(Mutex::new(0));*value.lock().unwrap()+=1;println!("{}",value.lock().unwrap());}
