struct MyType {
    i: i32,
    j: f64
}

impl Clone for MyType {
    fn clone(&self) -> Self {
        MyType {
            i: self.i,
            j: self.j
        }
    }
}

impl Copy for MyType {}

#[test]
fn test_copy() {
    let mt1 = MyType { i: 13, j: 3.1415 };
    let mt2 = mt1;
    let mt3 = mt1; // mt1 has not been moved so I can use it again
    assert_eq!(mt2.i, mt3.i);
}