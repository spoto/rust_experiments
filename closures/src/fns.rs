fn call_twice<F>(mut closure: F) where F: FnMut() {
    closure();
    closure();
}

#[test]
fn test_call_twice() {
    let mut i = 0;
    call_twice(|| i += 1);
    assert_eq!(i, 2);
}

#[test]
fn test_clone() {
    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        greeting
    };
    assert_eq!("Hello, Alfred", greet.clone()("Alfred"));
    assert_eq!("Hello, Bruce", greet.clone()("Bruce"));
}