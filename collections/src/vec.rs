#[test]
fn test_mut() {
    let mut v = vec![0, 1, 2, 3];
    let last = v.last_mut().unwrap();
    assert_eq!(*last, 3);
    *last = 100;
    assert_eq!(v, [0, 1, 2, 100]);
}