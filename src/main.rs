use rand::Rng; // random number dependency
use std::io::stdin; // standard input library 

fn main() {
    'outer_loop: loop {
        // let _number: u32 = 10;
        let _number: i32 = rand::thread_rng().gen_range(1..=15);

        println!("Pick a number between 1 to 15>>>");

        loop {
            let mut line = String::new();

            let _input = stdin().read_line(&mut line);

            let _guess: Option<i32> = _input.ok().map_or(None, |_| line.trim().parse().ok());

            match _guess {
                None => println!("Enter a number..."),
                Some(n) if n == _number => {
                    println!("Bravo! You guessed it right.");
                    break 'outer_loop;
                }
                Some(n) if n < _number => println!("Too low"),
                Some(n) if n > _number => println!("Too high"),
                Some(_) => println!("Error!"),
            }
        }
    }
}
