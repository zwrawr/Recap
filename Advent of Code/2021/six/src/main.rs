use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";
const TEST: &str = "./test.txt";

#[derive(Eq)]
struct Fish {
    number: u32,
    clock : u32
}

impl PartialEq for Fish {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number &&
        self.clock == other.clock 
    }
}

fn main() {
    
    let fish = get_data(INPUT);
    println!("num fish : {}",fish.len());

    smart_process(&fish, 256);
}

fn smart_process(data : &Vec<u64>, limit: u64){
	
	let mut buf = [0u64; 10];

	for d in data {
		buf[*d as usize] += 1;
    }

	println!("0 : {} : {:?}", total(buf), buf);

	for i in 1..(limit+1){

		buf[7] += buf[0];
		buf[9] += buf[0];

		for x in 1..buf.len(){
			buf[x-1] = buf[x]
        }

		buf[buf.len()-1] = 0;

		println!("{} : {} : {:?}", i, total(buf), buf)
    }
}

fn total (arr : [u64; 10]) -> u64{
	let mut sum: u64 = 0;

	for a in arr {
		sum += a;
    }
	
	return sum;
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