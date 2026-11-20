use std::collections::HashMap;fn main(){let mut f=HashMap::new();for c in "banana".chars(){*f.entry(c).or_insert(0)+=1;}println!("{:?}",f);}
