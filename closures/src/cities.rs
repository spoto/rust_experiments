use std::thread;

struct City {
    name: String,
    population: i64,
    country: String
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

#[test]
fn test_sort_cities() {
    let mut cities = create_cities();
    sort_cities(&mut cities);
    assert_eq!("France", cities[0].country);
}

fn create_cities() -> Vec<City> {
    let mut cities = Vec::new();
    cities.push(City {
        name: "Palermo".to_string(),
        population: 650_000,
        country: "Italy".to_string()
    });
    cities.push(City {
        name: "Paris".to_string(),
        population: 4_500_000,
        country: "France".to_string()
    });
    cities.push(City {
        name: "Berlin".to_string(),
        population: 3_000_000,
        country: "Germany".to_string()
    });
    cities
}

fn start_sorting_thread(mut cities: Vec<City>) -> thread::JoinHandle<Vec<City>> {
    // you can add a variable name as parameter, it is just ignored
    let key_fn: fn(&City) -> i64 = |city: &City| -> i64 { -city.population };

    // move is really important for key_fn here, since cities is moved anyway (it's passed by value
    // to the closure)
    thread::spawn(move || {
        //let key_fn: fn(&City) -> i64 = |city: &City| -> i64 { -city.population };
        cities.sort_by_key(key_fn);
        cities
    })
}

#[test]
fn test_start_sorting_thread() {
    let cities = create_cities();
    let future = start_sorting_thread(cities);
    assert_eq!("France", future.join().expect("thread failed!")[0].country);
}

fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    };
    count
}

fn count_selected_cities2<F>(cities: &Vec<City>, test_fn: F) -> usize
    where F: Fn(&City) -> bool
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    };
    count
}

fn eat_frogs(city: &City) -> bool {
    city.country == "France"
}

#[test]
fn test_count_selected_cities() {
    let cities = create_cities();
    let count = count_selected_cities(&cities, eat_frogs);
    assert_eq!(count, 1);
}

#[test]
fn test_count_selected_cities2() {
    let cities = create_cities();
    let count = count_selected_cities(&cities, |city| city.country == "France");
    assert_eq!(count, 1);
}

#[test]
fn test_count_selected_cities3() {
    let cities = create_cities();
    let count = count_selected_cities2(&cities, eat_frogs);
    assert_eq!(count, 1);
}

#[test]
fn test_count_selected_cities4() {
    let cities = create_cities();
    let count = count_selected_cities2(&cities, |city| city.country == "France");
    assert_eq!(count, 1);
}

/*
It doesn't compile since a closure that captures something is not an fn

#[test]
fn test_count_selected_cities5() {
    let cities = create_cities();
    let country = "France";
    let count = count_selected_cities(&cities, |city| city.country == country);
    assert_eq!(count, 1);
}
 */

#[test]
fn test_count_selected_cities6() {
    let cities = create_cities();
    let country = "France";
    let count = count_selected_cities2(&cities, |city| city.country == country);
    assert_eq!(count, 1);
}