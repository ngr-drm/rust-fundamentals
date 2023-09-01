use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    // add values
    marks.insert("rust programming", 96);
    marks.insert("web development", 94);
    marks.insert("ux design", 75);
    marks.insert("professinal computing studies", 45);

    // find length of HashMap
    println!("how many subjects have you studied? {}", marks.len());

    // get a single value
    match marks.get("web development") {
        Some(mark) => println!("you got {} for web development!", mark),
        None => println!("you didn't study web development"),
    }

    // remove a value
    marks.remove("ux design");

    // loop through HashMap
    for (subject, mark) in &marks {
        println!("for {} you got {}%!", subject, mark);
    }

    // check for value
    println!(
        "did you study c++? {}",
        marks.contains_key("c++ programming")
    );
}
