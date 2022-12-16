#[test]
fn test_vector() {
    let v = vec!["antimony", "arsenic", "aluminium", "selenium"];

    let mut result = "".to_string();
    for element in &v {
        result.push_str(element);
    }

    assert_eq!("antimonyarsenicaluminiumselenium", result);

    result = "".to_string();

    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        result.push_str(element);
    }

    assert_eq!("antimonyarsenicaluminiumselenium", result);
}