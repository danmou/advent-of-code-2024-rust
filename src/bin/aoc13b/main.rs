use std::{fs::read_to_string, ops};

use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl ops::Mul<isize> for Coordinate {
    type Output = Coordinate;

    fn mul(self, rhs: isize) -> Self::Output {
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

fn read_input() -> Vec<Machine> {
    let content = read_to_string("inputs/13.txt").expect("Problem reading the file");
    let parts: Vec<&str> = content.trim().split("\n\n").collect();
    let mut res = Vec::new();
    for part in parts {
        let mut lines = part.lines();

        let line_a = lines.next().unwrap();
        let a_x: isize = Regex::new(r"X\+(\d+)").unwrap().captures(line_a).unwrap()[1]
            .parse()
            .unwrap();
        let a_y: isize = Regex::new(r"Y\+(\d+)").unwrap().captures(line_a).unwrap()[1]
            .parse()
            .unwrap();

        let line_b = lines.next().unwrap();
        let b_x: isize = Regex::new(r"X\+(\d+)").unwrap().captures(line_b).unwrap()[1]
            .parse()
            .unwrap();
        let b_y: isize = Regex::new(r"Y\+(\d+)").unwrap().captures(line_b).unwrap()[1]
            .parse()
            .unwrap();

        let line_prize = lines.next().unwrap();
        let prize_x = Regex::new(r"X=(\d+)")
            .unwrap()
            .captures(line_prize)
            .unwrap()[1]
            .parse::<isize>()
            .unwrap()
            + 10000000000000;
        let prize_y = Regex::new(r"Y=(\d+)")
            .unwrap()
            .captures(line_prize)
            .unwrap()[1]
            .parse::<isize>()
            .unwrap()
            + 10000000000000;

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
    // machine.a.x * a + machine.b.x * b = machine.prize.x
    // machine.a.y * a + machine.b.y * b = machine.prize.y
    // a = (machine.prize.y - machine.b.y * b) / machine.a.y
    // b = (machine.prize.y - machine.a.y * a) / machine.b.y
    // machine.a.x * a + machine.b.x * (machine.prize.y - machine.a.y * a) / machine.b.y = machine.prize.x
    // machine.a.x * a - machine.b.x * machine.a.y * a / machine.b.y = machine.prize.x - machine.b.x * machine.prize.y / machine.b.y
    // a * (machine.a.x - machine.b.x * machine.a.y / machine.b.y) = machine.prize.x - machine.b.x * machine.prize.y / machine.b.y
    // a * (machine.a.x * machine.b.y - machine.b.x * machine.a.y) = machine.prize.x * machine.b.y - machine.b.x * machine.prize.y
    // a = (machine.prize.x * machine.b.y - machine.b.x * machine.prize.y) / (machine.a.x * machine.b.y - machine.b.x * machine.a.y)
    let denom = machine.a.x * machine.b.y - machine.b.x * machine.a.y;
    println!("- Denom: {}", denom);
    if denom == 0 {
        // Linearly dependent
        println!("- Linearly dependent");
        if machine.prize.x % machine.b.x == 0
        && machine.prize.y % machine.b.y == 0
        && machine.prize.x / machine.b.x == machine.prize.y / machine.b.y
        {
            return Some((machine.prize.x / machine.b.x) as usize);
        }
        return None;
    }
    let num = machine.prize.x * machine.b.y - machine.b.x * machine.prize.y;
    println!("- Num: {}", num);
    if num % denom != 0 {
        println!("- Non-integer solution for a");
        return None;
    }
    let a = num / denom;
    let b_num = machine.prize.y - machine.a.y * a;
    if b_num % machine.b.y != 0 {
        println!("- Non-integer solution for b");
        return None;
    }
    let b = b_num / machine.b.y;
    println!("- a: {}, b: {}", a, b);
    if a > 0 && b > 0 {
        assert!(machine.a * a + machine.b * b == machine.prize);
        Some((a as usize) * 3 + (b as usize))
    } else {
        println!("- Negative solution");
        None
    }
}

fn main() {
    let machines = read_input();
    // println!("{:?}", machines);
    let mut total = 0;
    for machine in machines {
        println!("Machine: {:?}", machine);
        if let Some(score) = solve_machine(machine) {
            println!("- Score: {}", score);
            total += score;
        }
    }
    println!("Total: {}", total);
}
