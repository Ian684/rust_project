fn main() {

    //vector
    let mut _arr: Vec<i32> = vec![1,2,3];
    //let mut _arr: Vec<i32> = Vec::new()
    _arr.push(4);
    _arr.push(4);
    _arr.push(4);
    _arr.push(4);
    _arr.pop();

    let x = _arr[0];
    println!("{}" , x);
    let y = _arr.get(0);
    match y{
        Some(x) => println!("{}" , x),
        None => println!("out of range"),
    }


    //utf8 string
    
    let s = String::from("你");
    //not use (let c = s[0];)
    for ch in s.chars(){
        println!("{}" , ch);
    }
    for b in s.bytes(){
        println!("{}" , b);
    }


    //hash map
    
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.remove(&"b");

    println!("{}" , map["a"]);
    match map.get("a"){
        Some(x) => println!("{}", x),
        None => println!("not found"),
    }


}
