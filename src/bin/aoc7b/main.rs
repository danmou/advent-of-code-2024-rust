use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<(u64, Vec<u64>)> {
    let file = File::open("inputs/7.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut res = Vec::new();

    for line in br.lines() {
        let line = line.unwrap();
        let mut parts = line.split(": ");
        let result = parts.next().unwrap().parse::<u64>().unwrap();
        let operands = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        res.push((result, operands));
    }

    res
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Operation {
    Add,
    Mul,
    Con,
}

fn calculate_result(operands: &Vec<u64>, operations: &Vec<Operation>, limit: u64) -> Option<u64> {
    assert_eq!(operands.len(), operations.len() + 1);
    let mut res = operands[0];
    for i in 0..operations.len() {
        match operations[i] {
            Operation::Add => res += operands[i + 1],
            Operation::Mul => res *= operands[i + 1],
            Operation::Con => {
                res = (res.to_string() + &operands[i + 1].to_string())
                    .parse::<u64>()
                    .unwrap()
            }
        }
        if res > limit {
            return None;
        }
    }
    Some(res)
}

fn can_give_result(operands: &Vec<u64>, target: u64) -> bool {
    assert!(operands.len() > 1);
    for operators in itertools::repeat_n(
        vec![Operation::Add, Operation::Mul, Operation::Con].into_iter(),
        operands.len() - 1,
    )
    .multi_cartesian_product()
    {
        let result = calculate_result(operands, &operators, target);
        if result == Some(target) {
            return true;
        }
    }
    false
}

fn calculate_total(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut sum = 0;
    for (result, operands) in input {
        if can_give_result(operands, *result) {
            // println!("{} can be given by {:?}", result, operands);
            sum += result;
        } else {
            // println!("{} cannot be given by {:?}", result, operands);
        }
        // println!("{}", sum);
    }
    sum
}

fn main() {
    let input = read_input();
    let result = calculate_total(&input);
    println!("{}", result);
}
