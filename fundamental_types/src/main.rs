fn main() {
    // in debug mode, this panics; in release mode, computes -32768...
    //print_abs(-32768);

    // attempts to forge chars from illegal UNICODE code points are caught at run time
    //print_chars('\u{d700}');

    checked_infinite();
}

fn print_abs(i: i16) {
    println!("abs({}) = {}", i, i.abs());
}

fn print_chars(mut c: char) {
    for offset in 0..1000 {
        println!("UNICODE {}: {}", c as u32, c);
        c = char::from_u32(c as u32 + 1).expect("out of UNICODE range");
    }
}

// this panics in debug mode and diverges in release mode!
fn infinite() {
    let mut i = 1;
    loop {
        i *= 10;
    }
}

// this panics in any case; corresponds to the behaviour in a debug build
fn checked_infinite() {
    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}

// this diverges in any case: corresponds to the behaviour in a release build
fn wrapping_infinite() {
    let mut i: i32 = 1;
    loop {
        i = i.wrapping_mul(10);
    }
}