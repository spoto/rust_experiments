use std::collections::{BTreeMap, HashMap};
use rand::random;
use num::Complex;
use std::iter::{from_fn, Peekable, successors};
use std::str::FromStr;

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

#[test]
fn test_map_filter() {
    let text = "  ponies   \n  giraffes\niguanas   \nsquid".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);
}

#[test]
fn test_do_nothing() {
    let mut counter = 0;
    let arr = ["earth", "water", "air", "fire"];
    let _v = arr.iter()
        .map(|_elt| counter += 1);
    assert_eq!(counter, 0);
    arr.iter()
        .map(|_elt| counter += 1)
        .count();
    assert_eq!(counter, 4);
}

#[test]
fn test_filter_map() {
    let text = "1\nfrond .25   289\n3.1415 estuary\n";
    text.split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
        .map(f64::sqrt)
        .for_each(|number| println!("{:4.2}", number));
}

#[test]
fn test_flat_map() {
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    let _ = countries.iter()
        .flat_map(|country| &major_cities[*country])
        .for_each(|city| println!("{}", *city));
}

#[test]
fn test_flatten() {
    let mut parks = BTreeMap::new();
    parks.insert("Portland",  vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto",     vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);
    let all_parks: Vec<_> = parks.values()
        .flatten()
        .cloned()
        .collect();

    assert_eq!(all_parks,
        vec!["Tadasu-no-Mori Forest", "Maruyama Koen", "Percy Warner Park", "Dragon Park", "Mt. Tabor Park", "Forest Park"]);
}

#[test]
fn test_flatten_options() {
    let v1 = vec![None, Some("day"), None, Some("one")];
    let v2 = v1.into_iter()
        .flatten()
        .collect::<Vec<_>>();
    assert_eq!(v2, vec!["day", "one"]);
}

#[test]
fn test_take_while() {
    let message = "To: fausto.spoto@....\r\n\
                         From: superego <editor@orelly.com>\r\n\
                         \r\n\
                         Did you get any writing down today?\r\n\
                         When will you stop wasting time?\r\n";
    let v: Vec<&str> = message.lines()
        .take_while(|l| !l.is_empty())
        .collect();
    assert_eq!(v, vec!["To: fausto.spoto@....", "From: superego <editor@orelly.com>"]);
}

#[test]
fn test_skip_while() {
    let message = "To: fausto.spoto@....\r\n\
                         From: superego <editor@orelly.com>\r\n\
                         \r\n\
                         Did you get any writing down today?\r\n\
                         When will you stop wasting time?\r\n";
    let v: Vec<&str> = message.lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .collect();
    assert_eq!(v, vec!["Did you get any writing down today?", "When will you stop wasting time?"]);
}

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where I: Iterator<Item=char>
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n
        }
        tokens.next();
    }
}

fn parse_number2<I>(tokens: &mut I) -> u32
    where I: Iterator<Item=char>
{
    tokens
        .take_while(|&c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |n, c| (n * 10 + c))
}

#[test]
fn test_parse_number() {
    assert_eq!(parse_number(&mut "1234hello".chars().peekable()), 1234);
    assert_eq!(parse_number(&mut "a1234hello".chars().peekable()), 0);
    assert_eq!(parse_number2(&mut "1234hello".chars()), 1234);
    assert_eq!(parse_number2(&mut "a1234hello".chars()), 0);
}

#[test]
fn test_fuse() {
    struct Flaky {
        state: bool
    }

    impl Flaky {
        fn new() -> Flaky {
            Flaky { state: true }
        }
    }

    impl Iterator for Flaky {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.state {
                self.state = false;
                Some("totally the last item")
            } else {
                self.state = true;
                None
            }
        }
    }

    let mut flaky = Flaky::new();
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut not_flaky = Flaky::new().fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);
}

#[test]
fn test_next_back() {
    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();

    assert_eq!(iter.next(),      Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(),      Some(&"thorax"));

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next()     , None);
}

#[test]
fn test_rev() {
    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next(), Some(&"breakfast"));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_inspect() {
    let upper_case: String = "große".chars()
        .inspect(|c| println!("before: {}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after: {}", c))
        .collect();

    assert_eq!(upper_case, "GROSSE");
}

#[test]
fn test_chain() {
    let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
    assert_eq!(v, [40, 30, 20, 3, 2, 1]);
}

#[test]
fn test_zip() {
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, [(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);
}

#[test]
fn test_by_ref() {
    let message = "To: fausto.spoto@....\r\n\
                         From: superego <editor@orelly.com>\r\n\
                         \r\n\
                         Did you get any writing down today?\r\n\
                         When will you stop wasting time?\r\n";

    let mut lines = message.lines();
    let mut s: String = "".to_string();

    s.push_str("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        s.push_str(header);
    }

    s.push_str("\nBody:");
    for body in lines {
        s.push_str(body);
    }

    assert_eq!(s,
        "Headers:To: fausto.spoto@....From: superego <editor@orelly.com>\n\
        Body:Did you get any writing down today?When will you stop wasting time?"
    );
}

#[test]
fn test_by_ref2() {
    let message = "To: fausto.spoto@....\r\n\
                         From: superego <editor@orelly.com>\r\n\
                         \r\n\
                         Did you get any writing down today?\r\n\
                         When will you stop wasting time?\r\n";

    let mut lines = message.lines();
    let mut s: String = "".to_string();

    s.push_str("Headers:");
    for header in (&mut lines).take_while(|l| !l.is_empty()) {
        s.push_str(header);
    }

    s.push_str("\nBody:");
    for body in lines {
        s.push_str(body);
    }

    assert_eq!(s,
               "Headers:To: fausto.spoto@....From: superego <editor@orelly.com>\n\
        Body:Did you get any writing down today?When will you stop wasting time?"
    );
}

#[test]
fn test_cycle() {
    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
}