fn main() {
    for i in 1..11 {
        println!("the number is {}", i);
    }

    let numbers = 30..51;

    for i in numbers {
        println!("the numbers is {}", i)
    }

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for a in animals.iter() {
        println!("the animal name is {}", a)
    }

    for (index, a) in animals.iter().enumerate() {
        println!("the index is {0} and the animal name is {1}", index, a)
    }
}
