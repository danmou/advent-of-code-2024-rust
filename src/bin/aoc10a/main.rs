use ndarray::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Array2<u8> {
    let file = File::open("inputs/10.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut res: Vec<Vec<u8>> = Vec::new();
    for line in br.lines() {
        res.push(line.unwrap().trim().chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect());
    }
    Array2::from_shape_vec(
        (res.len(), res[0].len()),
        res.into_iter().flatten().collect::<Vec<u8>>(),
    )
    .unwrap()
}

fn get_trailheads(map: &Array2<u8>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in 0..map.shape()[0] {
        for j in 0..map.shape()[1] {
            if map[[i, j]] == 0 {
                res.push((i, j));
            }
        }
    }
    res
}

fn score_trailhead(map: &Array2<u8>, trailhead: (usize, usize)) -> usize {
    let mut score = 0;
    let mut visited = HashSet::new();
    let mut to_visit = vec![trailhead];
    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        if map[current] == 9 {
            score += 1;
            continue;
        }
        let mut neighbors = Vec::new();
        if current.0 > 0 {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 < map.shape()[0] - 1 {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 < map.shape()[1] - 1 {
            neighbors.push((current.0, current.1 + 1));
        }
        for neighbor in neighbors {
            if map[neighbor] == map[current] + 1 {
                to_visit.push(neighbor);
            }
        }
    }
    score
}

fn main() {
    let map = read_input();
    let trailheads = get_trailheads(&map);
    let scores = trailheads.iter().map(|&trailhead| score_trailhead(&map, trailhead));
    println!("{}", scores.sum::<usize>());
}
