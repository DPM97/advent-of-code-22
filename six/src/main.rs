use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = io::BufReader::new(File::open("./input.txt").unwrap()).lines();
    let lines = lines
        .into_iter()
        .map(|line| {
            if let Ok(l) = line {
                l
            } else {
                panic!("failed to parse line :)")
            }
        })
        .collect::<Vec<String>>();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<String>) {
    let text: Vec<char> = lines[0].chars().collect();
    let mut chars = HashMap::<char, i32>::new();
    for i in 0..4 {
        (*chars.entry(text[i]).or_default()) += 1;
    }

    for i in 4..text.len() {
        if chars.len() == 4 {
            println!("Part 1: {}", i);
            return;
        }
        (*chars.entry(text[i-4]).or_default()) -= 1;
        if *chars.get(&text[i-4]).unwrap() == 0 {
            chars.remove(&text[i-4]);
        }
        (*chars.entry(text[i]).or_default()) += 1;
    }
}

fn part_two(lines: &Vec<String>) {
    let text: Vec<char> = lines[0].chars().collect();
    let mut chars = HashMap::<char, i32>::new();
    for i in 0..14 {
        (*chars.entry(text[i]).or_default()) += 1;
    }

    for i in 14..text.len() {
        if chars.len() == 14 {
            println!("Part 2: {}", i);
            return;
        }
        (*chars.entry(text[i-14]).or_default()) -= 1;
        if *chars.get(&text[i-14]).unwrap() == 0 {
            chars.remove(&text[i-14]);
        }
        (*chars.entry(text[i]).or_default()) += 1;
    }
}
