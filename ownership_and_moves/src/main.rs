fn main() {
    println!("Hello, world!");
}

struct Person { name: String, birth: i32 }

/*#[test]
fn test_struct_assign() {
    let p1 = Person { name: "Palestrina".to_string(), birth: 1525 };
    let p2 = p1;
    // it does not compile since p1 has been moved
    println!("{}, {}", p1.name, p2.name);
}*/

#[test]
fn test_struct_assign2() {
    let p1 = Person { name: "Palestrina".to_string(), birth: 1525 };
    let n = p1.name;
    // I can still use p1.birth although p1.name has been moved and p1 cannot be used anymore
    println!("{}, {}", n, p1.birth);
}

#[test]
fn test_composers() {
    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(), birth: 1563 });
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });

    // without the borrow, this would work, but the next loop would find composers moved
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

#[test]
fn test_move_from_vector() {
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // 2. Move a value out of a given index in the vector,
    //    and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Let's see what's left of our vector.
    let v2 = vec!["101", "104", "substitute"];
    assert_eq!(v, v2);
}

#[test]
fn test_rc() {
    use std::rc::Rc;

    let s = Rc::new("shirataki".to_string());
    let t = s.clone();
    let u = s.clone();
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    assert_eq!("shirataki", format!("{}", u));
}