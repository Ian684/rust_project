//! Exercises 05–50. Each function has a normal and a boundary test.
use std::collections::{BTreeMap, BTreeSet};

/// 5. 移動 tuple 所有權
pub fn swap_pair<T>(pair: (T, T)) -> (T, T) {
    (pair.1, pair.0)
}

/// 6. 相鄰重複值與 Vec
pub fn dedup_adjacent(values: &[i32]) -> Vec<i32> {
    let mut out = values.to_vec();
    out.dedup();
    out
}

/// 7. 切片分塊與錯誤
pub fn chunks(values: &[i32], width: usize) -> Result<Vec<Vec<i32>>, String> {
    if width == 0 {
        return Err("zero width".into());
    }
    Ok(values.chunks(width).map(|x| x.to_vec()).collect())
}

/// 8. 迭代器攤平
pub fn flatten(rows: &[Vec<i32>]) -> Vec<i32> {
    rows.iter().flat_map(|r| r.iter().copied()).collect()
}

/// 9. entry 計數
pub fn frequencies(values: &[i32]) -> BTreeMap<i32, usize> {
    let mut out = BTreeMap::new();
    for &x in values {
        *out.entry(x).or_insert(0) += 1;
    }
    out
}

/// 10. 依字元數分組
pub fn group_length(words: &[String]) -> BTreeMap<usize, Vec<String>> {
    let mut out = BTreeMap::new();
    for word in words {
        out.entry(word.chars().count())
            .or_insert_with(Vec::new)
            .push(word.clone());
    }
    out
}

/// 11. partition 拆分集合
pub fn partition_even(values: &[i32]) -> (Vec<i32>, Vec<i32>) {
    values.iter().copied().partition(|x| x % 2 == 0)
}

/// 12. scan 前綴累積
pub fn running_totals(values: &[i64]) -> Vec<i128> {
    values
        .iter()
        .scan(0i128, |sum, &x| {
            *sum += i128::from(x);
            Some(*sum)
        })
        .collect()
}

/// 13. zip 的長度契約
pub fn dot_product(a: &[i32], b: &[i32]) -> Option<i128> {
    if a.len() != b.len() {
        return None;
    }
    Some(
        a.iter()
            .zip(b)
            .map(|(&x, &y)| i128::from(x) * i128::from(y))
            .sum(),
    )
}

/// 14. checked_mul 溢位
pub fn factorial(n: u32) -> Option<u128> {
    (1..=n).try_fold(1u128, |acc, x| acc.checked_mul(u128::from(x)))
}

/// 15. 迴圈狀態與 checked_add
pub fn fibonacci(n: u32) -> Option<u64> {
    let (mut a, mut b) = (0u64, 1u64);
    for i in 0..n {
        if i + 1 == n {
            return Some(b);
        }
        let next = a.checked_add(b)?;
        a = b;
        b = next;
    }
    Some(a)
}

/// 16. 除以零與 MIN/-1
pub fn safe_divide(a: i64, b: i64) -> Option<i64> {
    a.checked_div(b)
}

/// 17. 借用的 Option
pub fn safe_get<T>(values: &[T], index: usize) -> Option<&T> {
    values.get(index)
}

/// 18. 字元不是位元組
pub fn reverse_unicode(text: &str) -> String {
    text.chars().rev().collect()
}

/// 19. 回傳原字串借用
pub fn first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}

/// 20. 字串配置與正規化
pub fn word_counts(text: &str) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for word in text.split_whitespace() {
        *out.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    out
}

/// 21. char_indices 的 byte 位移
pub fn character_offsets(text: &str) -> Vec<(usize, char)> {
    text.char_indices().collect()
}

/// 22. Option 表達不存在
pub fn trim_nonempty(text: &str) -> Option<&str> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// 23. split_once 與 Result
pub fn parse_property(text: &str) -> Result<(&str, &str), String> {
    let (k, v) = text.split_once('=').ok_or("missing =")?;
    let k = k.trim();
    if k.is_empty() {
        return Err("empty key".into());
    }
    Ok((k, v.trim()))
}

