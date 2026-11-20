use std::io;
fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .unwrap();
    println!("{}" , s);
    
    s = s.trim().to_string();
    println!("{}" , s);
    //one line str

    let n: i64 = s.trim().parse().unwrap();
    println!("{}" , n+5);
    //one line int

    let mut a: u64 = 1;
    let result: i64 = loop {
        if a < 1{
            println!("small");
        }
        else if a == 1{
            println!("middle");
            break 1111;
        }
        else{
            println!("big");
        }
    };
    println!("{}" , result);
    let mut i: u64 = 0;
    while true{
        if i >= 3{
            break;
        }
        println!("{}" , i);
        i += 1;
    }
    for i in 0..3{
        println!("{}" , i);
    }



}
