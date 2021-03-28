use std::collections::HashMap;

fn main() {
    let mut team_scores = HashMap::new();
    team_scores.insert(String::from("Blue"), 10);
    team_scores.insert(String::from("Yellow"), 50);

    // getting a value out of a hash map
    let team_name = String::from("Blue");
    let blue_score = team_scores.get(&team_name);
    match blue_score {
        Some(score) => println!("Blue score: {}", score),
        _ => ()
    }

    // iterating over values
    for (key, value) in &team_scores {
        println!("Team {}: {}", key, value);
    }
}
