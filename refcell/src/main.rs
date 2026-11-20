use std::cell::RefCell;fn main(){let x=RefCell::new(5);*x.borrow_mut()+=3;println!("{}",x.borrow());}
