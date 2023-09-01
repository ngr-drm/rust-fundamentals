fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();

    println!("collected (0..10) into: {:?}", collected_iterator);

    let mut my_vector = vec![1, 2, 3, 4];
    my_vector.push(44);
    my_vector.remove(1);

    for n in my_vector.iter() {
        println!("values: {}", n)
    }
}
