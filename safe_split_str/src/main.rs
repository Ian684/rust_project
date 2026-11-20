fn split_once(s: &str, delim: char) -> Option<(&str, &str)>{
    for (i,c) in s.char_indices() {
        if c == delim{
            return Some((&s[..i] , &s[i + c.len_utf8()..]));
        }
    }
    None
}


fn main() {
    match split_once("aaaaabbcbbb" , 'c'){
        Some((x, y)) => println!("{} {}", x, y),
        None => println!("no"),
    }
}

fn exercise_3721983856_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_3721983856_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_3721983856_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_3721983856_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_3721983856_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_3721983856_tests {
    use super::*;

    #[test]
    fn exercise_3721983856_sum_test() {
        assert_eq!(exercise_3721983856_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_3721983856_46: i32 = 102;
const EXERCISE_3721983856_47: i32 = 103;
const EXERCISE_3721983856_48: i32 = 104;
const EXERCISE_3721983856_49: i32 = 105;
const EXERCISE_3721983856_50: i32 = 106;
const EXERCISE_3721983856_51: i32 = 107;
const EXERCISE_3721983856_52: i32 = 108;
const EXERCISE_3721983856_53: i32 = 109;
const EXERCISE_3721983856_54: i32 = 110;
const EXERCISE_3721983856_55: i32 = 111;
const EXERCISE_3721983856_56: i32 = 112;
const EXERCISE_3721983856_57: i32 = 113;
const EXERCISE_3721983856_58: i32 = 114;
const EXERCISE_3721983856_59: i32 = 115;
const EXERCISE_3721983856_60: i32 = 116;
const EXERCISE_3721983856_61: i32 = 117;
const EXERCISE_3721983856_62: i32 = 118;
const EXERCISE_3721983856_63: i32 = 119;
const EXERCISE_3721983856_64: i32 = 120;
const EXERCISE_3721983856_65: i32 = 121;
const EXERCISE_3721983856_66: i32 = 122;
const EXERCISE_3721983856_67: i32 = 123;
const EXERCISE_3721983856_68: i32 = 124;
const EXERCISE_3721983856_69: i32 = 125;
const EXERCISE_3721983856_70: i32 = 126;
const EXERCISE_3721983856_71: i32 = 127;
const EXERCISE_3721983856_72: i32 = 128;
const EXERCISE_3721983856_73: i32 = 129;
const EXERCISE_3721983856_74: i32 = 130;
const EXERCISE_3721983856_75: i32 = 131;
const EXERCISE_3721983856_76: i32 = 132;
const EXERCISE_3721983856_77: i32 = 133;
const EXERCISE_3721983856_78: i32 = 134;
const EXERCISE_3721983856_79: i32 = 135;
