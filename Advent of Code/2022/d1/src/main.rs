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
    println!("part two   : {} \t ({} , {} , {})", two.0,two.1,two.2,two.3);

    println!("\n");
}


fn part_one ( people : &Vec<Vec<u64>> ) -> u64 {

    let mut best: u64 = 0;
    
    for p in people {

            let t = p.iter().fold(0, |acc,x| acc + x);

            if t > best {
                best = t;
            }
    }

    return best;

}


fn part_two( people : &Vec<Vec<u64>> ) -> (u64,u64,u64,u64) {

    let mut totals = Vec::<u64>::new();
    
    for p in people {

            // not a fast way to do it, but it's easy
            let t = p.iter().fold(0, |acc,x| acc + x);
            totals.push(t);
            totals.sort_by(|a , b| b.cmp(a));
            if totals.len() > 3 {
                totals = totals[0..3].to_vec();
            }
    }

    return (totals[0]+totals[1]+totals[2],totals[0],totals[1],totals[2]);

}


fn get_data(filename: &str) -> Vec<Vec<u64>> {

    let mut people = Vec::<Vec::<u64>>::new();
    let mut cals = Vec::<u64>::new();

    if let Ok(lines) = read_lines(filename) {

        for line in lines {
            if let Ok(l) = line {

                let cal = l.parse::<u64>();

                if !cal.is_ok() {
                    people.push(cals);
                    cals  = Vec::<u64>::new(); 
                } else {
                    cals.push(cal.unwrap());
                }
            }
        }
    }

    people.push(cals);

    return people;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}