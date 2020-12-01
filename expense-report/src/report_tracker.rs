pub struct ReportTracker {
}

pub fn build_report_tracker() -> ReportTracker {
    ReportTracker{}
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
        let report = build_report_tracker();
        let (matched, total) = report.add_expense(1337);
        assert_eq!(matched, false);
        assert_eq!(total, 0);
    }
}