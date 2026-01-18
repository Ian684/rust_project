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
