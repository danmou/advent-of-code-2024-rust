use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> (Vec<i64>, Vec<i64>) {
    let file = File::open("inputs/1.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in br.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        vec1.push(split.next().unwrap().parse().unwrap());
        vec2.push(split.next().unwrap().parse().unwrap());
    }

    (vec1, vec2)
}

fn calc_diff(mut vec1: Vec<i64>, mut vec2: Vec<i64>) -> i64 {
    vec1.sort();
    vec2.sort();
    vec1.iter()
        .zip(vec2.iter())
        .fold(0, |acc, pair| acc + (pair.1 - pair.0).abs())
}

fn main() {
    let (vec1, vec2) = read_input();
    let diff = calc_diff(vec1, vec2);
    println!("{}", diff);
}
