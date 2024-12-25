use ndarray::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn update(&mut self, move_: Move) {
        self.x += move_.dx();
        self.y += move_.dy();
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn dx(&self) -> isize {
        match self {
            Move::Left => -1,
            Move::Right => 1,
            _ => 0,
        }
    }

    fn dy(&self) -> isize {
        match self {
            Move::Up => -1,
            Move::Down => 1,
            _ => 0,
        }
    }
}

fn read_input() -> (Array2<Cell>, Position, Vec<Move>) {
    let file = File::open("inputs/15.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);
    let mut lines = br.lines();

    let mut map = Vec::new();
    let mut robot = Option::None;
    for line in &mut lines {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                'O' => Cell::Box,
                '@' => {
                    robot = Some(Position {
                        x: row.len() as isize,
                        y: map.len() as isize,
                    });
                    Cell::Empty
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

    let mut moves = Vec::new();
    for line in lines {
        let line = line.unwrap();
        for c in line.chars() {
            moves.push(match c {
                '^' => Move::Up,
                'v' => Move::Down,
                '<' => Move::Left,
                '>' => Move::Right,
                _ => panic!("Invalid character in input"),
            });
        }
    }
    (map, robot.unwrap(), moves)
}

fn print_map(map: &Array2<Cell>, robot: &Position) {
    for i in 0..map.shape()[0] {
        for j in 0..map.shape()[1] {
            if i == robot.y as usize && j == robot.x as usize {
                assert_eq!(map[[i, j]], Cell::Empty);
                print!("@");
            } else {
                print!(
                    "{}",
                    match map[[i, j]] {
                        Cell::Empty => '.',
                        Cell::Wall => '#',
                        Cell::Box => 'O',
                    }
                );
            }
        }
        println!();
    }
}

fn simulate(map: &mut Array2<Cell>, robot: &mut Position, move_: Move) {
    let mut pos = robot.clone();
    pos.update(move_);
    let mut boxes = Vec::new();
    while map[[pos.y as usize, pos.x as usize]] == Cell::Box {
        pos.update(move_);
        boxes.push(pos);
    }
    match map[[pos.y as usize, pos.x as usize]] {
        Cell::Empty => {
            for box_ in boxes {
                map[[box_.y as usize, box_.x as usize]] = Cell::Box;
            }
            robot.update(move_);
            map[[robot.y as usize, robot.x as usize]] = Cell::Empty;
        }
        Cell::Wall => {}
        Cell::Box => {panic!("Invalid state");}
    }
}

fn score(map: &Array2<Cell>) -> usize {
    let mut res = 0;
    for i in 0..map.shape()[0] {
        for j in 0..map.shape()[1] {
            if map[[i, j]] == Cell::Box {
                res += 100 * i + j;
            }
        }
    }
    res
}

fn main() {
    let (mut map, mut robot, moves) = read_input();
    print_map(&map, &robot);
    for move_ in moves {
        simulate(&mut map, &mut robot, move_);
        // println!("Move: {:?}", move_);
        // print_map(&map, &robot);
        // println!();
    }
    println!("{}", score(&map));
}
