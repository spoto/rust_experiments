use std::fmt::Display;

struct MyBox<T: ?Sized> {
    ref_count: usize,
    value: T
}

#[test]
fn test_boxed_lunch() {
    let boxed_lunch = MyBox {
        ref_count: 1,
        value: "lunch".to_string()
    };

    assert_eq!(boxed_lunch.value, "lunch")
}

#[test]
fn test_boxed_conversion() {
    let boxed_lunch = MyBox {
        ref_count: 1,
        value: "lunch".to_string()
    };

    let boxed_displayable: &MyBox<dyn Display> = &boxed_lunch;

    // you cannot pass boxed_displayable.value since it is unsized,
    // but you can pass a reference, which is sized
    assert_eq!(format!("For your enjoyment: {}", &boxed_displayable.value),
        "For your enjoyment: lunch");
}