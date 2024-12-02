use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<Vec<i16>> {
    let file = File::open("inputs/2.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut res = Vec::new();
    for line in br.lines() {
        res.push(
            line.unwrap()
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i16>>(),
        );
    }
    res
}

fn is_safe(levels: &Vec<i16>) -> bool {
    let increasing = levels[1] > levels[0];
    for i in 1..levels.len() {
        if (levels[i] > levels[i - 1]) != increasing {
            return false;
        }
        let diff = (levels[i] - levels[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn count_safe(report: &Vec<Vec<i16>>) -> usize {
    let mut count = 0;
    for row in report.iter() {
        if is_safe(row) {
            count += 1;
        } else {
            for i in 0..row.len() {
                let new_row = row.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| *v).collect();
                if is_safe(&new_row) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

fn main() {
    let report = read_input();
    let count = count_safe(&report);
    println!("{}", count);
}
