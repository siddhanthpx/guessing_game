use rand::{Rng};
use std::{io, cmp::Ordering};


fn main(){
    println!("guess the random number");

    let random_number = rand::thread_rng().gen_range(1..100);
    let mut counter = 0;
    const MAX_TRIES: u32 = 5;

    loop {
        counter += 1;
        println!("please input your guess: ");
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read input");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&random_number){
            Ordering::Less => println!("guess higher"),
            Ordering::Greater => println!("guess lower"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
            
        }
        
        println!("remaining guesses: {}", MAX_TRIES - counter);
        if counter == MAX_TRIES {
            println!("no more guesses remaining");
            break;
        }
        
    }


}
