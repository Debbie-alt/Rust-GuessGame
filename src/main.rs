use std::io;
use rand::Rng;

fn main(){
    println("Gues   the secret number!!")

    let secret_number = rand::thread_rng.gen_range(1..=100)
    println!("the secret number is: {secret_number}" )
    println!("please input your guess")
    let mut  guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    
     println!("You guessed {}: ", guess)
}