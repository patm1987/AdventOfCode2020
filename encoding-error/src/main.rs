use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify an input file");
        return;
    }

    let filename: &str = &args[1];
    let file = fs::read_to_string(filename).unwrap();
    let parsed = parse_numbers(&file);
    let error = find_encoding_error(&parsed, 25);
    println!("Found encoding error at {}", error);
    let weakness = find_weakness(&parsed, error);
    println!("Weakness: {}", weakness);
}

fn parse_numbers(input: &str) -> Vec<i32> {
    input.lines().filter_map(|x| x.parse::<i32>().ok()).collect()
}

fn find_encoding_error(input: &Vec<i32>, window: usize) -> i32 {
    *input.windows(window + 1).find(|test_window| {
        let (preamble, test) = test_window.split_at(window);
        assert_eq!(1, test.len());
        for i in 0..window - 1 {
            let i_val = preamble[i];
            for j in i + 1..window {
                let j_val = preamble[j];
                // I don't know if this is valid, instructions unclear
                if i_val == j_val {
                    continue;
                }
                if i_val + j_val == test[0] {
                    return false;
                }
            }
        }
        return true;
    }).unwrap().last().unwrap()
}

fn find_contiguous(input: &Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..input.len() - 2 {
        for j in i + 1..input.len() - 1 {
            let range = &input[i..j + 1];
            let sum: i32 = range.iter().sum();
            if sum == target {
                return range.to_vec();
            } else if sum > target {
                break;
            }
        }
    }
    vec![]
}

fn find_weakness(input: &Vec<i32>, error: i32) -> i32 {
    let contiguous = find_contiguous(input, error);
    let mut min = contiguous[0];
    let mut max = contiguous[0];
    for i in 1..contiguous.len() {
        let test = contiguous[i];
        if test < min {
            min = test;
        }
        if test > max {
            max = test;
        }
    }
    min + max
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    const SAMPLE_INPUT_2: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_parses_input() {
        let expected = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];

        assert_eq!(parse_numbers(SAMPLE_INPUT), expected);
    }

    #[test]
    fn test_sample() {
        assert_eq!(127, find_encoding_error(&parse_numbers(SAMPLE_INPUT), 5));
    }

    #[test]
    fn test_find_contiguous() {
        assert_eq!(vec![15, 25, 47, 40], find_contiguous(&parse_numbers(SAMPLE_INPUT_2), 127));
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(62, find_weakness(&parse_numbers(SAMPLE_INPUT_2), 127));
    }
}
