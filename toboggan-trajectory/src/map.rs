#[derive(Debug, PartialEq)]
pub struct Map {
    width: i32,
    height: i32,
    map: Vec<char>,
}

pub fn build_map(input: &str) -> Map {
    let lines: Vec<&str> = input.split('\n').collect();
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;
    Map {
        width,
        height,
        map: lines.iter().fold(String::new(), |acc, x| acc + x).chars().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let m1 = Map {
            width: 1,
            height: 2,
            map: vec!['a', 'b', 'c'],
        };

        let m2 = Map {
            width: 1,
            height: 2,
            map: vec!['a', 'b', 'c'],
        };

        assert_eq!(m1, m2);
    }

    #[test]
    fn test_builds_single_row() {
        let input = ".#";

        assert_eq!(Map { width: 2, height: 1, map: vec!['.', '#'] }, build_map(input))
    }
}