/// 24. match 白名單
pub fn parse_switch(text: &str) -> Result<bool, String> {
    match text.trim().to_ascii_lowercase().as_str() {
        "yes" | "true" | "1" => Ok(true),
        "no" | "false" | "0" => Ok(false),
        _ => Err("invalid boolean".into()),
    }
}

/// 25. ? 傳遞解析錯誤
pub fn sum_numbers(text: &str) -> Result<i128, String> {
    text.split_whitespace().try_fold(0i128, |sum, token| {
        token
            .parse::<i64>()
            .map(|v| sum + i128::from(v))
            .map_err(|_| format!("invalid integer: {token}"))
    })
}

/// 26. 排序副本不改輸入
pub fn median(values: &[i64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut v = values.to_vec();
    v.sort_unstable();
    let n = v.len();
    Some(if n % 2 == 1 {
        v[n / 2] as f64
    } else {
        v[n / 2 - 1] as f64 / 2.0 + v[n / 2] as f64 / 2.0
    })
}

/// 27. 連號壓縮與溢位邊界
pub fn compress_ranges(values: &[i32]) -> Vec<(i32, i32)> {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut out: Vec<(i32, i32)> = Vec::new();
    for x in sorted {
        if let Some(last) = out.last_mut() {
            if last.1.checked_add(1) == Some(x) {
                last.1 = x;
                continue;
            }
        }
        out.push((x, x));
    }
    out
}

/// 28. 雙 iter 的 merge
pub fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            out.push(a[i]);
            i += 1
        } else {
            out.push(b[j]);
            j += 1
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

/// 29. 二分搜尋最左位置
pub fn first_equal(values: &[i32], target: i32) -> Option<usize> {
    let (mut l, mut r) = (0, values.len());
    while l < r {
        let m = l + (r - l) / 2;
        if values[m] < target {
            l = m + 1
        } else {
            r = m
        }
    }
    if values.get(l) == Some(&target) {
        Some(l)
    } else {
        None
    }
}

/// 30. Vec 旋轉與空集合
pub fn rotate_left(values: &[i32], amount: usize) -> Vec<i32> {
    let mut out = values.to_vec();
    if !out.is_empty() {
        let n = out.len();
        out.rotate_left(amount % n);
    }
    out
}

/// 31. 括號配對
pub fn balanced(text: &str) -> bool {
    let mut stack = Vec::new();
    for c in text.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                let expected = match c {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };
                if stack.pop() != Some(expected) {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}

/// 32. 帶溢位檢查的運算式
pub fn postfix(tokens: &[&str]) -> Result<i64, String> {
    let mut stack = Vec::new();
    for &token in tokens {
        if matches!(token, "+" | "-" | "*") {
            let b = stack.pop().ok_or("missing operand")?;
            let a: i64 = stack.pop().ok_or("missing operand")?;
            let v = match token {
                "+" => a.checked_add(b),
                "-" => a.checked_sub(b),
                _ => a.checked_mul(b),
            }
            .ok_or("overflow")?;
            stack.push(v);
        } else {
            stack.push(token.parse().map_err(|_| "invalid integer")?);
        }
    }
    if stack.len() != 1 {
        return Err("invalid expression".into());
    }
    Ok(stack[0])
}

/// 33. 可變狀態的 Euclid
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// 34. Vec<bool> 篩法
pub fn primes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }
    let mut p = vec![true; limit + 1];
    p[0] = false;
    p[1] = false;
    let mut d = 2;
    while d <= limit / d {
        if p[d] {
            for n in (d * d..=limit).step_by(d) {
                p[n] = false;
            }
        }
        d += 1;
    }
    (2..=limit).filter(|&n| p[n]).collect()
}

