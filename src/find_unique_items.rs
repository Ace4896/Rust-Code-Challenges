// Generic implementation which takes in a vector of ordered values
fn unique<T: Ord>(mut items: Vec<T>) -> Vec<T> {
    items.sort_unstable();
    items.dedup();
    items
}

#[cfg(test)]
mod tests {
    use super::unique;

    #[test]
    fn empty_list() {
        let input: Vec<i32> = vec![];
        let expected_output = vec![];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn sorted_list() {
        let input = vec![1, 4, 5];
        let expected_output = vec![1, 4, 5];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list() {
        let input = vec![1, 5, 2];
        let expected_output = vec![1, 2, 5];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list_with_duplicates() {
        let input = vec![1, 5, 2, 2, 1];
        let expected_output = vec![1, 2, 5];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn sorted_list_with_duplicates() {
        let mut input = vec![1, 5, 2, 2, 1];
        input.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let expected_output = vec![1, 2, 5];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }
}
