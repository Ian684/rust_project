fn divide(a:i32,b:i32)->Result<i32,String>{if b==0{Err("division by zero".into())}else{Ok(a/b)}}fn main(){println!("{:?}",divide(8,2));println!("{:?}",divide(8,0));}
