use std::{fs::read_to_string, ops};

use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl ops::Mul<usize> for Coordinate {
    type Output = Coordinate;

    fn mul(self, rhs: usize) -> Self::Output {
        Coordinate {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
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

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: Coordinate,
    b: Coordinate,
    prize: Coordinate,
}

#[derive(Debug, Clone, Copy)]
struct Solution {
    a: usize,
    b: usize,
}

impl Solution {
    fn score(&self) -> usize {
        self.a * 3 + self.b
    }

    fn check(&self, machine: Machine) -> bool {
        let result = machine.a * self.a + machine.b * self.b;
        result == machine.prize
    }
}

fn read_input() -> Vec<Machine> {
    let content = read_to_string("inputs/13.txt").expect("Problem reading the file");
    let parts: Vec<&str> = content.trim().split("\n\n").collect();
    let mut res = Vec::new();
    for part in parts {
        let mut lines = part.lines();

        let line_a = lines.next().unwrap();
        let a_x: usize = Regex::new(r"X\+(\d+)").unwrap().captures(line_a).unwrap()[1]
            .parse()
            .unwrap();
        let a_y: usize = Regex::new(r"Y\+(\d+)").unwrap().captures(line_a).unwrap()[1]
            .parse()
            .unwrap();

        let line_b = lines.next().unwrap();
        let b_x: usize = Regex::new(r"X\+(\d+)").unwrap().captures(line_b).unwrap()[1]
            .parse()
            .unwrap();
        let b_y: usize = Regex::new(r"Y\+(\d+)").unwrap().captures(line_b).unwrap()[1]
            .parse()
            .unwrap();

        let line_prize = lines.next().unwrap();
        let prize_x: usize = Regex::new(r"X=(\d+)")
            .unwrap()
            .captures(line_prize)
            .unwrap()[1]
            .parse()
            .unwrap();
        let prize_y: usize = Regex::new(r"Y=(\d+)")
            .unwrap()
            .captures(line_prize)
            .unwrap()[1]
            .parse()
            .unwrap();

        res.push(Machine {
            a: Coordinate { x: a_x, y: a_y },
            b: Coordinate { x: b_x, y: b_y },
            prize: Coordinate {
                x: prize_x,
                y: prize_y,
            },
        });
    }
    res
}

fn solve_machine(machine: Machine) -> Option<usize> {
    let mut best = None;
    for a in 0..=100 {
        for b in 0..=100 {
            let solution = Solution { a, b };
            if solution.check(machine) && (best.is_none() || solution.score() < best.unwrap()) {
                best = Some(solution.score());
            }
        }
    }
    best
}


fn main() {
    let machines = read_input();
    // println!("{:?}", machines);
    let mut total = 0;
    for machine in machines {
        if let Some(score) = solve_machine(machine) {
            total += score;
        }
    }
    println!("Total: {}", total);
}
