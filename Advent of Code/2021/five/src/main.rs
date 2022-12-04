use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./input.txt";
const TEST: &str = "./test.txt";


fn main() {
    
    let (lines, x, y) = get_data(INPUT);

    println!("num lines : {}",lines.len());
    println!("feild is  : x[0,{}] y[0,{}]",x,y);

    let part_one = part_one(&lines, x, y);
    println!("Part one : {}\n\n\n\n\n", part_one);

    let part_two = part_two(&lines, x, y);    
    println!("Part two   : {}", part_two);

}

fn print_map(map: &Vec<Vec<u32>>){
    for m in map{
        println!("{:?}",m)
    }
}

fn part_one(lines: &Vec<Vec<u32>>, x:u32, y:u32) -> u32{

    let mut map = vec![vec![0; (x+1) as usize]; (y+1) as usize];

    for l in lines {

        println!("{:?}",l);
        if l[0] == l[2] {
            // The line is vertical

            let y1 = if l[1] > l[3] {l[3]} else {l[1]};
            let y2 = if l[1] <= l[3] {l[3]} else {l[1]};

            for dy in y1..(y2+1) {
                map[l[0] as usize][dy as usize]+= 1;
            }
            
            //print_map(&map);
            //println!("\n");
        }
        else if l[1] == l[3] {
            // The line is horozontal


            let x1 = if l[0] > l[2] {l[2]} else {l[0]};
            let x2 = if l[0] <= l[2] {l[2]} else {l[0]};

            for dx in x1..(x2+1) {
                map[dx as usize][l[1] as usize]+= 1;
            }
            //print_map(&map);
            //println!("\n");
        }
        else {
            // Line is diagonal

            // let dx : i32 = if l[0] < l[2] {1} else {-1};
            // let dy : i32 = if l[1] < l[3] {1} else {-1};

            // let mut x = l[0];
            // let mut y = l[1];

            // while x != l[2] {

            //     map[x as usize][y as usize] += 1;

            //     x=(x as i32 + dx) as u32;
            //     y=(y as i32 + dy) as u32;
            // }
            
        }
    }

    //print_map(&map);

    let mut more_than_one = 0;

    for m in map {
        for n in m {
            if n > 1 {
                more_than_one += 1;
            }
        }
    }


    return more_than_one;

    // This should of worked but there i a type issue i cannot figuire out

    // return map.iter().fold(0u32,
    //     |acc:u32 , x:Vec<u32>| 
    //     acc + x.iter().fold(0u32, 
    //         |acc:u32 , y:&u32| 
    //         if *y > 1 {return acc + 1} else{return acc}
    //     )
    // );
}

fn part_two(lines: &Vec<Vec<u32>>, x:u32, y:u32) -> u32{

    let mut map = vec![vec![0; (x+1) as usize]; (y+1) as usize];

    for l in lines {

        // Line is diagonal

        let dx : i32 = if l[0] < l[2] {1} else if l[0] > l[2] {-1} else {0};
        let dy : i32 = if l[1] < l[3] {1} else if l[1] > l[3] {-1} else {0};

        let mut x = l[0];
        let mut y = l[1];

        while !(x as i32 == (l[2] as i32 + dx) && y as i32 == (l[3] as i32 + dy)) {

            map[x as usize][y as usize] += 1;

            x=(x as i32 + dx) as u32;
            y=(y as i32 + dy) as u32;

        }

        //print_map(&map);
        //println!("{:?}",l);
        //println!("\n");
        
    }

    //print_map(&map);

    let mut more_than_one = 0;

    for m in map {
        for n in m {
            if n > 1 {
                more_than_one += 1;
            }
        }
    }


    return more_than_one;
}

fn get_data(filename: &str) -> (Vec::<Vec<u32>>, u32,u32) {
 
    let mut vents = Vec::<Vec<u32>>::new();
    let mut max :(u32,u32) = (0,0);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {

                let pos: Vec<_> = l.split(&['>', ',', '-', ' '][..]).filter(|x| x.len() > 0).collect();
                let nums : Vec<u32> = pos.iter().map(|x| x.parse::<u32>().unwrap()).collect();

                max.0 = if nums[0] > max.0 { nums[0] } else  if nums[2] > max.0 {nums[2]} else {max.0};  
                max.1 = if nums[1] > max.1 { nums[1] } else  if nums[3] > max.1 {nums[3]} else {max.1};

                vents.push(nums);

            }
        }
    }

    return (vents,max.0,max.1);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}