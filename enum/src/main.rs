enum Test {
    A4(u64),
    A6(u64),
    A8(u64)
}

fn route(_t: Test){
    match _t{
        Test::A4(v) => println!("{}" , v),
        Test::A6(v) => println!("{}" , v),
        Test::A8(v) => println!("{}" , v)
    }
    //println!("{:?}" , _t);
}

fn main() {

    let four: Test = Test::A4(4);
    let six: Test = Test::A6(6);
    let eight: Test = Test::A8(8);

    route(four);
    route(six);
    route(eight);
    



}
