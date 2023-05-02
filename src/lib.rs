
pub fn median(numbers: &Vec<i32>) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    if numbers.len() % 2 == 1 {
        let index = numbers.len() / 2;
        let result = numbers[index] as f64;
        return Some(result);
    } else {
        let index_two = numbers.len() / 2;
        let index_one = index_two - 1;
        let sum = (numbers[index_one] + numbers[index_two]) as f64;
        let result = sum / 2.0;
        return Some(result);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_returns_the_middle_value_if_the_length_is_odd() {
        let numbers = vec![1, 2, 3, 4, 5];
        let median = crate::median(&numbers);
        assert_eq!(median, Some(3.0));
    }
    #[test]
    fn it_returns_the_average_of_the_two_middle_values_if_the_length_is_even() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let median = crate::median(&numbers);
        assert_eq!(median, Some(3.5));
    }
    #[test]
    fn it_returns_none_if_the_array_is_empty() {
        let numbers: Vec<i32> = Vec::new();
        let median = crate::median(&numbers);
        assert_eq!(median, None);
    }
}