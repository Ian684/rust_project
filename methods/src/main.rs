struct Rectangle{w:u32,h:u32}impl Rectangle{fn area(&self)->u32{self.w*self.h}}fn main(){let r=Rectangle{w:4,h:3};println!("area={}",r.area());}
