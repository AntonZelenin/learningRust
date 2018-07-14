extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number1");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Type your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).
            expect("Error reading line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

    let char = "z";
    let char = 'z';
    let char = '😻';

    let tuple: (i32, char, bool) = (500, 'z', true);
    let (x, y, z) = tuple;
    let z = tuple.1;

    let arr = [1, 2, 3, 4];
}

fn get_name() {
    println!("Anton");
}