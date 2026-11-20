fn main() {
    let s1 = String::from("a");
    let mut s2 = s1;

    //let r1 = &s2;
    //let r2 = &s2;
    let r3 = &mut s2;
    *r3 += "a";
    println!("{}" , r3);

}

fn sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        assert_eq!(sum(&[1, 2, 3]), 6);
    }
}
