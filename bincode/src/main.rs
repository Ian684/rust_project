fn main(){let value:u16=0x1234;let bytes=value.to_le_bytes();println!("{:02x?} {}",bytes,u16::from_le_bytes(bytes));}
