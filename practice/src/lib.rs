//! Small exercises in ownership, Result, borrowed data and scoped threads.
pub mod more;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Stack<T> {
    values: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { Self { values: Vec::new() } }
    pub fn push(&mut self, value: T) { self.values.push(value); }
    pub fn pop(&mut self) -> Option<T> { self.values.pop() }
    pub fn peek(&self) -> Option<&T> { self.values.last() }
    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
}

#[derive(Debug, PartialEq)]
pub struct Statistics {
    pub count: usize,
    pub min: i64,
    pub max: i64,
    pub sum: i128,
}

/// Accept comma/newline separated i64 measurements; reject missing fields.
pub fn parse_statistics(input: &str) -> Result<Statistics, String> {
    let input = input.trim();
    if input.is_empty() { return Err("no measurements".into()); }
    let mut result = Statistics { count: 0, min: i64::MAX, max: i64::MIN, sum: 0 };
    for (index, field) in input.split([',', '\n']).enumerate() {
        let value = field.trim().parse::<i64>()
            .map_err(|_| format!("invalid integer at field {}", index + 1))?;
        result.count += 1;
        result.min = result.min.min(value);
        result.max = result.max.max(value);
        result.sum += i128::from(value);
    }
    Ok(result)
}

/// Words remain borrowed slices. Positions are word indices, not byte offsets.
pub fn word_positions(text: &str) -> BTreeMap<&str, Vec<usize>> {
    let mut index = BTreeMap::new();
    for (position, word) in text.split_whitespace().enumerate() {
        index.entry(word).or_insert_with(Vec::new).push(position);
    }
    index
}

/// Scoped threads can borrow the input without cloning it into Arc storage.
pub fn parallel_sum(values: &[i64], workers: usize) -> Result<i128, &'static str> {
    if workers == 0 { return Err("workers must be positive"); }
    if values.is_empty() { return Ok(0); }
    let workers = workers.min(values.len());
    let width = values.len() / workers + usize::from(values.len() % workers != 0);
    std::thread::scope(|scope| {
        let handles: Vec<_> = values.chunks(width)
            .map(|chunk| scope.spawn(move || chunk.iter().map(|&x| i128::from(x)).sum::<i128>()))
            .collect();
        let mut total = 0;
        for handle in handles {
            total += handle.join().map_err(|_| "worker panicked")?;
        }
        Ok(total)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_moves_strings_and_handles_empty() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), None);
        stack.push(String::from("first"));
        stack.push(String::from("second"));
        assert_eq!(stack.peek().map(String::as_str), Some("second"));
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop().as_deref(), Some("second"));
        assert_eq!(stack.pop().as_deref(), Some("first"));
        assert!(stack.is_empty());
    }

    #[test]
    fn statistics_and_wide_accumulator() {
        assert_eq!(parse_statistics("10,20\n-5").unwrap(), Statistics { count: 3, min: -5, max: 20, sum: 25 });
        assert_eq!(parse_statistics("9223372036854775807,1").unwrap().sum, 9223372036854775808);
    }

    #[test]
    fn malformed_fields_are_errors() {
        for input in ["", "1,,2", "x", "9223372036854775808", "1,"] {
            assert!(parse_statistics(input).is_err(), "{input:?}");
        }
    }

    #[test]
    fn borrowed_unicode_index() {
        let index = word_positions("貓 dog 貓\ndog");
        assert_eq!(index["貓"], vec![0, 2]);
        assert_eq!(index["dog"], vec![1, 3]);
        assert!(word_positions("").is_empty());
    }

    #[test]
    fn parallel_matches_sequential_for_uneven_chunks() {
        let values: Vec<i64> = (-53..58).collect();
        let expected: i128 = values.iter().map(|&x| i128::from(x)).sum();
        for workers in [1, 2, 3, 8, 200] {
            assert_eq!(parallel_sum(&values, workers), Ok(expected));
        }
        assert_eq!(parallel_sum(&[i64::MAX, i64::MAX], 2), Ok(i128::from(i64::MAX) * 2));
        assert_eq!(parallel_sum(&[], 3), Ok(0));
        assert!(parallel_sum(&values, 0).is_err());
    }
}
