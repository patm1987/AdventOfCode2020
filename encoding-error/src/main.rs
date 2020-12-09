fn main() {
    println!("Hello, world!");
}

fn parse_numbers(input: &str) -> Vec<i32> {
    input.lines().filter_map(|x| x.parse::<i32>().ok()).collect()
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
}
