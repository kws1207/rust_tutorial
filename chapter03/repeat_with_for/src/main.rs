fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for element in 1..6 {
        println!("the value is: {}", element);
    }

    for element in a.iter().rev() {
        println!("the value is: {}", element);
    }
}