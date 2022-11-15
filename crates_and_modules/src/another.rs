pub mod inner;

pub struct X { // if not pub, it would be an error for the foo() function signature

}

pub fn foo() -> X {
    X{}
}