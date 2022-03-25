pub fn median(mut nums: Vec<f32>) -> Option<f32> {
    if nums.is_empty() {
        return None;
    }

    // NOTE: If there's a "None" case, this will crash - different approach is needed to filter these out
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let midpoint = nums.len() / 2;
    let result = if nums.len() % 2 == 0 {
        (nums[midpoint - 1] + nums[midpoint]) / 2.0
    } else {
        nums[midpoint]
    };

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::median;

    #[test]
    fn empty_list() {
        let input = vec![];
        let expected_output = None;
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn sorted_list() {
        let input = vec![1.0, 4.0, 5.0];
        let expected_output = Some(4.0);
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn even_length() {
        let input = vec![1.0, 3.0, 5.0, 6.0];
        let expected_output = Some(4.0);
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list() {
        let input = vec![1.0, 5.0, 2.0];
        let expected_output = Some(2.0);
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }
}
