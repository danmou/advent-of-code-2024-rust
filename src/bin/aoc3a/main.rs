use std::fs;
use regex::Regex;

fn read_input() -> String {
    fs::read_to_string("inputs/3.txt").expect("File not found")
}

struct Mul {
    x: i32,
    y: i32,
}

fn extract_muls(code: &str) -> Vec<Mul> {
    let mut res = Vec::new();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in re.captures_iter(code) {
        res.push(Mul {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
        });
    }
    res
}

fn add_muls(muls: &Vec<Mul>) -> i32 {
    let mut res = 0;
    for mul in muls {
        res += mul.x * mul.y;
    }
    res
}

fn main() {
    let code = read_input();
    let muls = extract_muls(&code);
    let count = add_muls(&muls);
    println!("{}", count);
}
