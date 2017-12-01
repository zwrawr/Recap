fn main() {

    let number:u32 = get_users_number();

    let sum:bool = get_users_is_sum();

    // Figure out which function we want
    let func:fn(u32,u32)->u32 = match sum { true => add, false => multi};
    let op = if sum {"sum"} else {"product"};

    let mut x:u32 = match sum {true => 0, false => 1};
    for i in 1..(number+1) {
        x=func(x,i);
    }

    print!("The {} of all the numbers up to {} is {}",op,number,x);
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

fn get_users_is_sum() -> bool {

    use std::io::{stdin,stdout,Write};

    let mut sum:String = String::new();

    print!("Do you want to sum(S) or product(P) these numbers :: ");

    let _= stdout().flush();

    stdin().read_line(&mut sum).expect("That is not a valid input");

    // Remove the newline
    if let Some('\n')=sum.chars().next_back() {
        sum.pop();
    }
    if let Some('\r')=sum.chars().next_back() {
        sum.pop();
    }

    if sum.trim() == "S" {
        return true;
    }
    else if sum.trim() == "P" {
        return false;
    }
    else{
        println!("this was not an integer: {}", sum);
        std::process::abort();
    }
}

fn add (x:u32,y:u32) -> u32{
    x+y
}

fn multi (x:u32,y:u32) -> u32{
    x*y
}