use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList};

fn triangle(n: u64) -> u64 {
    (1..=n).sum()
}

#[test]
fn test_sum() {
    assert_eq!(triangle(20), 210);
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

#[test]
fn test_product() {
    assert_eq!(factorial(20), 2432902008176640000);
}

#[test]
fn test_min() {
    let m = [-2, 0, 1, -2, -5].iter().min();
    assert_eq!(m, Some(&-5));
}

#[test]
fn test_max() {
    let m = ["ciao", "amico", "come", "va", "oggi?"].iter().max();
    assert_eq!(m, Some(&"va"));
}

fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

#[test]
fn test_max_by() {
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));
}

#[test]
fn test_min_by_key() {
    let mut populations = HashMap::new();
    populations.insert("Portland",  583_776);
    populations.insert("Fossil",        449);
    populations.insert("Greenhorn",       2);
    populations.insert("Boring",      7_762);
    populations.insert("The Dalles", 15_340);

    let largest = populations.iter()
        .max_by_key(|&(_name, pop)| pop);
    let smallest = populations.iter()
        .min_by_key(|&pair| pair.1);
    assert_eq!(largest, Some((&"Portland", &583_776)));
    assert_eq!(smallest, Some((&"Greenhorn", &2)));
}

#[test]
fn test_comparisons() {
    let packed  = "Helen of Troy";
    let spaced  = "Helen   of   Troy";
    let longer  = "Helen of Troy in Asia";
    let obscure = "Helen of Sandusky";

    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
    assert!(spaced.split_whitespace().lt(longer.split_whitespace()));
}

#[test]
fn test_any_all() {
    let id = "Verona";
    assert!( id.chars().by_ref().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));
}

#[test]
fn test_position() {
    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);
    assert_eq!(text.chars().position(|c| c == 'X'), Some(0));

    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
    assert_eq!(bytes.iter().rposition(|&c| c == b'z'), None);
    assert_eq!(bytes.iter().rposition(|&c| c == b'X'), Some(0));
}

#[test]
fn test_fold() {
    let a = ["Pack", "my", "box", "with", "five", "dozen", "liquor", "jugs"];
    let pangram = a.iter()
        .fold(String::new(), |s, w| s + w + " ");
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");
    let weird_pangram = a.iter()
        .rfold(String::new(), |s, w| s + w + " ");
    assert_eq!(weird_pangram, "jugs liquor dozen five with box my Pack ");
}

#[test]
fn test_nth() {
    let mut squares = (0..10).map(|i| i * i);
    assert_eq!(squares.nth(4), Some(16));
    assert_eq!(squares.nth(0), Some(25));
    assert_eq!(squares.nth(6), None);
}

#[test]
fn test_find() {
    let mut populations = HashMap::new();
    populations.insert("Portland",  583_776);
    populations.insert("Fossil",        449);
    populations.insert("Greenhorn",       2);
    populations.insert("Boring",      7_762);
    populations.insert("The Dalles", 15_340);

    assert_eq!(populations.iter().find(|&(_name, &pop)| pop > 1_000_000), None);
    assert_eq!(populations.iter().find(|&(_name, &pop)| pop > 500_000), Some((&"Portland", &583_776)));
}

#[test]
fn test_collect() {
    let words = ["hello".to_string(), "friend".to_string(), "how".to_string(), "are you?".to_string()];

    let x: HashSet<_> = words.iter().collect();
    assert_eq!(x.len(), 4);
    let y = words.iter().collect::<BTreeSet<_>>();
    assert_eq!(y.len(), 4);
    let z = words.iter().collect::<LinkedList<&String>>();
    assert_eq!(z.len(), 4);
    let w = words.iter().zip(1..).collect::<HashMap<_, _>>();
    assert_eq!(w.len(), 4);
    let s: BTreeMap<&String,i32> = words.iter().zip(1..).collect();
    assert_eq!(s.len(), 4);
}

#[test]
fn test_extend() {
    let mut v: Vec<i32> = (0..5).map(|i| 1 << i).collect();
    v.extend(&[31, 57, 99, 163]);
    assert_eq!(v, &[1, 2, 4, 8, 16, 31, 57, 99, 163]);
}

#[test]
fn test_partition() {
    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];
    let (living, nonliving): (Vec<&str>, Vec<&str>) = things.iter()
        .partition(|name| name.as_bytes()[0] & 1 != 0);
    assert_eq!(living,    &["mushroom", "giraffe", "grapefruit"]);
    assert_eq!(living,    ["mushroom", "giraffe", "grapefruit"]);
    assert_eq!(living,    vec!["mushroom", "giraffe", "grapefruit"]);
    assert_eq!(nonliving, vec!["doorknob", "noodle"]);
}

#[test]
fn test_for_each() {
    let mut s = String::new();

    ["doves", "hens", "birds"].iter()
        .zip(["turtle", "french", "calling"].iter())
        .zip(2..5)
        .rev()
        .map(|((item, kind), quantity)|
            format!("{} {} {}", quantity, kind, item)
        )
        .for_each(|gift| {
            s.push_str("You have received: ");
            s.push_str(gift.as_str());
            s.push('\n');
        });

    println!("{}", s);
    assert_eq!(s, "\
            You have received: 4 calling birds\n\
            You have received: 3 french hens\n\
            You have received: 2 turtle doves\n\
    ");
}