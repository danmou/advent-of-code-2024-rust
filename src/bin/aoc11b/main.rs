use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn read_input() -> Vec<u64> {
    let file = File::open("inputs/11.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);
    let line = br.lines().next().unwrap().unwrap();
    line.split(' ').map(|s| s.parse().unwrap()).collect()
}

// fn simulate_step(stones: &Vec<u64>, memo: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
//     let mut res = Vec::new();
//     for &stone in stones {
//         res.extend_from_slice(memo.entry(stone).or_insert_with(|| {
//             if stone == 0 {
//                 vec![1]
//             } else {
//                 let str = stone.to_string();
//                 if str.len() % 2 == 0 {
//                     vec![
//                         str[0..str.len() / 2].parse().unwrap(),
//                         str[str.len() / 2..].parse().unwrap(),
//                     ]
//                 } else {
//                     vec![stone * 2024]
//                 }
//             }
//         }));
//     }
//     res
// }

fn simulate(stone: u64, memo1: &mut HashMap<u64, Vec<u64>>, memo2: &mut HashMap<(u64, u8), usize>, steps: u8) -> usize {
    if steps == 0 {
        return 1;
    }
    if let Some(&res) = memo2.get(&(stone, steps)) {
        // println!("Cache hit {}, {}", stone, steps);
        return res;
    }
    // println!("{stone}, {}, {steps}", memo1.len());
    let next = memo1.entry(stone)
        .or_insert_with(|| {
            if stone == 0 {
                vec![1]
            } else {
                let str = stone.to_string();
                if str.len() % 2 == 0 {
                    vec![
                        str[0..str.len() / 2].parse().unwrap(),
                        str[str.len() / 2..].parse().unwrap(),
                    ]
                } else {
                    vec![stone * 2024]
                }
            }
        }).clone();
    let res = next.iter().map(|&s| simulate(s, memo1, memo2, steps - 1)).collect::<Vec<_>>().into_iter().sum();
    memo2.insert((stone, steps), res);
    res

}

fn main() {
    let stones = read_input();
    let mut memo1 = HashMap::new();
    let mut memo2 = HashMap::new();
    let mut count = 0;
    let now = Instant::now();
    for stone in stones {
        count += simulate(stone, &mut memo1, &mut memo2, 75);
    }
    println!("{}ms, {}, {}", now.elapsed().as_millis(), memo1.len(), memo2.len());
    println!("{}", count);
}
