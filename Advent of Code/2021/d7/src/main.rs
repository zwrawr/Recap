use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";
const TEST: &str = "./test.txt";


fn main() {
    
    let crabs = get_data(INPUT);
    println!("num crabs : {}",crabs.len());

    part_one(&crabs);
    part_two(&crabs);
}


fn part_one ( crabs : &Vec<u64> ) {

    let mut best = (0u64,  999999999u64);
    
    let max = crabs.iter().fold(0u64,|max,b| if *b > max {*b} else {max});
    let min = crabs.iter().fold(999999u64,|min,b| if *b < min {*b} else {min});

    println!("max : {} \t min :{}", max, min);
    

    for x in min..(max+1) {

        let fuel = crabs.iter().fold(0u64,|a,b| a + ((x as i64)-(*b as i64)).abs() as u64);

        if fuel < best.1 {
            best = (x,fuel);
            println!("New Best : {} : {}", x, fuel);
        }
    }

}

fn part_two ( crabs : &Vec<u64> ) {

    let mut best = (0u64,  999999999u64);
    
    let max = crabs.iter().fold(0u64,|max,b| if *b > max {*b} else {max});
    let min = crabs.iter().fold(999999u64,|min,b| if *b < min {*b} else {min});

    println!("max : {} \t min :{}", max, min);
    

    for x in min..(max+1) {

        let fuel = crabs.iter().fold(0u64,|a,b| a + ((x as i64)-(*b as i64)).abs() as u64* (((x as i64)-(*b as i64)).abs() as u64 + 1)/2);

        if fuel < best.1 {
            best = (x,fuel);
            println!("New Best : {} : {}", x, fuel);
        }
    }

}

fn get_data(filename: &str) -> Vec<u64> {
 

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                println!("{}",l);
                return l.split(',').collect::<Vec<&str>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            }
        }
    }
    return Vec::<u64>::new();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}