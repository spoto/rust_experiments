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