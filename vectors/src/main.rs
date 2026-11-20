fn main(){let mut v=vec![1,2,3];v.push(4);v.retain(|n|n%2==0);println!("{:?}",v);}
