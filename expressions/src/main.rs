use std::ops::{Range, RangeInclusive, RangeTo};

fn main() {
    println!("Hello, world!");
    for_each();
    for_each2();
    selection();
    ranges();
    star()
}

fn only_declaration(j: i32) {
    let mut a = 13;

    {
        //let b = 17 // this requires a semicolon at the end
    }

    let k = if j > a {
        13
    }
    else { // without this else, doesn't compile
        17
    };

    if j > a {
        // 13 // this makes it not compile
    };

    {
        13
    };
}

fn for_each() {
    let strings = vec!["ciao".to_string(), "amico".to_string(), "come".to_string(), "stai?".to_string()];
    for s in &strings {
        println!("String {:?} is at address {:p}.", *s, s)
    }
}

fn for_each2() {
    let strs = vec!["ciao", "amico", "come", "stai?"];
    for s in &strs {
        println!("String {:?} is at address {:p}.", *s, s)
    }
}

fn never_ending(mut a: i32) -> ! {
    loop {
        if a > 100 {
            // break; // with this, it doesn't compile
        }

        a = a + 1;
    }
}

fn generic() {
    let v = Vec::<i32>::with_capacity(1000);
}

fn selection() {
    let strings = ["ciao".to_string(), "amico".to_string(), "come".to_string(), "stai?".to_string()];
    let x = &strings[1..3];
    println!("x is {:?}.", x);
}

fn ranges() {
    let x = Range { start: 10, end: 15 };
    let y = RangeInclusive::new(10, 15);
    let z = RangeTo { end: 15};
    let k = ..15;
    println!("x is {:?}.", x);
    println!("y is {:?}.", y);
    println!("z is {:?}.", z);
    println!("k is {:?}.", k);

    let strings = ["ciao".to_string(), "amico".to_string(), "come".to_string(), "stai?".to_string()];
    let xs = &strings[..4];
    println!("xs is {:?}.", xs);
}

fn star() {
    let strings = ["ciao".to_string(), "amico".to_string(), "come".to_string(), "stai?".to_string()];
    for elem in &strings {
        //let s = *elem; // this is a forbidden move
        //foo(*elem); // this is a forbidden move
        println!("elem: {}", *elem); // why is this not a (forbidden) move?
    }
    println!("strings: {:?}", strings)
}

fn foo(s: string) {

}