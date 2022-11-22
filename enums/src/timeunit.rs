#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[test]
fn test_plural() {
    assert_eq!(TimeUnit::Months.plural(), "months");
    assert_eq!(TimeUnit::Hours.plural(), "hours");
}

#[test]
fn test_singular() {
    assert_eq!(TimeUnit::Months.singular(), "month");
    assert_eq!(TimeUnit::Hours.singular(), "hour");
}