use std::collections::HashMap;

fn char_freq(ss: &str) -> std::collections::HashMap<char, usize> {
    let mut count: HashMap<char , usize> = HashMap::new();
    for s in ss.chars() {
        *count.entry(s).or_insert(0) += 1;
    }
    count
}



fn main() {
    let st: &str = "aaaaabbbccc";
    let result: std::collections::HashMap<char, usize> = char_freq(st);
    for (k , v) in result.iter() {
        println!("{} {}" , k , v);
    }
}
