use std::{fs,env};fn main(){let root=env::args().nth(1).unwrap_or_else(||".".into());if let Ok(entries)=fs::read_dir(root){for e in entries.flatten(){println!("{}",e.path().display());}}}
