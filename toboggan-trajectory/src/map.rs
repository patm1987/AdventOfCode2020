#[derive(Debug, PartialEq)]
pub struct Map {
    width: i32,
    height: i32,
    map: Vec<char>,
}

pub fn build_map(input: &str) -> Map {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;
    Map {
        width,
        height,
        map: lines.iter().fold(String::new(), |acc, x| acc + x).chars().collect(),
    }
}

impl Map {
    pub fn get(&self, x: i32, y: i32) -> char {
        return self.map[(x % self.width + y * self.width) as usize];
    }

    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
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

    #[test]
    fn test_builds_two_rows() {
        let input = ".#\n..";
        assert_eq!(Map { width: 2, height: 2, map: vec!['.', '#', '.', '.'] }, build_map(input))
    }

    #[test]
    fn test_can_index() {
        let input = "..\n.#";
        let map = build_map(input);
        assert_eq!('#', map.get(1, 1))
    }

    #[test]
    fn test_index_with_wrap() {
        let input = "..\n.#";
        let map = build_map(input);
        assert_eq!('#', map.get(3, 1))
    }

    #[test]
    fn test_get_width() {
        let input = "..";
        let map = build_map(input);
        assert_eq!(2, map.width());
    }

    #[test]
    fn test_get_height() {
        let input = "..\n..\n..";
        let map = build_map(input);
        assert_eq!(3, map.height());
    }

    #[test]
    fn test_handles_empty_row() {
        let input = "..\n..\n";
        let map = build_map(input);
        assert_eq!(2, map.height());
    }
}