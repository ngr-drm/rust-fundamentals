use std::num::ParseIntError;
fn main() {
    get();
}

async fn get() -> String {
    let body = reqwest::get("https://www.rust-lang.org").await.unwrap();

    println!("body = {:?}", body);
    return body;
}
