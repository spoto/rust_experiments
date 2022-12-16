#[test]
fn test_kill() {
    let my_str = "hello".to_string();
    //let f = || drop(my_str);
    let f = || { let _x = my_str; };
    f();
    //f(); // cannot call twice since the closure moves a value
}