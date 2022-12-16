#[test]
fn test_str_iter() {
    let s = "èéòàçù";
    let mut count_as_chars = 0;
    let mut count_as_bytes = 0;

    for _ in s.bytes() {
        count_as_bytes += 1;
    }

    for _ in s.chars() {
        count_as_chars += 1;
    }

    assert_eq!(count_as_bytes, 12);
    assert_eq!(s.bytes().count(), 12);
    assert_eq!(count_as_chars, 6);
    assert_eq!(s.chars().count(), 6);
}