use std::fs;
use std::collections::HashMap;

pub fn run() {
    let input_str = fs::read_to_string("inputs/input_14.txt")
        .expect("Failed to read file");

    let input: Vec<&str> = input_str
        .split("\n\n")
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<&str>) {
    let mut pair_insertion_rules: HashMap<&str, &str> = HashMap::new();
    for line in input[1].split("\n") {
        let line_split: Vec<&str> = line
            .split(" -> ")
            .collect();

        pair_insertion_rules.insert(line_split[0], line_split[1]);
    }

    let mut polymer: String = input[0].to_string();
    for _step in 0..10 {
        let mut polymer_temp: String = String::new();
        for i in 0..polymer.len()-1 {
            polymer_temp += &polymer.chars().nth(i).unwrap().to_string();
            let pair = &polymer[i..i+2];
            polymer_temp += pair_insertion_rules.get(pair).unwrap();
        }
        polymer_temp += &polymer.chars().nth(polymer.len()-1).unwrap().to_string();
        polymer = polymer_temp;
    }

    let mut min_count = 0;
    let mut max_count = 0;
    for i in polymer.chars() {
        let count = polymer.matches(i).count();
        if count > max_count {
            max_count = count;
        }

        if min_count == 0 || count < min_count {
            min_count = count;
        }
    }

    println!("Part 1: {}", max_count - min_count);
}

fn part_two(input: &Vec<&str>) {
    let mut pair_insertion_rules: HashMap<&str, &str> = HashMap::new();
    let mut pair_counts: HashMap<&str, u64> = HashMap::new();
    for line in input[1].split("\n") {
        let line_split: Vec<&str> = line
            .split(" -> ")
            .collect();

        pair_insertion_rules.insert(line_split[0], line_split[1]);
        pair_counts.insert(line_split[0], 0);
    }

    for i in 0..input[0].chars().count()-1 {
        *pair_counts.get_mut(&input[0][i..i+2]).unwrap() += 1;
    }

    let mut element_counts: HashMap<String, u64> = HashMap::new();
    for i in input[0].chars() {
        let element_count = element_counts.entry(i.to_string()).or_insert(0);
        *element_count += 1;
    }

    for _step in 0..40 {
        let mut pair_counts_temp: HashMap<&str, u64> = pair_counts.clone();
        for i in pair_counts.keys() {
            let new_element = pair_insertion_rules.get(i).unwrap();
            let polymer = format!("{}{}{}", i.chars().nth(0).unwrap(), new_element, i.chars().nth(1).unwrap());
            let count: u64 = *pair_counts.get(i).unwrap();

            let element_count = element_counts.entry(new_element.to_string()).or_insert(0);
            *element_count += count;
    
            *pair_counts_temp.get_mut(&polymer[..2]).unwrap() += count;
            *pair_counts_temp.get_mut(&polymer[1..]).unwrap() += count;
            *pair_counts_temp.get_mut(i).unwrap() -= count;
        }
        
        pair_counts = pair_counts_temp;
    }

    let mut min_count: u64 = 0;
    let mut max_count: u64 = 0;
    for count in element_counts.values() {
        if *count > max_count {
            max_count = *count;
        }

        if min_count == 0 || *count < min_count {
            min_count = *count;
        }
    }

    println!("Part 2: {}", max_count - min_count);
}
