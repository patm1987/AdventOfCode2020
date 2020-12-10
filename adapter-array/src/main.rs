fn main() {
    println!("Hello, world!");
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
    input.sort();
    let mut start = (0, 1); // start with 1 jump from highest to laptop (3)
    start = accumulate_window(start, &[0, input[0]]); // consider the first jump from wall
    input.windows(2).fold(start, |mut acc, x| accumulate_window(acc, x))
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().filter_map(|line| line.parse::<i32>().ok()).collect()
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
}
