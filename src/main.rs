use std::io;
use rand::Rng;

fn main() {

    //call the guessing game
    guess_game();
    }

    fn guess_game(){
        //Print welcome message
        println!("Welcome to the guessing game!");

        //Generate the random number
        let secret = rand::thread_rng().gen_range(1, 101);
        println!("Secret number generated");

        loop {
            //Accept the user input
            println!("Enter your guess: ");
            let mut guess_number = String::new();

            io::stdin().read_line(&mut guess_number)
                .expect("default");

            //Convert the guess to a number
            let guess_number:u32 = guess_number.trim().parse().expect("Failed to convert");

            if guess_number < secret {
                println!("Your guess was lower");
            }
            else if guess_number > secret{
                println!("Your guess was higher");
            }
            else{
                println!("Bingo!");
                break;
            }
    }
}
