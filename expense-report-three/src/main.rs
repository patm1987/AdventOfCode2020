fn main() {
    println!("Hello, world!");
}

fn find_multiple_of_three(input: &mut Vec<i32>, target: i32) -> Option<i32> {
    input.sort();

    if input.len() < 3 {
        return None;
    } else {
        let mut a = 0;
        let mut b = 1;
        let mut c = input.len() - 1;

        while a < b && b < c {
            while c > b && input[a] + input[b] + input[c] > target {
                c -= 1;
            }

            while b < c {
                if input[a] + input[b] + input[c] == target {
                    return Some(input[a] * input[b] * input[c]);
                }

                b += 1;

                // early out if we've gone too far
                if (target - (input[a] + input[c])) > input[b] {
                    break;
                }
            }

            a += 1;
            b = a + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_for_empty() {
        assert_eq!(None, find_multiple_of_three(&mut Vec::new(), 1))
    }

    #[test]
    fn multiplies_three() {
        let mut values = vec![2, 3, 4];
        assert_eq!(Some(24), find_multiple_of_three(&mut values, 9))
    }

    #[test]
    fn finds_none_if_sum_isnt_target() {
        let mut values = vec![2, 3, 4];
        assert_eq!(None, find_multiple_of_three(&mut values, 10))
    }

    #[test]
    fn finds_some_if_middle_left_solution() {
        let mut values = vec![2, 3, 4, 5];
        assert_eq!(Some(30), find_multiple_of_three(&mut values, 10))
    }

    #[test]
    fn finds_some_if_middle_right_solution() {
        let mut values = vec![2, 3, 4, 5];
        assert_eq!(Some(40), find_multiple_of_three(&mut values, 11))
    }

    #[test]
    fn finds_some_if_right_must_shift() {
        let mut values = vec![2, 3, 4, 5];
        assert_eq!(Some(24), find_multiple_of_three(&mut values, 9));
    }

    #[test]
    fn finds_some_if_left_must_shift() {
        let mut values = vec![2, 3, 4, 5];
        assert_eq!(Some(60), find_multiple_of_three(&mut values, 12))
    }

    #[test]
    fn verify_works_in_middle() {
        let mut values = vec![1, 3, 4, 5, 6];
        assert_eq!(Some(60), find_multiple_of_three(&mut values, 12))
    }

    #[test]
    fn verify_works_with_unsorted_data() {
        let mut values = vec![10, 3, 4, 2];
        assert_eq!(Some(24), find_multiple_of_three(&mut values, 9))
    }
}