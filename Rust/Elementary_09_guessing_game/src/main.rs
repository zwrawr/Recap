extern crate rand;

fn main() {

    let choice = pick_rand_number(100);

    println!("I've picked a number between {} and {}", 0, 100);

    let mut guesses:Vec<u32> = Vec::new();

    loop {
        let guess = get_user_guess();

        if choice == guess {
            break;
        } else {
            guesses.push(guess);
            println!("That's not right! it is {}.", match choice > guess {true => "HIGHER", false => "LOWER"});
        }
    }

    println!("Well done you got the correct answer, in {} attempts", guesses.len());

}

fn pick_rand_number(limit:u32) -> u32 {

    use rand::Rng;

    let mut r = rand::thread_rng();


    let pick = r.gen_range(0, limit);

    return pick;
}

fn get_user_guess() -> u32 {

    use std::io::{stdin,stdout,Write};

    let mut number:String = String::new();

    print!("Pick a number :: ");

    let _= stdout().flush();

    stdin().read_line(&mut number).expect("That is not a valid number");

    // Remove the newline
    if let Some('\n')=number.chars().next_back() {
        number.pop();
    }
    if let Some('\r')=number.chars().next_back() {
        number.pop();
    }

    match number.parse::<u32>() {
        Ok(i) => return i,
        Err(..) => {println!("this was not an integer: {}", number); std::process::abort()}
    };
}