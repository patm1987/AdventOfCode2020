const ROWS: (i32, i32) = (0, 127);
const COLS: (i32, i32) = (0, 7);

fn main() {
    println!("Hello, world!");
}

fn lower(input: (i32, i32)) -> (i32, i32) {
    (input.0, (input.0 + input.1) / 2)
}

fn upper(input: (i32, i32)) -> (i32, i32) {
    ((input.0 + input.1) / 2 + 1, input.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * Sample:
     * BFFFBBFRRR: row 70, column 7, seat ID 567.
     * FFFBBBFRRR: row 14, column 7, seat ID 119.
     * BBFFBBFRLL: row 102, column 4, seat ID 820.
     */

    #[test]
    fn test_finds_lower_range() {
        assert_eq!(lower(ROWS), (0, 63));
    }

    #[test]
    fn test_finds_lower_range_in_middle() {
        assert_eq!(lower((32, 63)), (32, 47));
    }

    #[test]
    fn test_finds_upper_range() {
        assert_eq!(upper(ROWS), (64, 127))
    }

    #[test]
    fn test_finds_upper_range_in_middle() {
        assert_eq!(upper((32, 47)), (40, 47))
    }
}