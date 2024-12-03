use regex::Regex;
use std::fs;

fn read_input() -> String {
    fs::read_to_string("inputs/3.txt").expect("File not found")
}

enum Op {
    Mul(i32, i32),
    Do,
    Dont,
}

fn extract_ops(code: &str) -> Vec<Op> {
    let mut res = Vec::new();
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    for cap in re.captures_iter(code) {
        if &cap[1] == "do()" {
            res.push(Op::Do);
        } else if &cap[1] == "don't()" {
            res.push(Op::Dont);
        } else {
            assert!(&cap[1].starts_with("mul("));
            res.push(Op::Mul(cap[2].parse().unwrap(), cap[3].parse().unwrap()));
        }
    }
    res
}

fn add_muls(ops: &Vec<Op>) -> i32 {
    let mut res = 0;
    let mut enable = true;
    for mul in ops {
        match mul {
            Op::Mul(x, y) => {
                if enable {
                    res += x * y;
                }
            }
            Op::Do => {
                enable = true;
            }
            Op::Dont => {
                enable = false;
            }
        }
    }
    res
}

fn main() {
    let code = read_input();
    let ops = extract_ops(&code);
    let count = add_muls(&ops);
    println!("{}", count);
}
