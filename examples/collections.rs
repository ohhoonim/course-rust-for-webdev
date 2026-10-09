
fn main() {
    // vec_exmple();
    // string_example();
    hashmap_exmple();
}

fn vec_exmple() {
    let mut scores: Vec<i32> = Vec::new();
    scores.push(10);
    scores.push(20);
    scores.push(30);
    scores.push(40);

    for score in &scores {
        println!("{}", score);
    }

    println!("fist index is {}", scores[0]);
}

fn string_example() {
    let owned = String::from("hello");
    let borrowed: &str = "world";

    println!("{} {}", owned, borrowed);
}

use std::collections::HashMap;

fn hashmap_exmple() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("math"), 90);
    scores.insert(String::from("korean"), 98);

    if let Some(score) = scores.get("korean") {
        println!("{}", score);
    }

    for (subject, score) in &scores {
        println!("subject: {}, score: {}", subject, score);
    }
    
}