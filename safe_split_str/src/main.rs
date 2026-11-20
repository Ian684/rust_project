fn split_once(s: &str, delim: char) -> Option<(&str, &str)>{
    for (i,c) in s.char_indices() {
        if c == delim{
            return Some((&s[..i] , &s[i + c.len_utf8()..]));
        }
    }
    None
}


fn main() {
    match split_once("aaaaabbcbbb" , 'c'){
        Some((x, y)) => println!("{} {}", x, y),
        None => println!("no"),
    }
}
