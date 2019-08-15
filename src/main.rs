use std::io;
use std::cmp::Ordering;
use rand::Rng;

const READ_LINE_ERROR : &str = "Failed to read line!";

fn main() {

    let secret: u32 = generate_secret();
    guess_number(&secret);
}

fn generate_secret() -> u32 {
    println!("Select top number - the higher the harder the game!");
    let mut top = String::new();
    io::stdin().read_line(&mut top).expect(READ_LINE_ERROR);
    let top: u32 = match top.trim().parse() {
        Ok(num) => num,
        Err(_) => generate_secret(),
    };

    let two : u32 = 2;

    match top.cmp(&two) {
        Ordering::Less => {
            println!("Too small!");
            generate_secret();
        },
        Ordering::Equal => {
            println!("Too small!");
            generate_secret();
        },
        Ordering::Greater => println!("Generating."),
    }
    return rand::thread_rng().gen_range(1, top);
}

fn guess_number(secret : &u32) {
    loop {
        println!("Now guess the number!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(READ_LINE_ERROR);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}