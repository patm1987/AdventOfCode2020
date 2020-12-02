use std::collections::HashSet;

pub struct ReportTracker {
    target_sum: i32,
    looking_for: HashSet<i32>,
}

pub fn build_report_tracker(target_sum: i32) -> ReportTracker {
    ReportTracker {
        target_sum,
        looking_for: HashSet::new(),
    }
}

impl ReportTracker {
    pub fn add_expense(&mut self, value: i32) -> (bool, i32) {
        if self.looking_for.contains(&value) {
            return (true, (self.target_sum - value) * value);
        } else if value < self.target_sum {
            self.looking_for.insert(self.target_sum - value);
        }
        (false, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_returns_empty() {
        let mut report = build_report_tracker(2);
        let (matched, total) = report.add_expense(1337);
        assert_eq!(matched, false);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_finds_match() {
        let mut report = build_report_tracker(3);
        assert_eq!((false, 0), report.add_expense(1));
        assert_eq!((true, 2), report.add_expense(2));
    }

    #[test]
    fn test_finds_match_if_first_large() {
        let mut report = build_report_tracker(3);
        assert_eq!((false, 0), report.add_expense(5));
        assert_eq!((false, 0), report.add_expense(1));
        assert_eq!((true, 2), report.add_expense(2));
    }

    #[test]
    fn test_finds_match_if_first_misleading() {
        let mut report = build_report_tracker(4);
        assert_eq!((false, 0), report.add_expense(2));
        assert_eq!((false, 0), report.add_expense(3));
        assert_eq!((true, 3), report.add_expense(1));
    }
}
