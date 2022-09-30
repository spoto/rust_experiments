use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
}

#[test]
fn test_hashmap() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);

    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    sort_works(&mut table);
    show(&table);
}

fn show(table: &HashMap<String, Vec<String>>) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

struct Anime {
    name: &'static str,
    #[allow(dead_code)]
    bechdel_pass: bool
}

#[test]
fn test_ref_deref() {
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    //let a = *anime_ref; // doesn't compile since I have no ownership of *anime_ref
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}

#[test]
fn test_assign_ref() {
    let mut x = 10;
    let mut y = 20;
    let mut r: &mut i32;
    if x * y > 10 {
        r = &mut x;
    }
    else {
        r = &mut y;
    }

    if *r == 10 {
        r = &mut y;
    }

    *r = *r + 1;

    assert!(*r == 11 || *r == 21);
}

#[test]
fn test_eq_references() {
    type Refint<'a> = &'a i32;

    let x = 13;
    let y = 13;
    let rx = &x;
    let ry: Refint = &y;
    assert_eq!(rx, ry); // looks at the final pointed values
}

fn factorial(n: usize) -> usize {
    (1..n+1).product()
}

#[test]
fn test_non_leftvalue_refs() {
    // the following is synonym to
    // let temp = factorial(6);
    // let r = &temp;
    let r = &factorial(6);

    assert_eq!(r + 1009, 1729);

    // the following is synonym to
    // {
    //   let temp = 1009;
    //   assert_eq!(r + temp, 1729);
    // }
    assert_eq!(r + &1009, 1729); // arithmetic operators dereference only once
    // assert_eq!(r + &&1009, 1729); // doesn't compile
}

#[test]
fn test_non_leftvalue_refs2() {
    let r = &&factorial(6);
    assert_eq!(r, &&720); // equality dereferences arbitrarily....
}

#[test]
fn test_lifetimes() {
    let mut r;
    let y = 17;
    {
        let x = 1;
        r = &x;
        r = &y; // by replacing the reference in r, everything is OK
    }
    assert_eq!(*r, 17);
}

/*
#[test]
fn test_lifetimes_broken() {
    let mut r;
    {
        let x = 1;
        r = &x;
    }
    assert_eq!(*r, 17);
}
*/