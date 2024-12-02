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

fn count_safe(report: &Vec<Vec<i16>>) -> usize {
    let mut count = 0;
    for row in report.iter() {
        let increasing = row[1] > row[0];
        let mut safe = true;
        for i in 1..row.len() {
            if (row[i] > row[i - 1]) != increasing {
                safe = false;
                break;
            }
            let diff = (row[i] - row[i - 1]).abs();
            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }
        }
        if safe {
            count += 1;
        }
    }
    count
}

fn main() {
    let report = read_input();
    let count = count_safe(&report);
    println!("{}", count);
}
