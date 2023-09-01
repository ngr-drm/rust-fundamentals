use std::io;

fn main() {
    let mut input = String::new();

    println!("hey mate! say something");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("success! you said: {}", input.to_uppercase());
        }
        Err(e) => println!("somenthing went wrong: {}", e),
    }
}
