use std::fs::read_to_string;

#[must_use]
pub fn get_fens(path: &str) -> std::io::Result<Vec<String>> {
    let mut result = Vec::new();

    for line in read_to_string(path)?.lines() {
        let line = line.trim();
        if !line.is_empty() && !line.starts_with('#') {
            result.push(line.to_string())
        }
    }

    Ok(result)
}
