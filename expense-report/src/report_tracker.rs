pub struct ReportTracker {
    target_sum: i32,
    looking_for: Option<i32>,
}

pub fn build_report_tracker(target_sum: i32) -> ReportTracker {
    ReportTracker {
        target_sum,
        looking_for: None,
    }
}

impl ReportTracker {
    pub fn add_expense(&mut self, value: i32) -> (bool, i32) {
        match self.looking_for {
            Some(x) => {
                if value == x {
                    return (true, (self.target_sum - x) * x);
                } else {
                    return (false, 0);
                }
            }
            None => {
                self.looking_for = Some(self.target_sum - value);
                return (false, 0);
            }
        }
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
}
