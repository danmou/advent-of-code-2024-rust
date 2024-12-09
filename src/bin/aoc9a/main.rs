use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<u8> {
    let file = File::open("inputs/9.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let line = br.lines().next().unwrap().unwrap();
    line.chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Block {
    Free,
    File(usize),
}

fn unpack_disk_map(disk_map: &Vec<u8>) -> Vec<Block> {
    let mut res = Vec::new();
    let mut id: usize = 0;
    let mut file = true;
    for &len in disk_map.iter() {
        for _ in 0..len {
            res.push(if file { Block::File(id) } else { Block::Free });
        }
        if file {
            id += 1;
        }
        file = !file;
    }
    res
}

fn compress(disk_map: &Vec<Block>) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut j = disk_map.len() - 1;
    while j >= i {
        match disk_map[i] {
            Block::File(id) => {
                i += 1;
                res.push(id);
                continue;
            }
            Block::Free => {}
        }
        match disk_map[j] {
            Block::Free => {}
            Block::File(id) => {
                res.push(id);
                i += 1;
            }
        }
        j -= 1;
    }
    res
}

fn compute_checksum(compressed: &Vec<usize>) -> usize {
    let mut sum = 0;
    for (i, &id) in compressed.iter().enumerate() {
        sum += i * id;
    }
    sum
}

fn main() {
    let disk_map = read_input();
    let unpacked = unpack_disk_map(&disk_map);
    let compressed = compress(&unpacked);
    // println!("{:?}", unpacked);
    // println!("{:?}", compressed);
    println!("{}", compute_checksum(&compressed));
}
