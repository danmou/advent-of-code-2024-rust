use ndarray::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn update(&mut self, move_: Move) {
        self.x += move_.dx();
        self.y += move_.dy();
    }

    fn after(&self, move_: Move) -> Position {
        Position {
            x: self.x + move_.dx(),
            y: self.y + move_.dy(),
        }
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

trait CellGetter {
    fn at(&self, pos: Position) -> Cell;

    fn set(&mut self, pos: Position, cell: Cell);
}

impl CellGetter for Array2<Cell> {
    fn at(&self, pos: Position) -> Cell {
        self[[pos.y as usize, pos.x as usize]]
    }

    fn set(&mut self, pos: Position, cell: Cell) {
        self[[pos.y as usize, pos.x as usize]] = cell;
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
            row.extend(match c {
                '.' => [Cell::Empty, Cell::Empty],
                '#' => [Cell::Wall, Cell::Wall],
                'O' => [Cell::BoxLeft, Cell::BoxRight],
                '@' => {
                    robot = Some(Position {
                        x: row.len() as isize,
                        y: map.len() as isize,
                    });
                    [Cell::Empty, Cell::Empty]
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
                        Cell::BoxLeft => '[',
                        Cell::BoxRight => ']',
                    }
                );
            }
        }
        println!();
    }
}

fn simulate(map: &mut Array2<Cell>, robot: &mut Position, move_: Move) {
    let mut boxes = Vec::new();
    let mut push_locs = HashSet::from([robot.after(move_)]);
    while push_locs.len() != 0 {
        let mut next_push_locs = HashSet::new();
        for push_loc in push_locs {
            match map.at(push_loc) {
                Cell::Empty => (),
                Cell::Wall => return,
                Cell::BoxLeft => {
                    boxes.push(push_loc);
                    match move_ {
                        Move::Right => {
                            next_push_locs.insert(push_loc.after(move_).after(move_));
                        }
                        Move::Up | Move::Down => {
                            next_push_locs.insert(push_loc.after(move_));
                            next_push_locs.insert(push_loc.after(move_).after(Move::Right));
                        }
                        _ => {
                            panic!();
                        }
                    }
                }
                Cell::BoxRight => {
                    boxes.push(push_loc.after(Move::Left));
                    match move_ {
                        Move::Left => {
                            next_push_locs.insert(push_loc.after(move_).after(move_));
                        }
                        Move::Up | Move::Down => {
                            next_push_locs.insert(push_loc.after(move_));
                            next_push_locs.insert(push_loc.after(move_).after(Move::Left));
                        }
                        _ => {
                            panic!();
                        }
                    }
                }
            }
        }
        push_locs = next_push_locs;
    }

    for box_ in boxes.iter().rev() {
        map.set(*box_, Cell::Empty);
        map.set(box_.after(Move::Right), Cell::Empty);
        map.set(box_.after(move_), Cell::BoxLeft);
        map.set(box_.after(move_).after(Move::Right), Cell::BoxRight);
    }
    robot.update(move_);
}

fn score(map: &Array2<Cell>) -> usize {
    let mut res = 0;
    for i in 0..map.shape()[0] {
        for j in 0..map.shape()[1] {
            if map[[i, j]] == Cell::BoxLeft {
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
