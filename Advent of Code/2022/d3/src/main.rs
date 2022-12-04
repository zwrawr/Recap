use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

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

    let backpacks = get_data(filename);

    println!("num bags   : {}", backpacks.len());

    let one = part_one(&backpacks);
    println!("part one   : {}", one);

    let two = part_two(&backpacks);
    println!("part two   : {}", two);

    println!("\n");
}


fn part_one ( bags : &Vec<(Vec<u32>,Vec<u32>,Vec<u32>)> ) -> u32 {

    let mut total = 0;
    for bag in bags {
        let a = bag.0.iter().cloned().collect::<HashSet<_>>();
        let b = bag.1.iter().cloned().collect::<HashSet<_>>();

        let common = a.intersection(&b).cloned().collect::<Vec<u32>>();

        // this allows for multiple common items, but i think it's just one each time.
        total += common.iter().sum::<u32>();
    }


    return total;
}

fn part_two ( bags : &Vec<(Vec<u32>,Vec<u32>,Vec<u32>)> ) -> u32 {

    let mut total = 0;
    for i in (0..bags.len()).step_by(3) {
        let a = bags[i].2.iter().cloned().collect::<HashSet<_>>();
        let b = bags[i+1].2.iter().cloned().collect::<HashSet<_>>();
        let c = bags[i+2].2.iter().cloned().collect::<HashSet<_>>();
        
        let common = c.intersection(&a.intersection(&b).cloned().collect::<HashSet<u32>>()).cloned().collect::<Vec<u32>>();

        // this allows for multiple common items, but i think it's just one each time.
        total += common.iter().sum::<u32>();
    }


    return total;
}



fn get_data(filename: &str) -> Vec<(Vec<u32>,Vec<u32>,Vec<u32>)> {

    let mut rounds = Vec::<(Vec<u32>,Vec<u32>,Vec<u32>)>::new();

    if let Ok(lines) = read_lines(filename) {

        for line in lines {
            if let Ok(l) = line {

                let mut x = l.chars().map(|c| if (c as u32) < 95 { return (c as u32) - 64 + 26} else { return (c as u32) - 96}).collect::<Vec<u32>>();

                let size = x.len()/2;
                let (a , b) = x.split_at_mut(size);

                rounds.push((a.to_vec(),b.to_vec(),x));
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