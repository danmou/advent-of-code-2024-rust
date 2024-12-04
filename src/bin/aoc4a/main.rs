use ndarray::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Array2<char> {
    let file = File::open("inputs/4.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut res: Vec<Vec<char>> = Vec::new();
    for line in br.lines() {
        res.push(line.unwrap().trim().chars().collect());
    }
    Array2::from_shape_vec(
        (res.len(), res[0].len()),
        res.into_iter().flatten().collect::<Vec<char>>(),
    )
    .unwrap()
}

fn get_chars(
    puzzle: &Array2<char>,
    location: (i32, i32),
    dir: (i32, i32),
    count: usize,
) -> Option<Vec<char>> {
    let mut res = Vec::new();
    let mut loc = location;
    for _ in 0..count {
        if loc.0 >= puzzle.shape()[0] as i32
            || loc.0 < 0
            || loc.1 >= puzzle.shape()[1] as i32
            || loc.1 < 0
        {
            return None;
        }
        res.push(puzzle[[loc.0 as usize, loc.1 as usize]]);
        loc = (loc.0 + dir.0, loc.1 + dir.1);
    }
    Some(res)
}

fn count_occurences(puzzle: &Array2<char>, target: &str, dir: (i32, i32)) -> usize {
    let mut count = 0;
    for i in 0..puzzle.shape()[0] as i32 {
        for j in 0..puzzle.shape()[1] as i32 {
            let loc = (i, j);
            if let Some(chars) = get_chars(puzzle, loc, dir, target.len()) {
                if chars.iter().collect::<String>() == target {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_all_occurences(puzzle: &Array2<char>, target: &str) -> usize {
    let mut count = 0;
    for dir_i in -1..=1 {
        for dir_j in -1..=1 {
            if dir_i == 0 && dir_j == 0 {
                continue;
            }
            count += count_occurences(puzzle, target, (dir_i, dir_j));
        }
    }
    count
}

fn main() {
    let puzzle = read_input();
    println!("{puzzle}");
    let count = count_all_occurences(&puzzle, "XMAS");
    println!("{}", count);
}
