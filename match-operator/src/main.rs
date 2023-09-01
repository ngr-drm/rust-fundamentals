fn main() {
    let number = 31;
    let name = "Domenic";

    match number {
        1 => println!("it is one!"),
        2..=30 => println!("it is greater than one!"),
        _ => println!("it doesn't match..."),
    }

    match name {
        "Chris" => println!("nice name, mate!"),
        "Domenic" => println!("yeah good on you!"),
        _ => println!("it doesn't match..."),
    }
}
