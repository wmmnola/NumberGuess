use std::io;
use rand::Rng;
extern crate rand;
pub const MAX: i32 = 214748364;

fn main() {
    let mut rng = rand::thread_rng();
    let random = (rng.gen::<i32>() % MAX).abs();
    println!("The random number is: {}",random );
    let steps = 0;
    computer_guess(MAX, 0, random, &steps);

}

fn dialog() -> i32 {
    println!("Guess a nuimber!");
    let mut guess = String::new();
    let mut guess_int = 10;
    io::stdin().read_line(&mut guess).expect("Fuck you you broke me");
//    let mut guess_int = guess.parse::<i32>();
    match guess.trim_right().parse::<i32>() {
        Ok(i) => guess_int = i,
        Err(_) => println!("Invalid number."),
    }
    guess_int
}

fn computer_guess(upperBound: i32, lowerBound :i32, random: i32, steps: &i32){
    let new_step = steps + 1;
    let sum:i64 = (upperBound + lowerBound) as i64;
    let guess:i32 = (sum/2) as i32;
    computer_guess_a_number(random, guess, upperBound, lowerBound,new_step);
}

fn computer_guess_a_number(number: i32 , guess: i32, oldUpperBound: i32, oldLowerBound: i32, steps:i32){
    let dif = &guess - &number;

    if dif > 0{
        computer_guess(guess, oldLowerBound, number,&steps);
    }
    else if dif < 0 {
        computer_guess(oldUpperBound, guess, number,&steps);
    }
    else {
        println!("You have beaten me, {} is infact {}. It only took {} steps", guess, number, steps);
    }

}


fn guessANumber(number: i32 , guess: i32 ) -> bool{
    if number == guess {
        return false;
    }
    else{
        let dif = guess - number;
        if dif > 0{
            println!("You guessed to high!")
        }
        else {
            println!("You guessed to low");
        }
        return true;
    }
}
