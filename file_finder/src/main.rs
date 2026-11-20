use std::fs;
use std::io;
use std::path::{Path , PathBuf};



fn find_files(root: &str, keyword: &str) -> Result<Vec<String>, std::io::Error> {
    let mut arr: Vec<String> = Vec::new();
    find_files_inner(Path::new(root) , keyword , &mut arr)?;
    Ok(arr)
}




fn find_files_inner(root: &Path, keyword: &str, arr: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(root)? {
        let _entry = match entry{
            Ok(x) => x,
            Err(_) => continue,
        };
        
        let path: PathBuf = _entry.path();
        if path.is_dir() {
            let _ = find_files_inner(&path , keyword , arr);
        }
        else if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            if name.contains(keyword) {
                arr.push(path.to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}


fn main() {
    match find_files(&"rust_project" , &"main.rs") {
        Ok(x) => println!("{:?}" , x),
        Err(y) => println!("{}" , y),
    }
}

fn exercise_70497782_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_70497782_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_70497782_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_70497782_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_70497782_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_70497782_tests {
    use super::*;

    #[test]
    fn exercise_70497782_sum_test() {
        assert_eq!(exercise_70497782_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_70497782_72: i32 = 154;
const EXERCISE_70497782_73: i32 = 155;
const EXERCISE_70497782_74: i32 = 156;
const EXERCISE_70497782_75: i32 = 157;
const EXERCISE_70497782_76: i32 = 158;
const EXERCISE_70497782_77: i32 = 159;
const EXERCISE_70497782_78: i32 = 160;
const EXERCISE_70497782_79: i32 = 161;
