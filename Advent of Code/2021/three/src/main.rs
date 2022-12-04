use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";


fn main() {
    
    let moves = get_data(INPUT);
    println!("num moves : {}",moves.len());

    let part_one = part_one(&moves);
    println!("Part one   :  {}     [g {} , e {}]", part_one.0 * part_one.1, part_one.0 , part_one.1);

    //let part_two = part_two(&moves);    
   // println!("Part two   : {}", part_two);

}

fn part_one(moves: &Vec::<u16>) -> (u32,u32) {

    let mut ans = 0;
    for n in 0..12 {
        ans  = ans << 1;
        ans += most_common_bit(moves, n);
    }

    println!("{}",ans);
    (ans, ans ^ 0b111111111111)
}

fn most_common_bit(moves: &Vec::<u16>, pos:u32) -> u32 {
    let mut gamma : u32 = 0;

    for n in 0..moves.len() {
        println!("{},    {},    {}", moves[n], ((0b1 << (11 - pos)) & moves[n]), (((0b1 << (11 - pos)) & moves[n]) >> (11-pos) ));
        gamma += (((0b1 << (11- pos)) & moves[n]) >> (11-pos) ) as u32;
    }


    println!("gamma {}, num {}, ans {}", gamma, moves.len(), 2 * gamma / moves.len() as u32);
    return 2 * gamma / moves.len() as u32
}

fn part_two(moves: &Vec::<u16>, gamma:u16, epsilon:u16) -> (u32,u32) {

    let mask = 0b100000000000;

    let life = moves.into_iter().filter(
        |&n| n & mask == gamma & mask
    );

    let oxygen = moves.into_iter().filter(
        |&n| n & mask == gamma & mask
    );

//   .into_iter()
//   .filter(|n| n % 2 == 0)
//   .collect::<Vec<_>>();

  (1,1)

}

fn get_data(filename: &str) -> Vec::<u16> {

    let mut moves = Vec::<u16>::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                println!("{}",l);
                moves.push(isize::from_str_radix(&l, 2).unwrap().try_into().unwrap());
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
fn test_get_data() {
    let test = "./test.txt";
    assert_eq!(get_data(test).len(), 9);
}

#[test]
fn test_part_one() {
    let moves: Vec::<u16> = vec![0b111100000101,0b001110100010,0b101110110011,0b100000001101,0b001101010011,0b101111110000,0b011101110001,0b000000010111,0b000000010111];
    let x  = part_one(&moves);
    assert_eq!(x.0 * x.1, (0b001100010011 * 0b110011101100 )as u32);
}

// #[test]
// fn test_part_two() {
//     let moves: Vec::<(char,u32)> = vec![('f',5),('d',5),('f',8),('u',3),('d',8),('f',2)];
//     assert_eq!(part_two(&moves), 900);
// }
