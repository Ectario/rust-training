use rust_training::checks::*;

#[cfg(test)]
mod outputs_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_output_wrong_charset() {
        assert_eq!(check_output_charset_and_format("outputs/tests/wrong_charset.txt"), false);
    }

    #[test]
    fn test_output_wrong_format() {
        assert_eq!(check_output_charset_and_format("outputs/tests/wrong_format.txt"), false);
    }

    #[test]
    fn test_output_good() {
        assert_eq!(check_output_charset_and_format("outputs/tests/good_output.txt"), true);
        assert_eq!(check_output_range_and_unicity("outputs/tests/good_output.txt", 5), true);
    }

    #[test]
    fn test_output_not_in_range() {
        assert_eq!(check_output_range_and_unicity("outputs/tests/wrong_not_in_range.txt", 5), false);
    }

    #[test]
    fn test_output_not_in_range_2() {
        assert_eq!(check_output_range_and_unicity("outputs/tests/wrong_not_in_range_2.txt", 5), false);
    }

    #[test]
    fn test_output_same_index() {
        assert_eq!(check_output_range_and_unicity("outputs/tests/wrong_same_index.txt", 5), false);
    }

}