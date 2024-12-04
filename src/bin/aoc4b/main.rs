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

fn count_occurences(puzzle: &Array2<char>) -> usize {
    let mut count = 0;
    const TARGET: &str = "MAS";
    for i in 0..puzzle.shape()[0] as i32 {
        for j in 0..puzzle.shape()[1] as i32 {
            let loc = (i, j);
            for dir1 in &[(-1, -1), (1, 1)] {
                for dir2 in &[(-1, 1), (1, -1)] {
                    let loc1 = (loc.0 - dir1.0, loc.1 - dir1.1);
                    let loc2 = (loc.0 - dir2.0, loc.1 - dir2.1);
                    if let Some(chars1) = get_chars(puzzle, loc1, *dir1, TARGET.len()) {
                        if chars1.iter().collect::<String>() != TARGET {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    if let Some(chars2) = get_chars(puzzle, loc2, *dir2, TARGET.len()) {
                        if chars2.iter().collect::<String>() != TARGET {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let puzzle = read_input();
    println!("{puzzle}");
    let count = count_occurences(&puzzle);
    println!("{}", count);
}
