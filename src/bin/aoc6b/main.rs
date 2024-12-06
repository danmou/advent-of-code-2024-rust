use ndarray::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq)]
enum State {
    Empty,
    Occupied,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Location {
    row: usize,
    col: usize,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Guard {
    loc: Location,
    dir: Direction,
}

fn read_input() -> (Array2<State>, Guard) {
    let file = File::open("inputs/6.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut start: Option<Guard> = None;
    let mut res: Vec<Vec<State>> = Vec::new();
    for line in br.lines() {
        let mut row = vec![];
        for c in line.unwrap().trim().chars() {
            row.push(match c {
                '.' => State::Empty,
                '#' => State::Occupied,
                '^' => {
                    assert!(start.is_none());
                    start = Some(Guard {
                        loc: Location {
                            row: res.len(),
                            col: row.len(),
                        },
                        dir: Direction::Up,
                    });
                    State::Empty
                }
                _ => panic!("Invalid character"),
            });
        }
        res.push(row);
    }
    let map = Array2::from_shape_vec(
        (res.len(), res[0].len()),
        res.into_iter().flatten().collect::<Vec<State>>(),
    )
    .unwrap();
    (map, start.unwrap())
}

fn simulate_once(map: &Array2<State>, guard: &Guard) -> Option<Guard> {
    let new_loc = match guard.dir {
        Direction::Up => {
            if guard.loc.row == 0 {
                return None;
            }
            Location {
                row: guard.loc.row - 1,
                col: guard.loc.col,
            }
        }
        Direction::Down => {
            if guard.loc.row == map.shape()[0] - 1 {
                return None;
            }
            Location {
                row: guard.loc.row + 1,
                col: guard.loc.col,
            }
        }
        Direction::Left => {
            if guard.loc.col == 0 {
                return None;
            }
            Location {
                row: guard.loc.row,
                col: guard.loc.col - 1,
            }
        }
        Direction::Right => {
            if guard.loc.col == map.shape()[1] - 1 {
                return None;
            }
            Location {
                row: guard.loc.row,
                col: guard.loc.col + 1,
            }
        }
    };
    if map[[new_loc.row, new_loc.col]] == State::Occupied {
        simulate_once(
            map,
            &Guard {
                loc: guard.loc.clone(),
                dir: match guard.dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                },
            },
        )
    } else {
        Some(Guard {
            loc: new_loc,
            dir: guard.dir.clone(),
        })
    }
}

fn simulate(map: &Array2<State>, guard: &Guard) -> HashSet<Location> {
    let mut visited = HashSet::new();
    let mut guard = guard.clone();
    loop {
        visited.insert(guard.loc.clone());
        match simulate_once(map, &guard) {
            Some(new_guard) => {
                guard = new_guard;
            }
            None => {
                break;
            }
        }
    }
    visited
}

fn will_loop(map: &Array2<State>, guard: &Guard) -> bool {
    let mut visited = HashSet::new();
    let mut guard = guard.clone();
    while !visited.contains(&guard) {
        visited.insert(guard.clone());
        match simulate_once(map, &guard) {
            Some(new_guard) => {
                guard = new_guard;
            }
            None => {
                return false;
            }
        }
    }
    true
}

fn count_looping_obstacle_locations(map: &Array2<State>, guard: &Guard) -> usize {
    let mut count = 0;
    let candidate_locations = simulate(map, guard);
    let num_candidates = candidate_locations.len();
    for (i, loc) in candidate_locations.into_iter().enumerate() {
        if i % 100 == 0 {
            println!("Checking location {i}/{num_candidates}");
        }
        if loc == guard.loc {
            continue;
        }
        let mut new_map = map.clone();
        new_map[[loc.row, loc.col]] = State::Occupied;
        // print_map(&new_map, guard);
        if will_loop(&new_map, guard) {
            count += 1;
        }
    }
    count
}

fn print_map(map: &Array2<State>, guard: &Guard) {
    for i in 0..map.shape()[0] {
        for j in 0..map.shape()[1] {
            if i == guard.loc.row && j == guard.loc.col {
                print!("^");
            } else {
                match map[[i, j]] {
                    State::Empty => print!("."),
                    State::Occupied => print!("#"),
                }
            }
        }
        println!();
    }
}

fn main() {
    let (map, start) = read_input();
    print_map(&map, &start);
    let count = count_looping_obstacle_locations(&map, &start);
    println!("{}", count);
}
