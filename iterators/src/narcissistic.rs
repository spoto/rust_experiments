use std::iter::successors;

fn is_narcissistic(x: u64) -> bool {
    let mut digits = 0;
    let mut y = x;
    while y > 0 {
        y = y / 10;
        digits += 1;
    };
    y = x;
    let mut sum: u64 = 0;
    while y > 0 {
        sum += pow(y % 10, digits);
        y = y / 10;
    };
    x == sum
}

fn pow(base: u64, exp: i32) -> u64 {
    let mut e = exp;
    let mut result = 1;
    while e > 0 {
        e -= 1;
        result *= base;
    };
    result
}

#[test]
fn test_narcissistic() {
    assert!(is_narcissistic(0));
    assert!(is_narcissistic(1));
    assert!(!is_narcissistic(10));
    assert!(is_narcissistic(153));
    assert!(!is_narcissistic(154));
}

#[test]
fn test_catch17() {
    let ns: Vec<u64> = successors(Some(10u64), |&p| Some(p + 1))
        .filter(|x| is_narcissistic(*x))
        .take(17)
        .collect();

    assert_eq!(ns, vec![153, 370, 371, 407, 1634, 8208, 9474, 54748, 92727, 93084, 548834, 1741725, 4210818, 9800817, 9926315, 24678050, 24678051]);
}