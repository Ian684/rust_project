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
