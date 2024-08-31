use std::fs;
use regex::Regex;

#[allow(dead_code)]
pub fn check_output_charset_and_format(file_path: &str) -> bool {
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let reg = Regex::new(r"^\d+\n(\d+\n)*$").unwrap();
    reg.is_match(&content)
}

#[allow(dead_code)]
pub fn check_output_range_and_unicity(file_path: &str, nb_max_demons: usize) -> bool {
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let content = binding.lines();
    let mut demons = vec![false; nb_max_demons];
    if content.clone().count() > nb_max_demons {
        return false;
    }
    for line in content {
        match line.parse::<usize>() {
            Ok(value) => {
                if value >= nb_max_demons || demons[value] {
                    return false;
                }
                demons[value] = true;
            }
            Err(_) => {
                return false;
            }
        }
    }
    return true;
}