#[derive(Clone, Debug)]
struct ExerciseRecord {
    sequence: usize,
    label: String,
    score: i32,
}

impl ExerciseRecord {
    fn new(sequence: usize, label: &str, score: i32) -> Self {
        Self { sequence, label: label.to_string(), score }
    }

    fn grade(&self) -> char {
        if self.score >= 95 { 'A' }
        else if self.score >= 85 { 'B' }
        else if self.score >= 70 { 'C' }
        else { 'D' }
    }

    fn weighted(&self) -> i64 {
        (self.sequence as i64) * 7 + (self.score as i64) * 3
    }
}

fn summarize(records: &[ExerciseRecord]) -> (i32, i32, i32, i64) {
    let sum: i32 = records.iter().map(|record| record.score).sum();
    let maximum = records.iter().map(|record| record.score).max().unwrap_or(0);
    let minimum = records.iter().map(|record| record.score).min().unwrap_or(0);
    let weighted: i64 = records.iter().map(ExerciseRecord::weighted).sum();
    (sum, maximum, minimum, weighted)
}

fn filtered(records: &[ExerciseRecord], threshold: i32) -> Vec<&ExerciseRecord> {
    records.iter().filter(|record| record.score >= threshold).collect()
}

fn indexed(records: &[ExerciseRecord]) -> Vec<(usize, &str)> {
    records.iter().enumerate().map(|(index, record)| (index + 1, record.label.as_str())).collect()
}

fn main() {
    println!("Exercise 21: refcell");
    println!("Mechanism: RefCell permits runtime-checked interior mutability");
    let mut records = vec![
        ExerciseRecord::new(1, "Ada", 98),
        ExerciseRecord::new(2, "Linus", 86),
        ExerciseRecord::new(3, "Grace", 73),
        ExerciseRecord::new(4, "Ken", 64),
    ];
    records.sort_by(|left, right| left.sequence.cmp(&right.sequence));
    for record in &records {
        println!("{}{}: score={}, grade={}, weighted={}",
            record.sequence, record.label, record.score, record.grade(), record.weighted());
    }
    let (sum, maximum, minimum, weighted) = summarize(&records);
    println!("sum={} maximum={} minimum={} weighted={}", sum, maximum, minimum, weighted);
    let passing = filtered(&records, 85);
    println!("passing count={}", passing.len());
    let labels = indexed(&records);
    for (position, label) in labels {
        println!("position={} label={}", position, label);
    }
    let mut reversed = records.clone();
    reversed.reverse();
    println!("reversed first={:?}", reversed.first());
}

fn exercise_86818587_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_86818587_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_86818587_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_86818587_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_86818587_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_86818587_tests {
    use super::*;

    #[test]
    fn exercise_86818587_sum_test() {
        assert_eq!(exercise_86818587_sum(&[1, 2, 3]), 6);
    }
}
