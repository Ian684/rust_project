fn split_once(s:&str,at:char)->Option<(&str,&str)>{let i=s.find(at)?;Some((&s[..i],&s[i+at.len_utf8()..]))}fn main(){println!("{:?}",split_once("key=value",'='));}
