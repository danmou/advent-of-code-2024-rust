use itertools::Itertools;
use ndarray::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SIZE: usize = 50;
// const SIZE: usize = 12;

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
    let mut map = Array2::<bool>::from_elem((SIZE, SIZE), false);
    for (c, locations) in antenna_locations {
        // println!("Locations for {}: {:?}", c, locations);
        for (loc1, loc2) in locations.iter().tuple_combinations() {
            // println!("Checking {:?} and {:?}", loc1, loc2);
            let (x1, y1) = *loc1;
            let (x2, y2) = *loc2;
            let dx = x2 as isize - x1 as isize;
            let dy = y2 as isize - y1 as isize;
            let node1_x = x1 as isize - dx;
            let node1_y = y1 as isize - dy;
            let node2_x = x2 as isize + dx;
            let node2_y = y2 as isize + dy;
            for (x, y) in vec![(node1_x, node1_y), (node2_x, node2_y)] {
                if x >= 0 && x < SIZE as isize && y >= 0 && y < SIZE as isize {
                    // println!("Setting {:?} to true", (x, y));
                    map[[x as usize, y as usize]] = true;
                } else {
                    // println!("Skipping {:?} as out of bounds", (x, y));
                }
            }
        }
    }
    map
}

fn main() {
    let antenna_locations = read_input();
    let map = map_antinodes(&antenna_locations);
    // println!("{:?}", map);
    println!("{}", map.iter().filter(|&&b| b).count());
}
