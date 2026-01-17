fn main() {
    let s1 = String::from("a");
    let mut s2 = s1;

    //let r1 = &s2;
    //let r2 = &s2;
    let r3 = &mut s2;
    *r3 += "a";
    println!("{}" , r3);

}
