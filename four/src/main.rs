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
    let pairs = lines
        .into_iter()
        .map(|c| {
            c.split(',')
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|v| {
                    v.to_string()
                        .split('-')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>()
        })
        .collect::<Vec<Vec<Vec<String>>>>();

    let mut ct = 0;
    for p in pairs {
        if (p[0][0].parse::<i32>().unwrap() >= p[1][0].parse::<i32>().unwrap()
            && p[0][1].parse::<i32>().unwrap() <= p[1][1].parse::<i32>().unwrap())
            || (p[1][0].parse::<i32>().unwrap() >= p[0][0].parse::<i32>().unwrap()
                && p[1][1].parse::<i32>().unwrap() <= p[0][1].parse::<i32>().unwrap())
        {
            ct += 1;
        }
    }

    println!("Part 1: {}", ct);
}

fn part_two(lines: &Vec<String>) {
    let pairs = lines
        .into_iter()
        .map(|c| {
            c.split(',')
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|v| {
                    v.to_string()
                        .split('-')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>()
        })
        .collect::<Vec<Vec<Vec<String>>>>();

    let mut ct = 0;
    for p in pairs {
        let (one_left, one_right) = (
            p[0][0].parse::<i32>().unwrap(),
            p[0][1].parse::<i32>().unwrap(),
        );
        let (two_left, two_right) = (
            p[1][0].parse::<i32>().unwrap(),
            p[1][1].parse::<i32>().unwrap(),
        );

        if (one_left <= two_right && one_right >= two_left)
            || (two_right <= one_left && two_right >= one_left)
        {
            ct += 1;
        }
    }
    println!("Part 2: {}", ct);
}
