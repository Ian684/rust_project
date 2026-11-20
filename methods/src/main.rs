struct Counter(u32);impl Counter{fn next(&mut self)->u32{self.0+=1;self.0}}fn main(){let mut c=Counter(0);println!("{}",c.next());}
