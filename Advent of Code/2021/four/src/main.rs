use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


const INPUT: &str = "./input.txt";
const TEST: &str = "./test.txt";

#[derive(Eq)]
struct Board {
    values: [[u8; 5]; 5],
    found : [[bool; 5]; 5],
    won : bool
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values &&
        self.found == other.found &&
        self.won == other.won
    }
}

fn main() {
    
    let (mut boards, nums) = get_data(INPUT);
    println!("num boards : {}",boards.len());
    println!("num nums   : {}",nums.len());

    //let part_one = part_one(&mut boards, &nums);
    //println!("Part one : {}\n\n\n\n\n", part_one);

    let part_two = part_two(&mut boards, &nums);    
    println!("Part two   : {}", part_two);

}

fn print_board(board :&Board) {
    for n in 0..board.values.len() {
        println!("{:?}\t\t{:?}",board.values[n],board.found[n]);
    }

}

fn part_two(boards: &mut Vec<Board>, nums: &Vec<u8>) -> u32 {

    for n in 0..nums.len() {
        println!("\n\ntesting : {}",nums[n]);
        
        for b in 0..boards.len() {
            process(&mut boards[b], &nums[n]);
        }

        let mut i = 0;
        let mut s = 0;
        while i != boards.len() {
            if boards[i].won {
                let b = boards.remove(i);
                s = score(&b, &nums[n]);
                println!("Removed board {} with score {}",i,s);

            } else {
                i += 1;
            }
        }

        if boards.len() == 0 {
            return s;
        }
    }

    return 0;
}

fn part_one(boards: &mut Vec<Board>, nums: &Vec<u8>) -> u32 {

    for n in nums {
        println!("\n\ntesting : {}",n);
        for b in &mut *boards {
            let found = process(b,n);

            if found { 

                return score(&b, n);
            }
        }
    }

    return 0;
}

fn score(board: &Board, num: &u8) -> u32 {

    let mut score = 0;

    for x in 0..board.values.len() {
        for y in 0..board.values[x].len() {
            if ! board.found[x][y] {
                score += board.values[x][y] as u32;
            }
        }
    }

    return score * (*num as u32);

}

fn process(board: &mut Board, num: &u8) -> bool {
    


    for x in 0..board.values.len() {
        for y in 0..board.values[x].len() {
            if board.values[x][y] == *num {
                board.found[x][y] = true;
            }
        }
    }

    print_board(board);

    return check(board);
}

fn check(board: &mut Board) -> bool {
    let mut vert = vec![1;5];
    let mut horz = vec![1;5];

    for x in 0..board.values.len() {
        for y in 0..board.values[x].len() {
            horz[x] *= if board.found[x][y] {1} else {0};
            vert[y] *= if board.found[x][y] {1} else {0};
        }
    }


    println!("horz : {:?}", horz);
    println!("vert : {:?}", vert);

    board.won = horz.into_iter().reduce(|x, y| x + y).unwrap() + vert.into_iter().reduce(|x,y| x + y).unwrap() > 0;

    println!("won  : {}", board.won);

    return board.won;

}

fn get_data(filename: &str) -> (Vec::<Board>, Vec::<u8>) {
 
    let mut boards = Vec::<Board>::new();
    boards.push(Board{values: [[0;5];5], found: [[false;5];5], won: false});
    let mut nums = Vec::<u8>::new();

    let mut i = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {

                let s = boards.len() - 1;
                let mut curr = &mut boards[s];

                //println!("line {}", l);

                if nums.len() == 0 { // the fist line is special
                    nums = l.split(',').map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>();
                    continue;
                }

                if l.len() < 2 {
                    continue;
                }

                curr.values[i] = l.split_whitespace().map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>().try_into()
                .unwrap_or_else(|v: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", curr.values[i].len(), v.len()));

                i+= 1;

                if i == 5{
                    i =0;
                    boards.push(Board{values: [[0;5];5], found: [[false;5];5], won: false});
                }
            }
        }
    }

    boards.pop();

    return (boards,nums);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// #[test]
// fn test_get_data() {
//     let test = "./test.txt";
//     assert_eq!(get_data(test).len(), 9);
// }

// #[test]
// fn test_part_one() {
//     let moves: Vec::<u16> = vec![0b111100000101,0b001110100010,0b101110110011,0b100000001101,0b001101010011,0b101111110000,0b011101110001,0b000000010111,0b000000010111];
//     let x  = part_one(&moves);
//     assert_eq!(x.0 * x.1, (0b001100010011 * 0b110011101100 )as u32);
// }
