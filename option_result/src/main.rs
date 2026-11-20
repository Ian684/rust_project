fn parse(s:&str)->Option<i32>{s.parse().ok()}fn main(){println!("{:?}",parse("42"));println!("{:?}",parse("bad"));}
