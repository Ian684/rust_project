use std::rc::Rc;fn main(){let p=Rc::new(String::from("shared"));println!("{} refs={}",p,Rc::strong_count(&p));}
