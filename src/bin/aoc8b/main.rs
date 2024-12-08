use gcd::Gcd;
use itertools::Itertools;
use ndarray::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SIZE: isize = 50;

fn read_input() -> HashMap<char, Vec<(usize, usize)>> {
    let file = File::open("inputs/8.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut antenna_locations = HashMap::new();
    for (i, line) in br.lines().enumerate() {
        for (j, c) in line.unwrap().trim().chars().enumerate() {
            if c == '.' {
                continue;
            }
            antenna_locations
                .entry(c)
                .or_insert(Vec::new())
                .push((i, j));
        }
    }
    antenna_locations
}

fn map_antinodes(antenna_locations: &HashMap<char, Vec<(usize, usize)>>) -> Array2<bool> {
    let mut map = Array2::<bool>::from_elem((SIZE as usize, SIZE as usize), false);
    for (_c, locations) in antenna_locations {
        for (loc1, loc2) in locations.iter().tuple_combinations() {
            let (x1, y1) = *loc1;
            let (x2, y2) = *loc2;
            let dx = x2 as isize - x1 as isize;
            let dy = y2 as isize - y1 as isize;
            let gcd = (dx.abs() as usize).gcd(dy.abs() as usize) as isize;
            let dx = dx / gcd;
            let dy = dy / gcd;
            for i in -SIZE..SIZE {
                let x = x1 as isize + i * dx;
                let y = y1 as isize + i * dy;
                if x >= 0 && x < SIZE && y >= 0 && y < SIZE {
                    map[[x as usize, y as usize]] = true;
                }
            }
        }
    }
    map
}

fn main() {
    let antenna_locations = read_input();
    let map = map_antinodes(&antenna_locations);
    println!("{}", map.iter().filter(|&&b| b).count());
}
