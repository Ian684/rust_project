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

fn exercise_1132583978_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_1132583978_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_1132583978_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_1132583978_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_1132583978_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_1132583978_tests {
    use super::*;

    #[test]
    fn exercise_1132583978_sum_test() {
        assert_eq!(exercise_1132583978_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_1132583978_55: i32 = 133;
const EXERCISE_1132583978_56: i32 = 134;
const EXERCISE_1132583978_57: i32 = 135;
const EXERCISE_1132583978_58: i32 = 136;
const EXERCISE_1132583978_59: i32 = 137;
const EXERCISE_1132583978_60: i32 = 138;
const EXERCISE_1132583978_61: i32 = 139;
const EXERCISE_1132583978_62: i32 = 140;
const EXERCISE_1132583978_63: i32 = 141;
const EXERCISE_1132583978_64: i32 = 142;
const EXERCISE_1132583978_65: i32 = 143;
const EXERCISE_1132583978_66: i32 = 144;
const EXERCISE_1132583978_67: i32 = 145;
const EXERCISE_1132583978_68: i32 = 146;
const EXERCISE_1132583978_69: i32 = 147;
const EXERCISE_1132583978_70: i32 = 148;
const EXERCISE_1132583978_71: i32 = 149;
const EXERCISE_1132583978_72: i32 = 150;
const EXERCISE_1132583978_73: i32 = 151;
const EXERCISE_1132583978_74: i32 = 152;
const EXERCISE_1132583978_75: i32 = 153;
const EXERCISE_1132583978_76: i32 = 154;
const EXERCISE_1132583978_77: i32 = 155;
const EXERCISE_1132583978_78: i32 = 156;
const EXERCISE_1132583978_79: i32 = 157;
