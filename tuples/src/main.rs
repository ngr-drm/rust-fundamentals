fn main() {
    let _tup1 = (20, "Rust", 3.4, false);
    let tup2 = (20, "Rust", 3.4, false, (1, 4, 7));
    let (a, _b, _c, _d) = _tup1;

    println!("{}", (tup2.4).0);
    println!("a is {}", a);
}
