use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";
const TEST: &str = "./test.txt";


fn main() {
    
    let mut programs = get_data(INPUT);
    println!("positions: {} , {}",programs.len(), programs[0].len());

    part_one(&mut programs);
}

fn part_one ( programs : &mut Vec<Vec<u64>> ) -> u64 {

    let mut flashes = 0u64;
    let mut stack = Vec::new();

    let mut step = 0;
    loop {

        step += 1;

        let mut flashed = vec!(vec!(false;programs[0].len());programs.len());

        // incriment every location
        for x in 0..programs.len(){
            for y in 0..programs[0].len(){
                programs[x][y] += 1;
                if programs[x][y] > 9 {
                    stack.push((x,y));
                }
            }
        }

        while stack.len() > 0 {
            let z = stack.pop().unwrap();

            if programs[z.0][z.1] > 9 && !flashed[z.0][z.1] {
                flashes += 1;
                flashed[z.0][z.1] = true;
                inc_adj(programs,&mut stack,(z.0 as u64,z.1 as u64));
            }
        }


        for x in 0..programs.len(){
            for y in 0..programs[0].len(){
                if programs[x][y] > 9 {
                    programs[x][y] = 0;
                }
            }
        }

        if step == 100 {
            println!("\n\n");
            print_it(programs);

            println!("[{}] f = {}",step,flashes);
        }

        
        if all(flashed) {
            println!("\n\n");
            print_it(programs);

            println!("[{}] f = {}",step,flashes);

            break;
        }

    }

    return 0;

}

fn all(flashed: Vec<Vec<bool>>) -> bool{
    
    for a in flashed {
        for b in a {
            if b == false {
                return false;
            } 
        }
    }
    return true;
}

fn print_it(programs: &mut Vec<Vec<u64>>) {
    for x in 0..programs.len(){
        let mut s: String = String::new();
        for y in 0..programs[0].len(){
            let n : String = programs[x][y].to_string();
            if programs[x][y] == 0 {
                s = format!("{}  \x1B[1;96;127m{}\x1B[0m",s, n);
            } else {
                s = format!("{}  {}",s, n);
            }
        }
        println!("{}",s);
    }
}

fn inc_adj(programs: &mut Vec<Vec<u64>>, stack: &mut Vec<(usize,usize)>, loc : (u64, u64)) {

    let x = loc.0 as i64;
    let y = loc.1 as i64;

    let map: [[i64;2];8] = [
        [-1,-1],[ 0,-1],[ 1,-1],
        [-1, 0]        ,[ 1, 0],
        [-1, 1],[ 0, 1],[ 1, 1]
    ];

    for m in map {
        let m_x = x + m[0];
        let m_y = y + m[1];
        
        if m_x >= 0 && m_x < programs.len() as i64 && m_y >= 0 && m_y < programs[0].len() as i64 {
            programs[m_x as usize ][m_y as usize] += 1;
            if programs[m_x as usize ][m_y as usize] >= 9 && !stack.contains(&(m_x as usize,m_y as usize)) {
                stack.push((m_x as usize,m_y as usize));
            }
        } 
    }
}

fn get_data(filename: &str) -> Vec<Vec<u64>> {
 
    let mut data = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                data.push( l.trim().chars().map(|x| x.to_digit(10).unwrap() as u64).collect::<Vec<u64>>());
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