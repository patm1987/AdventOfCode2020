const ROWS: (i32, i32) = (0, 127);
const COLS: (i32, i32) = (0, 7);

pub struct Seat {
    row: i32,
    col: i32,
    id: i32,
}

impl Seat {
    pub fn find_seat(code: &str) -> Seat {
        let (row_search, col_search) = code.split_at(7);
        let row = row_search.chars().fold(ROWS, |acc, x| {
            match x {
                'F' => lower(acc),
                'B' => upper(acc),
                _ => acc
            }
        });
        let col = col_search.chars().fold(COLS, |acc, x| {
            match x {
                'L' => lower(acc),
                'R' => upper(acc),
                _ => acc
            }
        });
        Seat {
            row: row.0,
            col: col.0,
            id: row.0 * 8 + col.0,
        }
    }

    pub fn get_row(&self) -> i32 { self.row }
    pub fn get_col(&self) -> i32 { self.col }
    pub fn get_id(&self) -> i32 { self.id }
}

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

    #[test]
    fn test_finds_row() {
        assert_eq!(70, Seat::find_seat("BFFFBBFRRR").get_row())
    }

    #[test]
    fn test_finds_col() {
        assert_eq!(7, Seat::find_seat("BFFFBBFRRR").get_col())
    }

    #[test]
    fn test_finds_id() {
        assert_eq!(567, Seat::find_seat("BFFFBBFRRR").get_id())
    }
}