/// 35. 二項式係數與 checked_add
pub fn pascal_row(n: usize) -> Option<Vec<u128>> {
    let mut row = vec![1u128];
    for _ in 0..n {
        let mut next = vec![1];
        for pair in row.windows(2) {
            next.push(pair[0].checked_add(pair[1])?);
        }
        next.push(1);
        row = next;
    }
    Some(row)
}

/// 36. 矩形檢查與配置
pub fn transpose(rows: &[Vec<i32>]) -> Result<Vec<Vec<i32>>, String> {
    if rows.is_empty() {
        return Ok(vec![]);
    }
    let w = rows[0].len();
    if rows.iter().any(|r| r.len() != w) {
        return Err("ragged matrix".into());
    }
    Ok((0..w)
        .map(|c| rows.iter().map(|r| r[c]).collect())
        .collect())
}

/// 37. 邊界收縮的矩陣走訪
pub fn spiral(rows: &[Vec<i32>]) -> Result<Vec<i32>, String> {
    if rows.is_empty() {
        return Ok(vec![]);
    }
    let w = rows[0].len();
    if rows.iter().any(|r| r.len() != w) {
        return Err("ragged matrix".into());
    }
    let (mut top, mut bottom, mut left, mut right) = (0, rows.len(), 0, w);
    let mut out = Vec::new();
    while top < bottom && left < right {
        for c in left..right {
            out.push(rows[top][c]);
        }
        top += 1;
        for row in rows.iter().take(bottom).skip(top) {
            out.push(row[right - 1]);
        }
        right -= 1;
        if top < bottom {
            bottom -= 1;
            for c in (left..right).rev() {
                out.push(rows[bottom][c]);
            }
        }
        if left < right {
            for row in rows.iter().take(bottom).skip(top).rev() {
                out.push(row[left]);
            }
            left += 1;
        }
    }
    Ok(out)
}

/// 38. 巢狀迭代器與 move
pub fn cartesian(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    a.iter()
        .flat_map(|&x| b.iter().map(move |&y| (x, y)))
        .collect()
}

/// 39. 不重複位置組合
pub fn index_pairs(n: usize) -> Vec<(usize, usize)> {
    (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .collect()
}

/// 40. Unicode 連續段
pub fn run_length(text: &str) -> Vec<(char, usize)> {
    let mut out: Vec<(char, usize)> = Vec::new();
    for c in text.chars() {
        if let Some(last) = out.last_mut() {
            if last.0 == c {
                last.1 += 1;
                continue;
            }
        }
        out.push((c, 1));
    }
    out
}

/// 41. 位元組格式化
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|x| format!("{x:02x}")).collect()
}

/// 42. chunks_exact 與非法 hex
pub fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
    if !text.is_ascii() || text.len() % 2 != 0 {
        return Err("invalid hex length".into());
    }
    text.as_bytes()
        .chunks_exact(2)
        .map(|p| {
            let s = std::str::from_utf8(p).map_err(|_| "invalid text")?;
            u8::from_str_radix(s, 16).map_err(|_| "invalid hex".into())
        })
        .collect()
}

/// 43. 浮點數有限值驗證
pub fn celsius(fahrenheit: f64) -> Result<f64, String> {
    if !fahrenheit.is_finite() || fahrenheit < -459.67 {
        return Err("invalid temperature".into());
    }
    Ok((fahrenheit - 32.0) * 5.0 / 9.0)
}

/// 44. 區間的 Result<Option>
pub fn overlap(a: (i32, i32), b: (i32, i32)) -> Result<Option<(i32, i32)>, String> {
    if a.0 > a.1 || b.0 > b.1 {
        return Err("reversed interval".into());
    }
    let l = a.0.max(b.0);
    let r = a.1.min(b.1);
    Ok(if l < r { Some((l, r)) } else { None })
}

/// 45. 排序後 take
pub fn smallest(values: &[i32], count: usize) -> Vec<i32> {
    let mut out = values.to_vec();
    out.sort_unstable();
    out.truncate(count);
    out
}

