use std::collections::HashSet;

#[test]
fn test_mut() {
    let mut v = vec![0, 1, 2, 3];
    let last = v.last_mut().unwrap();
    assert_eq!(*last, 3);
    *last = 100;
    assert_eq!(v, [0, 1, 2, 100]);
}

#[test]
fn test_extend() {
    let mut vec1 = vec!["ciao", "amico", "come va?"];
    let vec2 = vec!["hello", "friend"];
    vec1.extend(vec2);
    assert_eq!(vec1, vec!["ciao", "amico", "come va?", "hello", "friend"]);
    //println!("{:?}", vec2); // cannot use vec2 anymore: it has been moved
}

#[test]
fn test_append() {
    let mut vec1 = vec!["ciao", "amico", "come va?"];
    let mut vec2 = vec!["hello", "friend"];
    vec1.append(&mut vec2);
    assert_eq!(vec1, vec!["ciao", "amico", "come va?", "hello", "friend"]);
    assert!(vec2.is_empty()); // can still use vec2
}

#[test]
fn test_dedup() {
    let mut byte_vec = b"Misssssssissippi".to_vec();
    byte_vec.dedup();
    assert_eq!(&byte_vec, b"Misisipi");
}

#[test]
fn test_retain() {
    let mut byte_vec = b"Misssssssissippi".to_vec();
    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));
    assert_eq!(&byte_vec, b"Misp");
}

#[test]
fn test_borrow() {
    let v = vec![0, 1, 2, 3];
    let a = &v[1];
    let b = &v[3];
    let mid = v.len() / 2;
    let _front_half = &v[..mid];
    let _back_half = &v[mid..];
    assert_eq!(*a, 1);
    assert_eq!(*b, 3);
}

#[test]
fn test_shuffle() {
    use rand::seq::SliceRandom;
    let mut v = vec![0, 11, -2, 34, 11, -3];
    let sum: i32 = v.iter().sum();
    v.shuffle(&mut rand::thread_rng());
    assert_eq!(v.iter().sum::<i32>(), sum);
}