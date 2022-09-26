fn main() {
    println!("Run the tests in this program: \"cargo test\" or \"cargo test --release\"")
}

#[test]
fn string_tuple() {
    let text = "I see the eigenvalue in thine eyes";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eyes");
}

// in debug mode, this panics; in release mode, computes -32768...
#[test]
#[should_panic]
fn print_abs() {
    let i:i16 = -32768;
    println!("abs({}) = {}", i, i.abs());
}

#[test]
#[should_panic]
// attempts to forge chars from illegal UNICODE code points are caught at run time
fn print_chars() {
    let mut c = '\u{d700}';
    for _offset in 0..1000 {
        println!("UNICODE {}: {}", c as u32, c);
        c = char::from_u32(c as u32 + 1).expect("out of UNICODE range");
    }
}

// this panics in debug mode and diverges in release mode!
#[test]
#[should_panic]
fn infinite() {
    let mut _i = 1;
    loop {
        _i *= 10;
    }
}

#[test]
#[should_panic]
// this panics in any case; corresponds to the behaviour in a debug build
fn checked_infinite() {
    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}

#[allow(dead_code)]
// this diverges in any case: corresponds to the behaviour in a release build
fn wrapping_infinite() {
    let mut i: i32 = 1;
    loop {
        i = i.wrapping_mul(10);
    }
}

#[test]
fn test_tuples() {
    let t1 = (13, 17);
    let t2 = (13, 17);
    assert_eq!(t1, t2);
}

#[test]
fn test_assign_arrays() {
    let arr1 = [1, 2, 3, 4];
    let arr2 = arr1; // makes a copy of arr1
    assert_eq!(arr1, arr2);
}

#[test]
fn test_assign_modify_arrays() {
    let arr1 = [1, 2, 3, 4];
    let mut arr2 = arr1; // makes a copy of arr1
    arr2[2] = 5;
    assert_ne!(arr1, arr2);
}