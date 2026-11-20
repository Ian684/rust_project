use std::collections::VecDeque;fn main(){let mut q=VecDeque::from([1,2,3]);q.push_back(4);while let Some(v)=q.pop_front(){print!("{v} ");}println!();}
