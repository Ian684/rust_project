trait Area{fn area(&self)->u32;}struct Rect(u32,u32);impl Area for Rect{fn area(&self)->u32{self.0*self.1}}fn main(){println!("{}",Rect(4,5).area());}
