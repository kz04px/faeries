use std::fs::read_to_string;

#[must_use]
pub fn get_fens(path: &str) -> std::io::Result<Vec<String>> {
    let mut result = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        if !line.is_empty() && line.chars().nth(0).unwrap() != '#' {
            result.push(line.to_string())
        }
    }

    Ok(result)
}
