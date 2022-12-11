use std::cmp::Ordering;

// this implements the PartialEq trait but only if T implements PartialEq itself
#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T,
    upper: T
}

impl<T> Interval<T> {
    fn new(lower: T, upper: T) -> Interval<T> {
        Interval { lower, upper }
    }
}

fn is_eq<T: PartialEq>(i: &Interval<T>, j: &Interval<T>) -> bool {
    i == j
}

#[test]
fn test_is_eq() {
    let i = Interval { lower: 13, upper: 5 };
    let j = Interval { lower: 11, upper: 5 };
    let k = Interval { lower: 13, upper: 5 };
    assert!(is_eq(&i, &k));
    assert!(!is_eq(&i, &j));
}

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[test]
fn test_partial_ord() {
    assert!(Interval::new(10, 20) < Interval::new(20, 40));
    assert!(Interval::new(7, 8) >= Interval::new(0, 1));
    assert!(Interval::new(7, 8) <= Interval::new(7, 8));
    let left = Interval::new(10, 30);
    let right = Interval::new(20, 40);
    assert!(!(left < right));
    assert!(!(left >= right));
}