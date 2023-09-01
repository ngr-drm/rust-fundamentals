mod code {
    fn code() {
        println!("i love to code...")
    }

    pub fn print_message() {
        code();
        println!("come with me...")
    }
}

fn main() {
    code::print_message();
}
