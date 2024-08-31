use std::fmt::Write;
use std::io::Result;
use std::fs;

pub fn save_to_file(output_path: &str, demons_order: Vec<usize>) -> Result<()> {
    let mut content = String::new();
    for demon_id in demons_order {
        writeln!(&mut content, "{}", demon_id.to_string()).unwrap();
    }
    fs::write(output_path, content)?; // reminder for me: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    Ok(())
}