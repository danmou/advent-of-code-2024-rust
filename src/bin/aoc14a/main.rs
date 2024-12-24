use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl ops::Rem for Coordinate {
    type Output = Coordinate;

    fn rem(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: Coordinate,
    v: Coordinate,
}

fn read_input() -> Vec<Robot> {
    let file = File::open("inputs/14.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut res = Vec::new();
    for line in br.lines() {
        let line = line.unwrap();
        let parts = line.split(" ").collect::<Vec<&str>>();
        let p_parts = parts[0].split("=").collect::<Vec<&str>>();
        let p = p_parts[1].split(",").collect::<Vec<&str>>();
        let p = Coordinate {
            x: p[0].parse().unwrap(),
            y: p[1].parse().unwrap(),
        };
        let v_parts = parts[1].split("=").collect::<Vec<&str>>();
        let v = v_parts[1].split(",").collect::<Vec<&str>>();
        let v = Coordinate {
            x: v[0].parse().unwrap(),
            y: v[1].parse().unwrap(),
        };
        res.push(Robot { p, v });
    }
    res
}

fn simulate(robots: &mut Vec<Robot>, dim: Coordinate) {
    for robot in robots.iter_mut() {
        robot.p = (robot.p + robot.v) % dim;
    }
}

fn score(robots: &Vec<Robot>, dim: Coordinate) -> usize {
    let mut quadrants = [0; 4];
    for robot in robots.iter() {
        if robot.p.x < dim.x / 2 && robot.p.y < dim.y / 2 {
            quadrants[0] += 1;
        } else if robot.p.x > dim.x / 2 && robot.p.y < dim.y / 2 {
            quadrants[1] += 1;
        } else if robot.p.x < dim.x / 2 && robot.p.y > dim.y / 2 {
            quadrants[2] += 1;
        } else if robot.p.x > dim.x / 2 && robot.p.y > dim.y / 2 {
            quadrants[3] += 1;
        }
    }
    quadrants.iter().product()
}

fn main() {
    let mut robots = read_input();
    // let dim = Coordinate { x: 11, y: 7 };
    let dim = Coordinate { x: 101, y: 103 };
    for _ in 0..100 {
        simulate(&mut robots, dim);
    }
    // println!("{:?}", robots);
    println!("{}", score(&robots, dim));
}