/// 46. BTreeSet 集合差
pub fn set_difference(a: &[i32], b: &[i32]) -> Vec<i32> {
    let a: BTreeSet<_> = a.iter().copied().collect();
    let b: BTreeSet<_> = b.iter().copied().collect();
    a.difference(&b).copied().collect()
}

/// 47. 保留出現次數的交集
pub fn multiset_intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut counts = frequencies(b);
    let mut out = Vec::new();
    for &x in a {
        if let Some(n) = counts.get_mut(&x) {
            if *n > 0 {
                out.push(x);
                *n -= 1;
            }
        }
    }
    out.sort_unstable();
    out
}

/// 48. windows API 與錯誤
pub fn window_maxima(values: &[i32], width: usize) -> Option<Vec<i32>> {
    if width == 0 || width > values.len() {
        return None;
    }
    Some(
        values
            .windows(width)
            .map(|w| *w.iter().max().unwrap())
            .collect(),
    )
}

/// 49. 候選人再驗票
pub fn majority(values: &[i32]) -> Option<i32> {
    let (mut candidate, mut votes) = (0, 0);
    for &x in values {
        if votes == 0 {
            candidate = x;
        }
        if x == candidate {
            votes += 1
        } else {
            votes -= 1
        }
    }
    if values.iter().filter(|&&x| x == candidate).count() > values.len() / 2 {
        Some(candidate)
    } else {
        None
    }
}

