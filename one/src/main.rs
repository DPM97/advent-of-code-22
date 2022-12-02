use std::collections::BinaryHeap;
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
    let mut max_sum = 0;
    let mut cur_sum = 0;
    for l in lines {
        if l == "" {
            max_sum = std::cmp::max(max_sum, cur_sum);
            cur_sum = 0;
        } else {
            cur_sum += l.parse::<i32>().unwrap();
        }
    }
    println!("Part 1: {max_sum}");
}

fn part_two(lines: &Vec<String>) {
    let mut cals = BinaryHeap::<i32>::new();
    let mut cur_sum = 0;
    for l in lines {
        if l == "" {
            cals.push(cur_sum);
            cur_sum = 0;
        } else {
            cur_sum += l.parse::<i32>().unwrap();
        }
    }

    let mut three_sum = 0;
    for _ in 0..3 {
        three_sum += cals.pop().unwrap();
    }

    println!("Part 2: {three_sum}");
}
