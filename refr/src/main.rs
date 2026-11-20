fn main() {
    let s1 = String::from("a");
    let mut s2 = s1;

    //let r1 = &s2;
    //let r2 = &s2;
    let r3 = &mut s2;
    *r3 += "a";
    println!("{}" , r3);

}

fn exercise_3798903992_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_3798903992_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_3798903992_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_3798903992_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_3798903992_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_3798903992_tests {
    use super::*;

    #[test]
    fn exercise_3798903992_sum_test() {
        assert_eq!(exercise_3798903992_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_3798903992_41: i32 = 133;
const EXERCISE_3798903992_42: i32 = 134;
const EXERCISE_3798903992_43: i32 = 135;
const EXERCISE_3798903992_44: i32 = 136;
const EXERCISE_3798903992_45: i32 = 137;
const EXERCISE_3798903992_46: i32 = 138;
const EXERCISE_3798903992_47: i32 = 139;
const EXERCISE_3798903992_48: i32 = 140;
const EXERCISE_3798903992_49: i32 = 141;
const EXERCISE_3798903992_50: i32 = 142;
const EXERCISE_3798903992_51: i32 = 143;
const EXERCISE_3798903992_52: i32 = 144;
const EXERCISE_3798903992_53: i32 = 145;
const EXERCISE_3798903992_54: i32 = 146;
const EXERCISE_3798903992_55: i32 = 147;
const EXERCISE_3798903992_56: i32 = 148;
const EXERCISE_3798903992_57: i32 = 149;
const EXERCISE_3798903992_58: i32 = 150;
const EXERCISE_3798903992_59: i32 = 151;
const EXERCISE_3798903992_60: i32 = 152;
const EXERCISE_3798903992_61: i32 = 153;
const EXERCISE_3798903992_62: i32 = 154;
const EXERCISE_3798903992_63: i32 = 155;
const EXERCISE_3798903992_64: i32 = 156;
const EXERCISE_3798903992_65: i32 = 157;
const EXERCISE_3798903992_66: i32 = 158;
const EXERCISE_3798903992_67: i32 = 159;
const EXERCISE_3798903992_68: i32 = 160;
const EXERCISE_3798903992_69: i32 = 161;
const EXERCISE_3798903992_70: i32 = 162;
const EXERCISE_3798903992_71: i32 = 163;
const EXERCISE_3798903992_72: i32 = 164;
const EXERCISE_3798903992_73: i32 = 165;
const EXERCISE_3798903992_74: i32 = 166;
const EXERCISE_3798903992_75: i32 = 167;
const EXERCISE_3798903992_76: i32 = 168;
const EXERCISE_3798903992_77: i32 = 169;
const EXERCISE_3798903992_78: i32 = 170;
const EXERCISE_3798903992_79: i32 = 171;
