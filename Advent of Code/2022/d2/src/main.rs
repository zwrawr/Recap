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

    println!("");
    println!("filename   : {}", filename);

    let people = get_data(filename);

    println!("num people : {}", people.len());

    let one = part_one(&people);
    println!("part one   : {}", one);

    let two = part_two(&people);
    println!("part two   : {}", two);

    println!("\n");
}


fn part_one ( rounds : &Vec<(char,char)> ) -> u64 {

    let mut points: u64 = 0;
    
    for r in rounds {
        points += points_for_choice(r.1);
        points += 6 - calc_round_winner_one(r.0,r.1);
    }

    return points;

}

// this could be way faster if we dealt witht the choice as numbers not chars, but it's so fast already it's not worth it.
fn calc_round_winner_one (p1:char, p2:char) -> u64 {

    return match (p1,p2) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        _ => panic!("UNKNOWN COMBO [{} v {}]", p1, p2),
    };
}

fn points_for_choice (c : char) -> u64 {

    return match (c)  {
        'X' | 'A' | 'R' => 1,
        'Y' | 'B' | 'P' => 2,
        'Z' | 'C' | 'S' => 3,
        _ => panic!("UNKNOWN Choice [{}]", c),
    }
}

fn calc_round_winner_two (p1:char, p2:char) -> u64 {

    if p2 == 'X' {return 0};
    if p2 == 'Y' {return 3};
    if p2 == 'Z' {return 6};

    panic!("UNKNOWN Result :\t[{}]", p2);
}

fn calc_choice_for_result (p1:char, r:char) -> char {

    return match (p1,r) {
        ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 'S',
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 'R',
        ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 'P',
        _ => panic!("UNKNOWN COMBO [{} v {}]", p1, r),
    };

}

fn part_two ( rounds : &Vec<(char,char)> ) -> u64 {

    let mut points: u64 = 0;
    
    for r in rounds {
        points += points_for_choice(calc_choice_for_result(r.0, r.1));
        points += calc_round_winner_two(r.0, r.1);
    }

    return points;

}



fn get_data(filename: &str) -> Vec<(char,char)> {

    let mut rounds = Vec::<(char,char)>::new();

    if let Ok(lines) = read_lines(filename) {

        for line in lines {
            if let Ok(l) = line {

                let x : Vec<char> = l.split(" ").map(|s| s.chars().collect::<Vec<char>>()[0]).collect::<Vec<char>>();
                rounds.push((x[0],x[1]))
               
            }
        }
    }


    return rounds;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}