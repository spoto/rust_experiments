use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn new_probation(name: &str) -> Self {
        Self::new(name, VisitorAction::Probation, 0)
    }

    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welsome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new("Steve", VisitorAction::AcceptWithNote {
            note: String::from("Lactose-free milk is in the fridge")
        }, 15),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];
    while let Some(name) = ask_for_name() {
        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet(),
            None => {
                println!("{} is not on the visitor list.", name);
                visitor_list.push(Visitor::new_probation(&name));
            },
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}

fn ask_for_name() -> Option<String> {
    println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    let name = your_name
        .trim()
        .to_lowercase();

    if name.is_empty() {
        None
    }
    else {
        Some(name)
    }
}
