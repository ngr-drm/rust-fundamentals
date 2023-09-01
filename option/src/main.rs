fn main() {
    println!(
        "occupation is: {}",
        match get_occupation("Maria") {
            Some(o) => o,
            None => "no occupation found!",
        }
    );
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Domenic" => Some("software Developer"),
        "Michael" => Some("dentist"),
        _ => None,
    }
}
