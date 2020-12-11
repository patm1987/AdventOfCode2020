use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify program input");
        return;
    }

    let filename: &str = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to parse input");
    let mut parsed_input = parse_input(&input);
    let enumerated = enumerate_differences(&mut parsed_input);
    println!("The delta is {} and {} product {}", enumerated.0, enumerated.1, enumerated.0 * enumerated.1)
}

fn accumulate_window(mut acc: (i32, i32), window: &[i32]) -> (i32, i32) {
    let difference = window[1] - window[0];
    if difference == 1 {
        acc.0 += 1;
    } else if difference == 3 {
        acc.1 += 1;
    }
    acc
}

// returns (1 joltage differences, 3 joltage differences)
fn enumerate_differences(input: &mut Vec<i32>) -> (i32, i32) {
    let mut start = (0, 1); // start with 1 jump from highest to laptop (3)
    start = accumulate_window(start, &[0, input[0]]); // consider the first jump from wall
    input.windows(2).fold(start, |acc, x| accumulate_window(acc, x))
}

fn parse_input(input: &str) -> Vec<i32> {
    let mut output: Vec<i32> = input.trim().lines().filter_map(|line| line.parse::<i32>().ok()).collect();
    output.sort();
    output
}

fn validate_range<'a, T>(range: T, start: i32, end: i32) -> bool
    where
        T: IntoIterator<Item=&'a i32>, {
    let acc = range.into_iter().try_fold(start, |acc, &x| {
        if x - acc > 3 {
            None
        } else {
            Some(x)
        }
    });
    match acc {
        Some(x) => {
            if end - x > 3 {
                false
            } else {
                true
            }
        }
        None => { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_passes_sample() {
        assert_eq!((22, 10), enumerate_differences(&mut parse_input(SAMPLE_INPUT)))
    }

    #[test]
    fn validates_good_array() {
        assert!(validate_range(&[1, 2, 3], 0, 4));
    }

    #[test]
    fn doesnt_validate_bad_array() {
        assert!(!validate_range(&[1, 6], 0, 7));
    }

    #[test]
    fn checks_start_for_validation() {
        assert!(!validate_range(&[4, 5, 6], 0, 7));
    }

    #[test]
    fn checks_end_for_validation() {
        assert!(!validate_range(&[1, 2, 3], 0, 7))
    }
}
