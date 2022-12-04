use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";
const TEST: &str = "./test.txt";


fn main() {
    
    let floor = get_data(INPUT);
    println!("positions: {}",floor.len() * floor[0].len());

    let part_one = part_one(&floor);

    let part_two = part_two(&floor, &part_one);
}

fn part_one ( floor : &Vec<Vec<u64>> ) -> Vec<(usize,usize)>{

    let mut lowpoints = Vec::new();

    let x_max = floor.len();
    let y_max = floor[0].len();

    for x in 0..x_max{
        for y in 0..y_max{
            let mut lower = true;

            if x > 0 {
                if floor[x - 1][y] <= floor[x][y] {
                    lower = false;
                }
            }

            if  y < (y_max - 1) {
                if floor[x][y + 1] <= floor[x][y] {
                    lower = false;
                }
            }

            if y > 0 {
                if floor[x][y - 1] <= floor[x][y] {
                    lower = false;
                }
            }

            if x < (x_max - 1) {
                if floor[x + 1][y] <= floor[x][y] {
                    lower = false;
                }
            }


            if lower {
                lowpoints.push((x,y));
            }
        }    
    }
   
    println!("Part One : {:?}", lowpoints.iter().map(|x| floor[x.0][x.1] + 1).fold(0u64, |a,b| a + b));

    return lowpoints;
}

fn part_two ( floor : &Vec<Vec<u64>>, lowpoints: &Vec<(usize,usize)> ) -> usize{

    let mut basins = Vec::new();

    for i in 0..(lowpoints.len()) {

        let mut stack : Vec<(usize,usize)> = Vec::new();
        let mut basin : Vec<(usize,usize)> = Vec::new();

        stack.push((lowpoints[i].0,lowpoints[i].1));

        while stack.len() > 0 {
            expand(&mut stack, &mut basin, &floor);
        }

        println!("basin : {:?}", basin);
        basins.push(basin);
    }

    basins.sort_by(|a,b| a.len().cmp(&b.len()));
    println!("{:?}",basins);
    let ans = basins.pop().unwrap().len() * basins.pop().unwrap().len() * basins.pop().unwrap().len();
    println!("Part Two {}", ans);

    return ans;
}

fn expand (stack: &mut Vec<(usize,usize)>, basin: &mut Vec<(usize,usize)>, map: &Vec<Vec<u64>>) {

    let curr = stack.pop().unwrap();

    if map[curr.0][curr.1] == 9 {
        return;
    }


    basin.push(curr);

    if curr.0 > 0 {
        let left = (curr.0 - 1, curr.1);
        if !basin.contains(&left) && !stack.contains(&left) {
                stack.push(left);
        }
    }

    if curr.0 < (map.len() - 1) {
        let right = (curr.0 + 1, curr.1);
        if  !basin.contains(&right) && !stack.contains(&right) {
                stack.push(right);
        }
    }

    if curr.1 > 0 {
        let top = (curr.0, curr.1 - 1);
        if !basin.contains(&top) && !stack.contains(&top) {
                stack.push(top);
        }
    }

    if curr.1 < (map[0].len() - 1) {
        let bottom = (curr.0, curr.1 + 1);
        if !basin.contains(&bottom) && !stack.contains(&bottom) {
                stack.push(bottom);
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