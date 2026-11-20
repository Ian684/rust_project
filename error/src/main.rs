//error handle
//approach 1

/* 
enum Option<T>{
    Some(T),
    None,
}
*/

fn divide(num: f64, deno: f64) -> Option<f64>{
    if deno == 0.0{
        None
    }else{
        Some(num/deno)
    }
}

//approach 2

/*
enum Result<T , E>{
    Ok(T),
    Err(E),
}
*/

fn divide1(num: f64, deno: f64) -> Result<f64 , String>{
    if deno == 0.0{
        Err("cannot divide by zero!".to_string())
    }else{
        Ok(num/deno)
    }
}

fn main() {
    let result: Option<f64> = divide(10.0, 0.0);
    match result{
        Some(x) => println!("{}" , x),
        None => println!("connot divide by zero!"),
    }


    let result1: Result<f64 , String> = divide1(10.0, 0.0);
    match result1{
        Ok(x) => println!("{}" , x),
        Err(y) => println!("{}" , y),
    }
}
