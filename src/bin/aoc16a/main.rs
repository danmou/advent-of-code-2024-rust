use ndarray::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

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

fn solve(map: &Array2<Cell>, start: Pose, goal: Coordinate) -> Vec<Move> {
    let mut visited = HashMap::new();
    let mut queue = vec![(vec![], start)];
    let mut solution = None;
    let mut best_score = None;
    while let Some((moves, pose)) = queue.pop() {
        let current_score = score(&moves);
        if pose.position == goal {
            if best_score.is_none() || current_score < best_score.unwrap() {
                best_score = Some(current_score);
                solution = Some(moves);
            }
            continue;
        }
        if best_score.is_some() && current_score >= best_score.unwrap() {
            // println!("skipping");
            continue;
        }
        // println!("{} {} {} {}", current_score, moves.len(), best_score.unwrap_or(999999), visited.contains_key(&pose));
        if visited.contains_key(&pose) && visited[&pose] <= current_score {
            // println!("visited");
            continue;
        // } else if visited.contains_key(&pose) {
            // println!("visited but {} < {}", current_score, visited[&pose]);
        }
        visited.insert(pose, current_score);
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
    solution.unwrap()
}

fn main() {
    let (map, start, goal) = read_input();
    let solution = solve(&map, start, goal);
    print_map(&map, &apply_moves(start, &solution), goal);
    println!("Score: {}", score(&solution));
}
