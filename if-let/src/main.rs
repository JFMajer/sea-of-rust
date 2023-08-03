use std::collections::HashMap;
use std::io;

fn main() {
    // Hardcoded data in a HashMap
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("John".to_string(), "Happy".to_string());
    data.insert("Jane".to_string(), "Sad".to_string());
    data.insert("Michael".to_string(), "Excited".to_string());
    data.insert("Emily".to_string(), "Calm".to_string());
    data.insert("David".to_string(), "Anxious".to_string());


    println!("Please provide a name to check mood for: ");
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    let name = user_input.trim();
    println!("Provided name is {name}");

    // if let Some(mood) = data.get(name) {
    //     println!("{name}'s mood is {mood}");
    // } else {
    //     println!("Name {name} not found in database");
    // }

    match data.get(name) {
        Some(mood) => println!("{name}'s mood is {mood}"),
        None => println!("Name {name} not found in database"),
    }
}