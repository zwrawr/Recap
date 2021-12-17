use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";
const TEST: &str = "./test.txt";


fn main() {
    
    let inputs = get_data(TEST);
    println!("num inputs : {}", inputs.len());

    let part_one = part_one(&inputs);
    println!("part one : {}", part_one);

    smart_process(&inputs);
}

fn part_one(data : & Vec<(Vec<Vec<char>>,Vec<Vec<char>>)>) -> u64 {
    
    let mut count : u64 = 0;
    
    for d in data {
        let ans = &d.1;

        for x in ans {
            
            let l = x.len();

            if l==2 || l==3 || l==4 || l==7 {
                count += 1;
            }
        }
    
    }

    return count;
}

fn smart_process(data : &Vec<(Vec<Vec<char>>,Vec<Vec<char>>)>) {
	

    for d in data {
        process(&d);
    }
}

fn process(input : &(Vec<Vec<char>>,Vec<Vec<char>>)) {

    //    1111
    //   2    3
    //   2    3
    //    4444
    //   5    6
    //   5    6
    //    7777  8

    let mut map : Vec<Vec<char>> = vec!(vec!('a','b','c','d','e','f','g'); 7);

    println!("{:?}",map);

    
    let one = input.0.iter().find(|x| x.len() == 2).unwrap();
    map[2].retain(|&x| x == one[0] || x == one[1]);
    map[5].retain(|&x| x == one[0] || x == one[1]);

    for i in [0,1,3,4,6] {
        map[i].retain(|&x| x != one[0] && x != one[1]);
    }
    println!("{:?}",map);

    let three = input.0.iter().find(|x| x.len() == 3).unwrap();
    map[0].retain(|&x| x == three[0] || x == three[1] || x == three[2]);
    map[0].retain(|&x| x != one[0] && x != one[1]);
    let top = map[0][0].clone();
    for i in [1,2,3,4,5,6] {
        map[i].retain(|&x| x != top);
    }
    println!("{:?}",map);

    let four = input.0.iter().find(|x| x.len() == 4).unwrap();
    map[1].retain(|&x| x == four[0] || x == four[1] || x == four[2] || x == four[3]);
    map[3].retain(|&x| x == four[0] || x == four[1] || x == four[2] || x == four[3]);
    map[1].retain(|&x| x != one[0] && x != one[1]);
    map[3].retain(|&x| x != one[0] && x != one[1]);

    let poss = map[1].clone();
    for i in [0,4,6] {
        map[i].retain(|&x| x != poss[0] && x != poss[1]);
    }

    println!("{:?}",map);

}

fn get_data(filename: &str) -> Vec<(Vec<Vec<char>>,Vec<Vec<char>>)> {
 

    let mut inputs : Vec<(Vec<Vec<char>>,Vec<Vec<char>>)> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            //println!("{:?}",line);
            if let Ok(l) = line {
                //println!("{}",l);

                let mut parts = l.split('|');

                let test = parts.next().unwrap().split(' ').collect::<Vec<&str>>().iter().map(|x| x.trim().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
                let ans = parts.next().unwrap().split(' ').collect::<Vec<&str>>().iter().map(|x| x.trim().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
                
                inputs.push((test,ans));
            }
        }
    }
    return inputs;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
