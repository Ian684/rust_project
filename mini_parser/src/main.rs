fn parse_pair(s:&str)->Option<(i32,i32)>{let mut it=s.split(',');Some((it.next()?.parse().ok()?,it.next()?.parse().ok()?))}fn main(){println!("{:?}",parse_pair("3,4"));}
