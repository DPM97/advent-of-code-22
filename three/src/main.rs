use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = io::BufReader::new(File::open("./input.txt").unwrap()).lines();
    let lines = lines
        .into_iter()
        .map(|line| {
            if let Ok(l) = line {
                l.chars().collect::<Vec<char>>()
            } else {
                panic!("failed to parse line :)")
            }
        })
        .collect::<Vec<Vec<char>>>();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<Vec<char>>) {
    let lines = lines
        .into_iter()
        .map(|line| {
            let (mut one, mut two) = (vec![], vec![]);
            for i in 0..line.len() / 2 {
                one.push(line[i]);
            }
            for i in line.len() / 2..line.len() {
                two.push(line[i]);
            }
            (one, two)
        })
        .collect::<Vec<(Vec<char>, Vec<char>)>>();
    let mut sum = 0;

    for (one, two) in lines {
        let mut set = HashMap::<char, i32>::new();
        for v in one {
            (*set.entry(v).or_default()) = 1;
        }
        for v in two {
            if set.contains_key(&v) == true {
                (*set.entry(v).or_default()) = 2;
            }
        }

        for (k, v) in set {
            if v == 2 {
                if k >= 'a' {
                    sum += ((k as u8) - 0x60) as i64;
                } else {
                    sum += ((k as u8) - 0x40 + 26) as i64;
                }
            }
        }
    }
    println!("Part 1: {sum}");
}

fn part_two(lines: &Vec<Vec<char>>) {
    let mut sum = 0;

    let mut i = 0;
    while i < lines.len() - 2 {
        let mut set = HashMap::<char, i32>::new();
        for j in i..i + 3 {
            let prev = (j - i) as i32;
            for c in &lines[j] {
                let v = set.entry(*c).or_default();
                if *v == prev {
                    *v += 1;
                }
            }
        }
        for (k, v) in set {
            if v == 3 {
                if k >= 'a' {
                    sum += ((k as u8) - 0x60) as i64;
                } else {
                    sum += ((k as u8) - 0x40 + 26) as i64;
                }
            }
        }
        i += 3;
    }

    println!("Part two: {sum}");
}
