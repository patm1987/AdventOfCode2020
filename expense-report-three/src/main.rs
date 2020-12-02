fn main() {
    println!("Hello, world!");
}

fn find_multiple_of_three(input: Vec<i32>, target: i32) -> Option<i32> {
    if input.len() < 3 {
        return None;
    } else {
        let mut a = 0;
        let mut b = a + 1;
        let mut c = input.len() - 1;

        return if input[a] + input[b] + input[c] == target {
            Some(input[a] * input[b] * input[c])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_for_empty() {
        assert_eq!(None, find_multiple_of_three(Vec::new(), 1))
    }

    #[test]
    fn multiplies_three() {
        let values = vec![2, 3, 4];
        assert_eq!(Some(24), find_multiple_of_three(values, 9))
    }

    #[test]
    fn finds_none_if_sum_isnt_target() {
        let values = vec![2, 3, 4];
        assert_eq!(None, find_multiple_of_three(values, 10))
    }

    #[test]
    fn finds_some_if_middle_left_solution() {
        let values = vec![2, 3, 4, 5];
        assert_eq!(Some(30), find_multiple_of_three(values, 10))
    }
}