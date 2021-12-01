use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";


fn main() {
    
    let depths = get_depths(INPUT);
    println!("num depths : {}",depths.len());

    let part_one = part_one(&depths);
    println!("Part one   : {}", part_one);

    let part_two = part_two(&depths);    
    println!("Part two   : {}", part_two);

}

fn part_one(depths: &Vec<i32>) -> i32 {

    let mut delta = vec![0; depths.len()];
    let mut changes = 0;

    for n in 1..depths.len() {
        delta[n] = depths[n] - depths[n-1];
        if delta[n] > 0 {
            changes += 1
        }
    }

    return changes;
}

fn part_two(depths: &Vec<i32>) -> i32 {
    let mut delta = vec![0; depths.len()];
    let mut sums = vec![0; depths.len()];
    let mut changes = 0;


    for n in 0..(depths.len() - 2) {

        for m in 0..3 {
            sums[n] += depths[n + m]
        }
    }

    for n in 1..(depths.len() - 2) {
        delta[n] = sums[n] - sums[n-1];

        if delta[n] > 0 {
            changes += 1
        }
    }

    return changes;
}

fn get_depths(filename: &str) -> Vec::<i32> {
    let mut depths = Vec::<i32>::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                depths.push(l.parse::<i32>().unwrap())
            }
        }
    }

    return depths;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test_get_depths() {
    let test = "./test.txt";
    assert_eq!(get_depths(test).len(), 10);
}

#[test]
fn test_part_one() {
    let depths: Vec<i32> = vec![199,200,208,210,200,207,240,269,260,263];
    assert_eq!(part_one(&depths), 7);
}

#[test]
fn test_part_two() {
    let depths: Vec<i32> = vec![199,200,208,210,200,207,240,269,260,263];
    assert_eq!(part_two(&depths), 5);
}