/// 50. 泛型閉包參數
pub fn apply_twice<T, F: Fn(T) -> T>(value: T, operation: F) -> T {
    operation(operation(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exercise_05_swap_pair() {
        assert_eq!(swap_pair((1, 2)), (2, 1));
        assert_eq!(
            swap_pair(("a".to_string(), "b".to_string())),
            ("b".into(), "a".into())
        );
    }
    #[test]
    fn exercise_06_dedup_adjacent() {
        assert_eq!(dedup_adjacent(&[1, 1, 2, 1]), vec![1, 2, 1]);
        assert!(dedup_adjacent(&[]).is_empty());
    }
    #[test]
    fn exercise_07_chunks() {
        assert_eq!(chunks(&[1, 2, 3], 2).unwrap(), vec![vec![1, 2], vec![3]]);
        assert!(chunks(&[], 0).is_err());
    }
    #[test]
    fn exercise_08_flatten() {
        assert_eq!(flatten(&[vec![1, 2], vec![], vec![3]]), vec![1, 2, 3]);
        assert!(flatten(&[]).is_empty());
    }
    #[test]
    fn exercise_09_frequencies() {
        assert_eq!(frequencies(&[2, 1, 2])[&2], 2);
        assert!(frequencies(&[]).is_empty());
    }
    #[test]
    fn exercise_10_group_length() {
        assert_eq!(group_length(&["貓".into(), "a".into()])[&1].len(), 2);
        assert!(group_length(&[]).is_empty());
    }
    #[test]
    fn exercise_11_partition_even() {
        assert_eq!(partition_even(&[1, 2, -4, 3]), (vec![2, -4], vec![1, 3]));
        assert_eq!(partition_even(&[]), (vec![], vec![]));
    }
    #[test]
    fn exercise_12_running_totals() {
        assert_eq!(running_totals(&[2, -3, 5]), vec![2, -1, 4]);
        assert!(running_totals(&[]).is_empty());
    }
    #[test]
    fn exercise_13_dot_product() {
        assert_eq!(dot_product(&[1, 2], &[3, 4]), Some(11));
        assert_eq!(dot_product(&[1], &[]), None);
    }
    #[test]
    fn exercise_14_factorial() {
        assert_eq!(factorial(0), Some(1));
        assert_eq!(factorial(5), Some(120));
        assert_eq!(factorial(100), None);
    }
    #[test]
    fn exercise_15_fibonacci() {
        assert_eq!(fibonacci(0), Some(0));
        assert_eq!(fibonacci(10), Some(55));
        assert_eq!(fibonacci(100), None);
    }
    #[test]
    fn exercise_16_safe_divide() {
        assert_eq!(safe_divide(9, 2), Some(4));
        assert_eq!(safe_divide(1, 0), None);
        assert_eq!(safe_divide(i64::MIN, -1), None);
    }
    #[test]
    fn exercise_17_safe_get() {
        assert_eq!(safe_get(&[4, 5], 1), Some(&5));
        assert_eq!(safe_get::<i32>(&[], 0), None);
    }
    #[test]
    fn exercise_18_reverse_unicode() {
        assert_eq!(reverse_unicode("a貓🙂"), "🙂貓a");
        assert_eq!(reverse_unicode(""), "");
    }
    #[test]
    fn exercise_19_first_word() {
        assert_eq!(first_word("  hello world"), Some("hello"));
        assert_eq!(first_word("   "), None);
    }
    #[test]
    fn exercise_20_word_counts() {
        assert_eq!(word_counts("Rust rust 貓")["rust"], 2);
        assert!(word_counts("").is_empty());
    }
    #[test]
    fn exercise_21_character_offsets() {
        assert_eq!(character_offsets("a貓"), vec![(0, 'a'), (1, '貓')]);
        assert!(character_offsets("").is_empty());
    }
    #[test]
    fn exercise_22_trim_nonempty() {
        assert_eq!(trim_nonempty(" hi "), Some("hi"));
        assert_eq!(trim_nonempty("\t "), None);
    }
    #[test]
    fn exercise_23_parse_property() {
        assert_eq!(parse_property("x = a=b").unwrap(), ("x", "a=b"));
        assert!(parse_property("=2").is_err());
    }
    #[test]
    fn exercise_24_parse_switch() {
        assert_eq!(parse_switch(" YES "), Ok(true));
        assert_eq!(parse_switch("0"), Ok(false));
        assert!(parse_switch("maybe").is_err());
    }
    #[test]
    fn exercise_25_sum_numbers() {
        assert_eq!(sum_numbers("1 -2 4"), Ok(3));
        assert!(sum_numbers("1 x").is_err());
        assert_eq!(sum_numbers(""), Ok(0));
    }
    #[test]
    fn exercise_26_median() {
        assert_eq!(median(&[4, 1, 2, 3]), Some(2.5));
        assert_eq!(median(&[]), None);
    }
    #[test]
    fn exercise_27_compress_ranges() {
        assert_eq!(compress_ranges(&[3, 1, 2, 7, 7]), vec![(1, 3), (7, 7)]);
        assert_eq!(compress_ranges(&[i32::MAX]), vec![(i32::MAX, i32::MAX)]);
    }
    #[test]
    fn exercise_28_merge_sorted() {
        assert_eq!(merge_sorted(&[1, 3], &[2, 4]), vec![1, 2, 3, 4]);
        assert_eq!(merge_sorted(&[], &[1]), vec![1]);
    }
    #[test]
    fn exercise_29_first_equal() {
        assert_eq!(first_equal(&[1, 2, 2, 4], 2), Some(1));
        assert_eq!(first_equal(&[], 1), None);
    }
    #[test]
    fn exercise_30_rotate_left() {
        assert_eq!(rotate_left(&[1, 2, 3], 4), vec![2, 3, 1]);
        assert!(rotate_left(&[], 7).is_empty());
    }
    #[test]
    fn exercise_31_balanced() {
        assert!(balanced("a{[()]}"));
        assert!(!balanced("([)]"));
    }
    #[test]
    fn exercise_32_postfix() {
        assert_eq!(postfix(&["3", "4", "+", "2", "*"]), Ok(14));
        assert!(postfix(&["+"]).is_err());
    }
    #[test]
    fn exercise_33_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(0, 0), 0);
    }
    #[test]
    fn exercise_34_primes() {
        assert_eq!(primes(10), vec![2, 3, 5, 7]);
        assert!(primes(1).is_empty());
    }
    #[test]
    fn exercise_35_pascal_row() {
        assert_eq!(pascal_row(4), Some(vec![1, 4, 6, 4, 1]));
        assert_eq!(pascal_row(0), Some(vec![1]));
    }
    #[test]
    fn exercise_36_transpose() {
        assert_eq!(
            transpose(&[vec![1, 2], vec![3, 4]]).unwrap(),
            vec![vec![1, 3], vec![2, 4]]
        );
        assert!(transpose(&[vec![1], vec![]]).is_err());
    }
    #[test]
    fn exercise_37_spiral() {
        assert_eq!(spiral(&[vec![1, 2], vec![3, 4]]).unwrap(), vec![1, 2, 4, 3]);
        assert_eq!(spiral(&[vec![1], vec![2]]).unwrap(), vec![1, 2]);
    }
    #[test]
    fn exercise_38_cartesian() {
        assert_eq!(cartesian(&[1, 2], &[3]), vec![(1, 3), (2, 3)]);
        assert!(cartesian(&[], &[1]).is_empty());
    }
    #[test]
    fn exercise_39_index_pairs() {
        assert_eq!(index_pairs(3), vec![(0, 1), (0, 2), (1, 2)]);
        assert!(index_pairs(1).is_empty());
    }
    #[test]
    fn exercise_40_run_length() {
        assert_eq!(run_length("貓貓a"), vec![('貓', 2), ('a', 1)]);
        assert!(run_length("").is_empty());
    }
    #[test]
    fn exercise_41_hex_encode() {
        assert_eq!(hex_encode(&[0, 15, 255]), "000fff");
        assert_eq!(hex_encode(&[]), "");
    }
    #[test]
    fn exercise_42_hex_decode() {
        assert_eq!(hex_decode("00fF"), Ok(vec![0, 255]));
        assert!(hex_decode("0").is_err());
        assert!(hex_decode("zz").is_err());
    }
    #[test]
    fn exercise_43_celsius() {
        assert_eq!(celsius(32.0), Ok(0.0));
        assert!(celsius(f64::NAN).is_err());
    }
    #[test]
    fn exercise_44_overlap() {
        assert_eq!(overlap((1, 5), (3, 7)), Ok(Some((3, 5))));
        assert_eq!(overlap((0, 1), (1, 2)), Ok(None));
        assert!(overlap((2, 1), (0, 3)).is_err());
    }
    #[test]
    fn exercise_45_smallest() {
        assert_eq!(smallest(&[3, 1, 2], 2), vec![1, 2]);
        assert!(smallest(&[1], 0).is_empty());
    }
    #[test]
    fn exercise_46_set_difference() {
        assert_eq!(set_difference(&[1, 2, 2, 3], &[2]), vec![1, 3]);
        assert!(set_difference(&[], &[]).is_empty());
    }
    #[test]
    fn exercise_47_multiset_intersection() {
        assert_eq!(multiset_intersection(&[2, 2, 2, 1], &[2, 2, 3]), vec![2, 2]);
        assert!(multiset_intersection(&[], &[1]).is_empty());
    }
    #[test]
    fn exercise_48_window_maxima() {
        assert_eq!(window_maxima(&[1, 3, 2, 5], 2), Some(vec![3, 3, 5]));
        assert_eq!(window_maxima(&[], 0), None);
    }
    #[test]
    fn exercise_49_majority() {
        assert_eq!(majority(&[2, 1, 2]), Some(2));
        assert_eq!(majority(&[1, 2]), None);
        assert_eq!(majority(&[]), None);
    }
    #[test]
    fn exercise_50_apply_twice() {
        let delta = 3;
        assert_eq!(apply_twice(1, |x| x + delta), 7);
        assert_eq!(
            apply_twice(String::new(), |mut x| {
                x.push('a');
                x
            }),
            "aa"
        );
    }
}
