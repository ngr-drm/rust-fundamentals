fn main() {
    let mut my_string = String::from("how's it going? My name is Ana");

    // length
    println!("length: {}", my_string.len());

    // is empty?
    println!("string is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!(
        "does the string contain 'Ana'? {}",
        my_string.contains("Ana")
    );
    my_string.push_str("welcome!");

    println!("{}", my_string);
}
