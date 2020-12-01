pub struct ReportTracker {
    target_sum: u32,
}

pub fn build_report_tracker(target_sum: u32) -> ReportTracker {
    ReportTracker { target_sum }
}

impl ReportTracker {
    pub fn add_expense(&self, value: u32) -> (bool, u32) {
        (false, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_returns_empty() {
        let report = build_report_tracker(2);
        let (matched, total) = report.add_expense(1337);
        assert_eq!(matched, false);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_finds_match() {
        let report = build_report_tracker(2);
        assert_eq!((false, 0), report.add_expense(1));
        assert_eq!((true, 1), report.add_expense(1));
    }
}
