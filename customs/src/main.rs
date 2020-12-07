use std::collections::{HashSet};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify an input file");
        return
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename).unwrap();
    println!("Sum of all answers: {}", parse_input(input).iter().sum::<i32>());
}

fn parse_input(input: String) -> Vec<i32> {
    input.trim().split("\n\n").map(|x| {
        let mut map = HashSet::new();
        x.chars().for_each(|c| {
            if !c.is_whitespace() {
                map.insert(c);
            }
        });
        map.len() as i32
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_parses_sample_input() {
        let expected = vec![3, 3, 3, 1, 1];
        // expected.iter().zip(parse_input(String::from(SAMPLE_INPUT)).for_each(|(lhs, rhs)| assert_eq!(lhs, rhs));
        assert_eq!(expected, parse_input(String::from(SAMPLE_INPUT)));
    }

    #[test]
    fn test_can_sum() {
        assert_eq!(11, parse_input(String::from(SAMPLE_INPUT)).iter().sum())
    }
}
