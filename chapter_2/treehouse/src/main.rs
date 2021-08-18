use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let mut allow_access = false;
    let visitor_list = ["bert", "steve", "fred"];
    for visitor in &visitor_list {
        if visitor == &name {
            allow_access = true;
        }
    }

    if allow_access {
        println!("Weclome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list");
    }
}
