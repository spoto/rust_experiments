struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

// receives b by value: takes ownership
fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom { height: b.height / 2, ..b };
    let mut broom2 = Broom { name: broom1.name.clone(), ..broom1 };
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

#[test]
fn chop_broom_test() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);

    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);
}