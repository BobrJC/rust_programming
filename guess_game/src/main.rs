use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Input your guess");
    let random_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Couldn't read string.");

    println!("Your guess: {guess}");

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Less"),
        Ordering::Equal =>  println!("Eq"),
        Ordering::Greater => println!("Greater"),
    }
}
