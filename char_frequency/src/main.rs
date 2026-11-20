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

fn exercise_2949680777_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_2949680777_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_2949680777_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_2949680777_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_2949680777_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_2949680777_tests {
    use super::*;

    #[test]
    fn exercise_2949680777_sum_test() {
        assert_eq!(exercise_2949680777_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_2949680777_49: i32 = 126;
const EXERCISE_2949680777_50: i32 = 127;
const EXERCISE_2949680777_51: i32 = 128;
const EXERCISE_2949680777_52: i32 = 129;
const EXERCISE_2949680777_53: i32 = 130;
const EXERCISE_2949680777_54: i32 = 131;
const EXERCISE_2949680777_55: i32 = 132;
const EXERCISE_2949680777_56: i32 = 133;
const EXERCISE_2949680777_57: i32 = 134;
const EXERCISE_2949680777_58: i32 = 135;
const EXERCISE_2949680777_59: i32 = 136;
const EXERCISE_2949680777_60: i32 = 137;
const EXERCISE_2949680777_61: i32 = 138;
const EXERCISE_2949680777_62: i32 = 139;
const EXERCISE_2949680777_63: i32 = 140;
const EXERCISE_2949680777_64: i32 = 141;
const EXERCISE_2949680777_65: i32 = 142;
const EXERCISE_2949680777_66: i32 = 143;
const EXERCISE_2949680777_67: i32 = 144;
const EXERCISE_2949680777_68: i32 = 145;
const EXERCISE_2949680777_69: i32 = 146;
const EXERCISE_2949680777_70: i32 = 147;
const EXERCISE_2949680777_71: i32 = 148;
const EXERCISE_2949680777_72: i32 = 149;
const EXERCISE_2949680777_73: i32 = 150;
const EXERCISE_2949680777_74: i32 = 151;
const EXERCISE_2949680777_75: i32 = 152;
const EXERCISE_2949680777_76: i32 = 153;
const EXERCISE_2949680777_77: i32 = 154;
const EXERCISE_2949680777_78: i32 = 155;
const EXERCISE_2949680777_79: i32 = 156;
