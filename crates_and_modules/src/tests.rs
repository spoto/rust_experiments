#![cfg(test)]
// without ! the attribute applies to the following struct X only
// and a warning would be generated for roughly_equal

#[allow(dead_code)]
struct X {}

fn roughly_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

#[test]
fn trig_works() {
    use std::f64::consts::PI;
    assert!(roughly_equal(PI.sin(), 0.0));
}