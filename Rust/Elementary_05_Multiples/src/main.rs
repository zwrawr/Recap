fn main() {

    let number:u32 = get_users_number();

    let mut x:u32 = 0;
    for i in 1..(number+1) {
        if i%3 == 0 || i%5 ==0 {
            x += i;
        }
    }

    print!("The sum of all the numbers up to {} that are multiples of three or five is {}",number,x);
}

fn get_users_number() -> u32 {

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
