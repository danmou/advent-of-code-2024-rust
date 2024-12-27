use ndarray::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Free,
    Wall,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl ops::Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Coordinate {
    fn move_forward(&self, orientation: Orientation) -> Self {
        *self
            + match orientation {
                Orientation::North => Coordinate { x: 0, y: -1 },
                Orientation::East => Coordinate { x: 1, y: 0 },
                Orientation::South => Coordinate { x: 0, y: 1 },
                Orientation::West => Coordinate { x: -1, y: 0 },
            }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn rotate_left(&self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Move {
    Forward,
    RotateLeft,
    RotateRight,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pose {
    position: Coordinate,
    orientation: Orientation,
}

impl Pose {
    fn apply(&self, move_: Move) -> Self {
        match move_ {
            Move::Forward => Pose {
                position: self.position.move_forward(self.orientation),
                orientation: self.orientation,
            },
            Move::RotateLeft => Pose {
                position: self.position,
                orientation: self.orientation.rotate_left(),
            },
            Move::RotateRight => Pose {
                position: self.position,
                orientation: self.orientation.rotate_right(),
            },
        }
    }
}

trait CellGetter {
    fn at(&self, pos: Coordinate) -> Cell;
}

impl CellGetter for Array2<Cell> {
    fn at(&self, pos: Coordinate) -> Cell {
        self[[pos.y as usize, pos.x as usize]]
    }
}

fn read_input() -> (Array2<Cell>, Pose, Coordinate) {
    let file = File::open("inputs/16.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut map = Vec::new();
    let mut start = Option::None;
    let mut goal = Option::None;
    for line in br.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '.' => Cell::Free,
                '#' => Cell::Wall,
                'S' => {
                    assert!(start.is_none());
                    start = Some(Pose {
                        position: Coordinate {
                            x: row.len() as isize,
                            y: map.len() as isize,
                        },
                        orientation: Orientation::East,
                    });
                    Cell::Free
                }
                'E' => {
                    assert!(goal.is_none());
                    goal = Some(Coordinate {
                        x: row.len() as isize,
                        y: map.len() as isize,
                    });
                    Cell::Free
                }
                _ => panic!("Invalid character in input"),
            });
        }
        map.push(row);
    }
    let map = Array2::from_shape_vec(
        (map.len(), map[0].len()),
        map.into_iter().flatten().collect::<Vec<Cell>>(),
    )
    .unwrap();

    (map, start.unwrap(), goal.unwrap())
}

fn print_map(map: &Array2<Cell>, poses: &Vec<Pose>, goal: Coordinate) {
    let poses: HashMap<Coordinate, &Pose> =
        poses.iter().map(|pose| (pose.position, pose)).collect();
    for i in 0..map.shape()[0] {
        for j in 0..map.shape()[1] {
            let coord = Coordinate {
                x: j as isize,
                y: i as isize,
            };
            print!(
                "{}",
                if let Some(pose) = poses.get(&coord) {
                    assert_eq!(map[[i, j]], Cell::Free);
                    match pose.orientation {
                        Orientation::North => '^',
                        Orientation::East => '>',
                        Orientation::South => 'v',
                        Orientation::West => '<',
                    }
                } else if coord == goal {
                    assert_eq!(map[[i, j]], Cell::Free);
                    'E'
                } else {
                    match map[[i, j]] {
                        Cell::Free => '.',
                        Cell::Wall => '#',
                    }
                }
            );
        }
        println!();
    }
}

fn apply_moves(pose: Pose, moves: &Vec<Move>) -> Vec<Pose> {
    let mut poses = vec![pose];
    for move_ in moves {
        poses.push(poses.last().unwrap().apply(*move_));
    }
    poses
}

fn score(moves: &Vec<Move>) -> usize {
    moves
        .into_iter()
        .map(|move_| match move_ {
            Move::Forward => 1,
            Move::RotateLeft | Move::RotateRight => 1000,
        })
        .sum()
}

fn solve(map: &Array2<Cell>, start: Pose, goal: Coordinate, max_score: usize) -> (usize, Vec<Vec<Move>>) {
    let mut visited: HashMap<Pose, (usize, Vec<Vec<Move>>)> = HashMap::new();
    let mut queue = vec![(vec![], start)];
    let mut solutions = vec![];
    let mut best_score = None;
    while let Some((moves, pose)) = queue.pop() {
        let current_score = score(&moves);
        if pose.position == goal {
            if best_score.is_none() || current_score < best_score.unwrap() {
                best_score = Some(current_score);
                solutions = vec![moves];
            } else if current_score == best_score.unwrap() {
                solutions.push(moves);
            }
            continue;
        }
        if current_score >= best_score.unwrap_or(max_score).min(max_score) {
            // println!("skipping");
            continue;
        }
        // println!("{} {} {} {}", current_score, moves.len(), best_score.unwrap_or(999999), visited.contains_key(&pose));
        if visited.contains_key(&pose) && visited[&pose].0 < current_score {
            // println!("visited");
            continue;
        } else if visited.contains_key(&pose) && visited[&pose].0 == current_score {
            visited.get_mut(&pose).unwrap().1.push(moves.clone());
            continue;
        }
        // } else if visited.contains_key(&pose) {
            // println!("visited but {} < {}", current_score, visited[&pose]);
        visited.insert(pose, (current_score, vec![moves.clone()]));
        println!("queue: {} best: {} current: {} {}", queue.len(), best_score.unwrap_or(999999), current_score, moves.len());
        // println!("queue: {} best: {} current: {} {}", queue.len(), best_score.unwrap_or(999999), current_score, moves.iter().map(|m| match m {
        //     Move::Forward => 'F',
        //     Move::RotateLeft => 'L',
        //     Move::RotateRight => 'R',
        // }).collect::<String>());
        for move_ in [Move::Forward, Move::RotateLeft, Move::RotateRight].iter() {
            let new_pose = pose.apply(*move_);
            if map.at(new_pose.position) == Cell::Free {
                let mut new_moves = moves.clone();
                new_moves.push(*move_);
                if move_ == &Move::Forward {
                    queue.insert(0, (new_moves, new_pose));
                } else {
                    queue.push((new_moves, new_pose));
                }
            }
        }
    }
    assert!(solutions.len() == 1);
    let mut all_solutions = HashSet::new();
    all_solutions.insert(solutions[0].clone());
    let mut i = 0;
    while i < solutions.len() {
        let sol = solutions[i].clone();
        for pose in apply_moves(start, &sol).iter().rev().skip(1) {
            let paths = &visited[pose].1;
            assert!(paths.len()  > 0);
            if paths.len() == 1 {
                continue;
            }
            for path in paths.iter().skip(1) {
                let remainder = sol[path.len()..].to_vec();
                let new_sol = path.iter().chain(remainder.iter()).copied().collect::<Vec<Move>>();
                if all_solutions.insert(new_sol.clone()) {
                    solutions.push(new_sol);
                }
            }
        }
        i += 1;
    }
    (best_score.unwrap(), solutions)
}

fn main() {
    let now = Instant::now();
    let (map, start, goal) = read_input();
    let (best_score, solutions) = solve(&map, start, goal, 94444);
    let mut best_tiles = HashSet::from([start.position]);
    for solution in solutions.iter() {
        assert!(score(&solution) == best_score);
        let poses = apply_moves(start, &solution);
        assert!(*poses.first().unwrap() == start);
        assert!(poses.last().unwrap().position == goal);
        for pose in poses {
            best_tiles.insert(pose.position);
        }
    }
    println!("Score: {}, solutions: {}", best_score, solutions.len());
    println!("Best tiles: {}", best_tiles.len());
    println!("{}s", now.elapsed().as_secs_f64());
}
