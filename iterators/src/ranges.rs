#[test]
fn test_range() {
    let r = 13..=17;
    let mut sum = 0;
    for i in r {
        sum += i;
    }
    assert_eq!(sum, 75);
    // I cannot use r again since it has been consumed
    /*for i in r {
        println!("{}", i);
    }*/
}