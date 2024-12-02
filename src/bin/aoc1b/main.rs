use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

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

fn calc_score(vec1: Vec<i64>, vec2: Vec<i64>) -> i64 {
    let mut counts = HashMap::new();
    for num in vec2.iter() {
        counts.insert(num, counts.get(num).unwrap_or(&0) + 1);
    }
    let mut score = 0;
    for num in vec1.iter() {
        score += num * counts.get(num).unwrap_or(&0);
    }
    score
}

fn main() {
    let (vec1, vec2) = read_input();
    let diff = calc_score(vec1, vec2);
    println!("{}", diff);
}
