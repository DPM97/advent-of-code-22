use std::time::Instant;
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
                l.chars()
                    .map(|x| x.to_string().parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            } else {
                panic!("failed to parse line :)")
            }
        })
        .collect::<Vec<Vec<u8>>>();

    let now = Instant::now();
    println!("part 1: {} ({:?})", part_one(&lines), now.elapsed());
    let now = Instant::now();
    println!("part 2: {} ({:?})", part_two(&lines), now.elapsed());
}

fn part_one(map: &Vec<Vec<u8>>) -> i32 {
    let mut visible = vec![vec![false; map[0].len()]; map.len()];

    // max from top
    let mut max = vec![vec![0; map[0].len()]; map.len()];
    for j in 0..map[0].len() {
        max[0][j] = map[0][j]
    }
    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            if map[i][j] > max[i - 1][j] {
                visible[i][j] = true;
                max[i][j] = map[i][j];
            } else {
                max[i][j] = max[i - 1][j]
            }
        }
    }

    // max from bottom
    max = vec![vec![0; map[0].len()]; map.len()];
    for j in 0..map[0].len() {
        max[map.len() - 1][j] = map[map.len() - 1][j]
    }
    for i in (1..map.len() - 1).rev() {
        for j in 1..map[0].len() - 1 {
            if map[i][j] > max[i + 1][j] {
                visible[i][j] = true;
                max[i][j] = map[i][j];
            } else {
                max[i][j] = max[i + 1][j]
            }
        }
    }

    // max from left
    max = vec![vec![0; map[0].len()]; map.len()];
    for i in 0..map.len() {
        max[i][0] = map[i][0]
    }
    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            if map[i][j] > max[i][j - 1] {
                visible[i][j] = true;
                max[i][j] = map[i][j];
            } else {
                max[i][j] = max[i][j - 1]
            }
        }
    }

    // max from right
    max = vec![vec![0; map[0].len()]; map.len()];
    for i in 0..map.len() {
        max[i][map.len() - 1] = map[i][map.len() - 1]
    }
    for i in 1..map.len() - 1 {
        for j in (1..map[0].len() - 1).rev() {
            if map[i][j] > max[i][j + 1] {
                visible[i][j] = true;
                max[i][j] = map[i][j];
            } else {
                max[i][j] = max[i][j + 1]
            }
        }
    }

    let v: usize = visible
        .iter()
        .map(|v| v.iter().filter(|&x| *x == true).count())
        .sum();

    (v + ((map.len() * 2) + (map[0].len() * 2) - 4)) as i32
}

fn part_two(map: &Vec<Vec<u8>>) -> i32 {
    let mut max = 0;
    for (i, v) in map.iter().enumerate() {
        for (j, x) in v.iter().enumerate() {
            let mut res = 1;

            // left
            let mut mult = 0;
            let mut cur_x = j as i32 - 1;
            loop {
                if cur_x == -1 {
                    break;
                }
                if map[i][cur_x as usize] < *x {
                    mult += 1;
                    cur_x -= 1;
                } else {
                    mult += 1;
                    break;
                }
            }
            res *= mult;

            // right
            mult = 0;
            cur_x = j as i32 + 1;
            loop {
                if cur_x == map[0].len() as i32 {
                    break;
                }
                if map[i][cur_x as usize] < *x {
                    mult += 1;
                    cur_x += 1;
                } else {
                    mult += 1;
                    break;
                }
            }
            res *= mult;

            // up
            mult = 0;
            let mut cur_y = i as i32 - 1;
            loop {
                if cur_y == -1 as i32 {
                    break;
                }
                if map[cur_y as usize][j] < *x {
                    mult += 1;
                    cur_y -= 1;
                } else {
                    mult += 1;
                    break;
                }
            }
            res *= mult;

            // down
            mult = 0;
            cur_y = i as i32 + 1;
            loop {
                if cur_y == map.len() as i32 {
                    break;
                }
                if map[cur_y as usize][j] < *x {
                    mult += 1;
                    cur_y += 1;
                } else {
                    mult += 1;
                    break;
                }
            }
            res *= mult;

            max = std::cmp::max(max, res);
        }
    }
    max as _
}
