struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("my name is {} an I am {}.", self.name, self.age);
    }
}

fn main() {
    let lebron = Person {
        name: String::from("Lebron"),
        age: 21,
    };
    println!("{}", lebron.to_string());
}
