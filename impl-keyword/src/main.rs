struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
fn main() {
    let my_rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("rectangle is a square: {}", my_rect.is_square());
    my_rect.print_description();
}
