fn main() {
    println!("Hello,");

    let name = get_users_name();

    println!("Hello {}, it's nice to meet you!", name);

    return;
}

fn get_users_name() -> String {
    use std::io::{stdin,stdout,Write};

    let mut name: String = String::new();

    print!("What is your Name? :: ");

    let _= stdout().flush();

    stdin().read_line(&mut name).expect("Did not enter a correct string");
    if let Some('\n')=name.chars().next_back() {
        name.pop();
    }
    if let Some('\r')=name.chars().next_back() {
        name.pop();
    }

    return name;
}
