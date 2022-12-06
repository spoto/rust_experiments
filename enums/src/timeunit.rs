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

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32)
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(TimeUnit::Hours, 1) =>
            format!("an hour ago"),
        RoughTime::InThePast(units, 1) =>
            format!("a {} ago", units.singular()),
        RoughTime::InThePast(units, count) =>
            format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow =>
            format!("just now"),
        RoughTime::InTheFuture(TimeUnit::Hours, 1) =>
            format!("an hour from now"),
        RoughTime::InTheFuture(units, 1) =>
            format!("a {} from now", units.singular()),
        RoughTime::InTheFuture(units, count) =>
            format!("{} {} from now", count, units.plural())
    }
}

#[test]
fn test_to_english() {
    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Days, 2)),
               "2 days ago".to_string());
    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Days, 1)),
               "a day ago".to_string());
    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Hours, 1)),
               "an hour ago".to_string());
    assert_eq!(rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Days, 2)),
               "2 days from now".to_string());
    assert_eq!(rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Days, 1)),
               "a day from now".to_string());
    assert_eq!(rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Hours, 1)),
               "an hour from now".to_string());
    assert_eq!(rough_time_to_english(RoughTime::JustNow),
               "just now".to_string());
}