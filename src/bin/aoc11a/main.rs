use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<u64> {
    let file = File::open("inputs/11.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);
    let line = br.lines().next().unwrap().unwrap();
    line.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn simulate_step(stones: &Vec<u64>) -> Vec<u64> {
    let mut res = Vec::new();
    for &stone in stones {
        if stone == 0 {
            res.push(1);
        } else {
            let str = stone.to_string();
            if str.len() % 2 == 0 {
                res.push(str[0..str.len() / 2].parse().unwrap());
                res.push(str[str.len() / 2..].parse().unwrap());
            } else {
                res.push(stone * 2024);
            }
        }
    }
    res
}


fn main() {
    let mut stones = read_input();
    for _ in 0..25 {
        stones = simulate_step(&stones);
    }
    println!("{}", stones.len());
}
