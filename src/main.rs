use regex::Regex;

mod checks;

fn main() {
    let re = Regex::new(r"(?m)^(\d\n)+$").unwrap();

    let test_string = "1\n2\n3\n4\n0\n";

    if re.is_match(test_string) {
        println!("Le format correspond !");
    } else {
        println!("Le format ne correspond pas.");
    }
}
