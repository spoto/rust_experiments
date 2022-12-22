use std::cmp::Ordering;

fn triangle(n: u64) -> u64 {
    (1..=n).sum()
}

#[test]
fn test_sum() {
    assert_eq!(triangle(20), 210);
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

#[test]
fn test_product() {
    assert_eq!(factorial(20), 2432902008176640000);
}

#[test]
fn test_min() {
    let m = [-2, 0, 1, -2, -5].iter().min();
    assert_eq!(m, Some(&-5));
}

#[test]
fn test_max() {
    let m = ["ciao", "amico", "come", "va", "oggi?"].iter().max();
    assert_eq!(m, Some(&"va"));
}

fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

#[test]
fn test_max_by() {
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));
}