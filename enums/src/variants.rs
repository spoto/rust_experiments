#[derive(Eq, PartialEq)]
enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        reason: String,
        since: u8
    }
}

#[test]
fn test_relationship() {
    let rs1 = RelationshipStatus::ItsComplicated(Option::Some("polygamy".to_string()));
    // println!("{}", RelationshipStatus::Single as i32); // works only for unit-like variants
    let rs2 = RelationshipStatus::ItsExtremelyComplicated { reason: "married".to_string(), since: 10 };
    assert!(rs1 != rs2);
}

fn foo(i: i32) -> i32 {
    match i {
        x if x % 2 == 0 => 13,
        // x if x % 2 == 1 => 17 // this seems non-exhaustive to the compiler
        _ => 17
    }
}

#[test]
fn test_exhaustive() {
    assert_eq!(foo(0), 13);
    assert_eq!(foo(13), 17);
}

fn strange(j @ &i: &i32) {
    assert_eq!(i, *j);
}

#[test]
fn test_strange() {
    strange(&13);
    let j = 17;
    strange(&j);
}