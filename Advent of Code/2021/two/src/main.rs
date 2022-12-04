use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";


fn main() {
    
    let moves = get_moves(INPUT);
    println!("num moves : {}",moves.len());

    let part_one = part_one(&moves);
    println!("Part one   : {}", part_one);

    let part_two = part_two(&moves);    
    println!("Part two   : {}", part_two);

}

fn part_one(moves: &Vec::<(char,i32)>) -> i32 {

    let mut pos = (0,0);

    for n in 0..moves.len() {
        if moves[n].0 == 'f' {
            pos.0 += moves[n].1
        } 
        if moves[n].0 == 'u' {
            pos.1 -= moves[n].1
        } 
        if moves[n].0 == 'd' {
            pos.1 += moves[n].1
        } 
    }

    pos.0 * pos.1
}

fn part_two(moves: &Vec::<(char,i32)>) -> i32 {

    let mut pos = (0,0);
    let mut aim = 0;

    for n in 0..moves.len() {
        if moves[n].0 == 'f' {
            pos.0 += moves[n].1;
            pos.1 += moves[n].1 * aim;
        } 
        if moves[n].0 == 'u' {
            aim -= moves[n].1;
        } 
        if moves[n].0 == 'd' {
            aim += moves[n].1;
        } 
    }

    pos.0 * pos.1
}

fn get_moves(filename: &str) -> Vec::<(char,i32)> {

    let mut moves = Vec::<(char,i32)>::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {

                let parts = l.split(" ").collect::<Vec<_>>();
                moves.push((
                    parts[0].chars().nth(0).unwrap(),
                    parts[1].parse::<i32>().unwrap()
                ));
            }
        }
    }

    return moves;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test_get_moves() {
    let test = "./test.txt";
    assert_eq!(get_moves(test).len(), 6);
}

#[test]
fn test_part_one() {
    let moves: Vec::<(char,i32)> = vec![('f',5),('d',5),('f',8),('u',3),('d',8),('f',2)];
    assert_eq!(part_one(&moves), 150);
}

#[test]
fn test_part_two() {
    let moves: Vec::<(char,i32)> = vec![('f',5),('d',5),('f',8),('u',3),('d',8),('f',2)];
    assert_eq!(part_two(&moves), 900);
}
