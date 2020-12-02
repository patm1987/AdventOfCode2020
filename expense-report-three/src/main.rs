fn main() {
    println!("Hello, world!");
}

fn find_multiple_of_three(input: Vec<i32>, target: i32) -> Option<i32> {
    None
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
}