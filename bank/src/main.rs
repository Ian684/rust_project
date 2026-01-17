struct User{
    name: String,
    age: u64,
}

impl User{
    fn print_age(&self){
        println!("{} -> age: {}" , self.name , self.age);

    }
    fn adjust_age(&mut self , ad_age: u64){
        println!("{} -> original age: {}" , self.name , self.age);
        self.age = ad_age;
        println!("{} -> age: {}" , self.name , self.age);
    }
}

fn main() {
    let mut user: User = User{
        name: "ian".to_string(),
        age: 1000,
    };
    user.print_age();
    user.adjust_age(20);
}
