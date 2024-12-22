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

fn flood_fill(map: &Array2<char>, start: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
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
    let i_min = plot.iter().map(|&(i, _)| i).min().unwrap();
    let i_max = plot.iter().map(|&(i, _)| i).max().unwrap();
    let j_min = plot.iter().map(|&(_, j)| j).min().unwrap();
    let j_max = plot.iter().map(|&(_, j)| j).max().unwrap();
    let mut res = 0;
    for i in i_min..=i_max {
        for j in j_min..=j_max {
            let is_in = plot.contains(&(i, j));
            let has_left = i > 0 && plot.contains(&(i - 1, j));
            let has_right = plot.contains(&(i + 1, j));
            let has_up = j > 0 && plot.contains(&(i, j - 1));
            let has_down = plot.contains(&(i, j + 1));
            let has_left_up = i > 0 && j > 0 && plot.contains(&(i - 1, j - 1));
            let has_right_up = j > 0 && plot.contains(&(i + 1, j - 1));
            let has_right_down = plot.contains(&(i + 1, j + 1));
            let has_left_down = i > 0 && plot.contains(&(i - 1, j + 1));
            if is_in {
                if !has_left && !has_up {
                    res += 1;
                }
                if !has_up && !has_right {
                    res += 1;
                }
                if !has_right && !has_down {
                    res += 1;
                }
                if !has_down && !has_left {
                    res += 1;
                }
            } else {
                if has_left && has_up && has_left_up {
                    res += 1;
                }
                if has_up && has_right && has_right_up {
                    res += 1;
                }
                if has_right && has_down && has_right_down {
                    res += 1;
                }
                if has_down && has_left && has_left_down {
                    res += 1;
                }
            }
            // println!("{i}, {j}, {is_in}, {has_left}, {has_up}, {has_right}, {has_down}, {res}");
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
