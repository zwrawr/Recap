use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";
const TEST: &str = "./test.txt";

fn main() {
    // run on both the test data and the input
    run(TEST);
    run(INPUT);
}

fn run (filename:  &str) {

    println!("\nfilename    : {}", filename);

    let backpacks = get_data(filename);

    println!("num pairs   : {}", backpacks.len());

    let one = part_one(&backpacks);
    println!("part one    : {}", one);

    let two = part_two(&backpacks);
    println!("part two    : {}\n", two);

}

fn part_one ( rooms : &Vec<(u128,u128)> ) -> u32 {

    let mut total = 0;
    for room in rooms {
        let c = room.0 &room.1;
        if c == room.0 || c == room.1 {total+=1}
    }

    return total;
}

fn part_two ( rooms : &Vec<(u128,u128)> ) -> u32 {

    let mut total = 0;

    for room in rooms {
        let c = room.0 & room.1;
        if c != 0 {total+=1}
    }

    return total;
}

fn get_data(filename: &str) -> Vec<(u128,u128)> {

    let mut rooms = Vec::<(u128,u128)>::new();

    if let Ok(lines) = read_lines(filename) {

        for line in lines {
            if let Ok(l) = line {

                let x = l.split(&[',','-']).map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();
                //println!("{:?}", x);

                let mut a : u128 = 0;
                for i in x[0]..(x[1]+1) { a = a | (1 << (i-1)) }
                //println!("{:#0100b}",a);

                let mut b : u128 = 0;
                for i in x[2]..(x[3]+1) { b = b | (1 << (i-1)) }
                //println!("{:#0100b}",b);

                rooms.push((a,b));
            }
        }
    }

    return rooms;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
