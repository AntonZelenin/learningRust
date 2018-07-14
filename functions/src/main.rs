fn main() {
    let arr = [1, 2, 3, 4, 5];

    for elem in arr.iter() {
        println!("{}", elem);
    }
    println!();

    for num in (1..4).rev() {
        println!("{}!", num);
    }

    println!("LIFTOFF!!!");

    println!("{}", get_name());
}

fn get_name() -> String {
    String::from("Anton")
}
