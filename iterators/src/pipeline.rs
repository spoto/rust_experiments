use rand::random;
use num::Complex;
use std::iter::{from_fn, successors};

#[test]
fn test_from_fn() {
    let lengths: Vec<f64> =
        from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
            .take(1000)
            .collect();

    assert_eq!(lengths.len(), 1000);
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let zero = Complex {re: 0.0, im: 0.0 };
    successors(Some(zero), |&z| Some(z * z + c))
        .take(limit)
        .enumerate()
        .find(|(_, z)| z.norm_sqr() > 4.0)
        .map(|(i, _z)| i)
}

#[test]
fn test_escape_time() {
    assert_eq!(None, escape_time(Complex {re: 0.3, im: 0.1 }, 1000));
    assert_eq!(Some(36), escape_time(Complex {re: 0.38, im: 0.1 }, 1000));
}

fn fibonacci() -> impl Iterator<Item=usize> {
    let mut state = (0, 1);
    from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci().take(8).collect::<Vec<_>>(),
        vec![1, 1, 2, 3, 5, 8, 13, 21])
}

#[test]
fn test_drain() {
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
}