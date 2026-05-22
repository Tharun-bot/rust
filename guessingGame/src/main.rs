//1. First generate a randome number and store it in a immutable variable
//2. Input a number from the user
//Go into a infinite loop until the input number matches the generated random number


use rand::Rng;
use std::io;

fn main(){
    let fruits:[&str;5] = ["apple", "orange", "papaya", "banana", "strawberry"];
    let mut rng  = rand::thread_rng();
    let generated_num = rng.gen_range(0..fruits.len());
    let mut count = 1;
    println!("Guessed Fruit : {}", fruits[generated_num]);
    //take user input
    loop {
        let mut input = String::new();
        println!("Enter your Guess : ");

        io::stdin()
            .read_line(&mut input)
            .expect("Error in reading input");

        if fruits[generated_num] == input.trim() {
            break;
        }

        count+=1;
    }

    println!("You took {} attempts to get it correct as {}", count, fruits[generated_num]);

}