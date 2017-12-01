extern crate time;

fn main() {

    let mut year = get_current_year() - 1;
    println!("It is currently {}", year + 1);


    for _i in 0..20 {
        year = get_next_leap_year(year);
        println!("{} is a leap year!", year);
    }

}


fn get_current_year() -> i32 {
    let now = time::now().tm_year + 1900;
    return now;
}

fn get_next_leap_year(start:i32) -> i32 {

    let offset = 4 - (start % 4);

    let next = start + offset;

    if (next % 100 == 0) && !(next % 400 == 0) {
        return  get_next_leap_year(next);
    }

    return next;

}