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
    line.chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Block {
    size: usize,
    id: Option<usize>,
}

fn unpack_disk_map(disk_map: &Vec<u8>) -> Vec<Block> {
    let mut res = Vec::new();
    let mut id: usize = 0;
    let mut file = true;
    for &len in disk_map.iter() {
        res.push(Block {
            size: len as usize,
            id: if file { Some(id) } else { None },
        });
        if file {
            id += 1;
        }
        file = !file;
    }
    res
}

fn compress(disk_map: &Vec<Block>) -> Vec<Block> {
    let mut res = disk_map.clone();
    let mut i = 0;
    let mut j = res.len() - 1;
    while j > 0 {
        if let Block { id: Some(_), size } = res[j] {
            // println!("Trying to insert {:?}", res[j]);
            while i <= j {
                // println!("Trying to insert {:?} at {:?}", res[j], res[i]);
                if res[i].id.is_some() || res[i].size < size {
                    i += 1;
                    continue;
                }
                let free_size = res[i].size - size;
                res[i] = res[j];
                res[j].id = None;
                i += 1;
                if free_size > 0 {
                    if let Block { id: None, .. } = res[i] {
                        res[i].size += free_size;
                    } else {
                        res.insert(
                            i,
                            Block {
                                size: free_size,
                                id: None,
                            },
                        );
                        j += 1;
                    }
                }
                // println!("{:?}", res);
                break;
            }
        }
        j -= 1;
        i = 0;
    }
    res
}

fn compute_checksum(disk_map: &Vec<Block>) -> usize {
    let mut sum = 0;
    let mut i = 0;
    for &block in disk_map.iter() {
        if let Some(id) = block.id {
            for _ in 0..block.size {
                sum += i * id;
                i += 1;
            }
        } else {
            i += block.size;
        }
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
