fn parse_positive(input: &str) -> Result<u32, String> {
    let value = input
        .trim()
        .parse::<u32>()
        .map_err(|error| format!("invalid number: {error}"))?;
    if value == 0 {
        Err("value must be positive".to_string())
    } else {
        Ok(value)
    }
}

fn main() {
    for input in ["42", "0", "rust"] {
        println!("{input:?} -> {:?}", parse_positive(input));
    }
}
