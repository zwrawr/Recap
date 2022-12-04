use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";
const TEST: &str = "./test.txt";


fn main() {
    
    let programs = get_data(INPUT);
    println!("positions: {} , {}",programs.len(), programs[0].len());

    part_one(&programs);
}

fn part_one ( programs : &Vec<Vec<char>> ) -> (u64,u64) {

    let mut score_one = 0u64;
    let mut score_two = Vec::<u64>::new();

    for program in programs {
        let mut stack = Vec::new();

        let mut score = 0u64;
        let mut valid = true; 

        for i in 0..program.len() {
            let x = program[i];

            if i == 0 {
                stack.push(x);
            } else {
                if x == '(' || x == '[' || x == '{' || x == '<' {
                    stack.push(x);
                } else if x == ')' {
                    let y = stack.pop().unwrap();
                    if y != '(' {
                        score_one += 3;
                        //println!("{:?} \t : i:{}\tx:{}\ty:{}",program,i,x,y)
                        valid = false;
                    }
                } else if x == ']' {
                    let y = stack.pop().unwrap();
                    if y != '[' {
                        score_one += 57;
                        //println!("{:?} \t : i:{}\tx:{}\ty:{}",program,i,x,y)
                        valid = false;
                    }
                } else if x == '}' {
                    let y = stack.pop().unwrap();
                    if y != '{' {
                        score_one += 1197;
                        //println!("{:?} \t : i:{}\tx:{}\ty:{}",program,i,x,y)
                        valid = false;
                    }
                } else if x == '>' {
                    let y = stack.pop().unwrap();
                    if y != '<' {
                        score_one += 25137;
                        //println!("{:?} \t : i:{}\tx:{}\ty:{}",program,i,x,y)
                        valid = false;
                    }
                }
            }
        }
        if stack.len() != 0 && valid == true{

            while stack.len() > 0 {
                let z = stack.pop().unwrap();

                score *= 5;
                
                if z == '(' {
                    score += 1;
                }
                if z == '[' {
                    score += 2;
                }
                if z == '{' {
                    score += 3;
                }
                if z == '<' {
                    score += 4;
                }
            }

            score_two.push(score);
        }


    }
    score_two.sort();
    println!("{:?}",score_two);

    let s =score_two[score_two.len()/2];
    println!("Part one : {}", score_one);
    println!("Part two : {}", s);
    
    return (score_one, s);
}

fn get_data(filename: &str) -> Vec<Vec<char>> {
 
    let mut data = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {

                data.push( l.trim().chars().collect::<Vec<char>>());

            }
        }
    }

    return data;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}