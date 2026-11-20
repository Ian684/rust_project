fn main(){let mut parts="x=12".split('=');let key=parts.next().unwrap();let value=parts.next().unwrap_or("0").parse::<i32>().unwrap_or(0);println!("{}={}",key,value);}
