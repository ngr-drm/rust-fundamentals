struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let bg = Color {
        red: 255,
        green: 70,
        blue: 15,
    };

    print_color(&bg);
    print_color(&bg);
}

fn print_color(c: &Color) {
    println!("color - R:{} G:{} B:{}", c.red, c.green, c.blue)
}
