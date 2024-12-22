use ndarray::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Array2<char> {
    let file = File::open("inputs/12.txt");

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

fn flood_fill(
    map: &Array2<char>,
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    let mut to_visit = vec![start];
    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        if map[current] != map[start] {
            continue;
        }
        visited.insert(current);
        res.insert(current);
        if current.0 > 0 {
            to_visit.push((current.0 - 1, current.1));
        }
        if current.0 < map.shape()[0] - 1 {
            to_visit.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            to_visit.push((current.0, current.1 - 1));
        }
        if current.1 < map.shape()[1] - 1 {
            to_visit.push((current.0, current.1 + 1));
        }
    }
    res
}

fn segment_plots(map: &Array2<char>) -> Vec<(char, HashSet<(usize, usize)>)> {
    let mut res = Vec::new();
    let mut visited = HashSet::new();
    for i in 0..map.shape()[0] {
        for j in 0..map.shape()[1] {
            if visited.contains(&(i, j)) {
                continue;
            }
            let plot = flood_fill(map, (i, j), &mut visited);
            res.push((map[[i, j]], plot));
        }
    }
    res
}

fn calculate_perimeter(plot: &HashSet<(usize, usize)>) -> usize {
    let mut res = 0;
    for &(i, j) in plot.iter() {
        if i == 0 || !plot.contains(&(i - 1, j)) {
            res += 1;
        }
        if !plot.contains(&(i + 1, j)) {
            res += 1;
        }
        if j == 0 || !plot.contains(&(i, j - 1)) {
            res += 1;
        }
        if !plot.contains(&(i, j + 1)) {
            res += 1;
        }
    }
    res
}

fn score_plot(plot: &HashSet<(usize, usize)>) -> usize {
    plot.len() * calculate_perimeter(plot)
}

fn main() {
    let map = read_input();
    let plots = segment_plots(&map);
    let scores = plots.iter().map(|(c, plot)| (c, score_plot(plot)));
    println!("{:?}", scores.clone().collect::<Vec<_>>());
    println!("{}", scores.map(|(_, score)| score).sum::<usize>());
}
