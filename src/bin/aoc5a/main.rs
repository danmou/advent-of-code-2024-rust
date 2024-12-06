use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

fn read_input() -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let file = File::open("inputs/5.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);
    let mut lines = br.lines().map(|l| l.unwrap());

    let mut rules = Vec::new();
    let mut updates = Vec::new();
    for line in &mut lines {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let parts = line
            .split("|")
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        assert_eq!(parts.len(), 2);
        rules.push((parts[0], parts[1]));
    }
    for line in lines {
        let line = line.trim();
        updates.push(
            line.split(",")
                .map(|s| s.trim().parse::<u8>().unwrap())
                .collect::<Vec<u8>>(),
        );
    }
    (rules, updates)
}

fn rules_to_mapping(rules: &Vec<(u8, u8)>) -> HashMap<u8, HashSet<u8>> {
    let mut mapping = HashMap::new();
    for rule in rules {
        mapping.entry(rule.1).or_insert(HashSet::new()).insert(rule.0);
    }
    mapping
}

fn is_valid_update(rules: &HashMap<u8, HashSet<u8>>, update: &Vec<u8>) -> bool {
    let mut illegal: HashSet<u8> = HashSet::new();
    illegal.extend(rules.get(&update[0]).unwrap_or(&HashSet::new()));
    for i in 1..update.len() {
        if illegal.contains(&update[i]) {
            return false;
        }
        illegal.extend(rules.get(&update[i]).unwrap_or(&HashSet::new()));
    }
    true
}

fn get_valid_updates(rules: &HashMap<u8, HashSet<u8>>, updates: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    updates
        .iter()
        .filter(|update| is_valid_update(rules, update))
        .cloned()
        .collect()
}

fn get_middle_item(update: &Vec<u8>) -> u8 {
    assert!(update.len() % 2 == 1);
    update[update.len() / 2]
}

fn main() {
    let (rules, updates) = read_input();
    let rules = rules_to_mapping(&rules);
    // println!("rules={:?}", rules);
    // println!("updates={:?}", updates);
    let valid_updates = get_valid_updates(&rules, &updates);
    // println!("valid_updates={:?}", valid_updates);
    let score = valid_updates.iter().map(|update| get_middle_item(update) as usize).sum::<usize>();
    println!("score={}", score);
}
