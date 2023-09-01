enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let up: Direction = Direction::Up;
    let down: Direction = Direction::Down;
    let left: Direction = Direction::Left;
    let right: Direction = Direction::Right;

    match up {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    }
}
