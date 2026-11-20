fn hello(x: i32 , y: u64 , z: f64) -> f64{
    println!("{} {} {}" , x , y , z);
    (x + y as i32) as f64 * z
}
fn main() {
    let x: i32 = -43;
    let y: u64 = 302;
    let z: f64 = 2.53;
    let a: bool = true;
    let b: String = "aaa".to_string();
    println!("signed: {}" , x);
    println!("unsigned: {}" , y);
    println!("float: {}" , z);
    println!("bool: {}" , a);
    println!("String: {}" , b);
    

    let arr: [i64;5] = [1,2,3,4,5];
    let brr: [&str;3] = ["a" , "b" , "c"];
    println!("{:?}" , arr);
    println!("{}" , brr[0]);
    println!("{}" , brr[1]);
    println!("{}" , brr[2]);


    let tuple1: (String , i64 , bool) = ("a".to_string() , 43 , true);
    println!("{:?}" , tuple1);
    let tuple2 = ("b" , 242 , false , [1,3,34,4]);
    println!("{:?}" , tuple2);


    let slice1: &[i64] = &[1,34,4,4,4];
    println!("{:?}" , slice1);
    let slice2: &[&str] = &["a" , "b" , "c"];
    println!("{:?}" , slice2);
    let slice3: &[&String] = &[&"a".to_string() , &"b".to_string() , &"c".to_string()];
    println!("{:?}" , slice3);
    //str &str String difference
    

    let c: f64 = hello(x , y , z);
    print!("{}\n" , c);


    
}
