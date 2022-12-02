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
        .collect::<Vec<String>>()
        .into_iter()
        .map(|s| {
            let v = s.split(' ').collect::<Vec<_>>();
            (
                v[0].chars().nth(0).unwrap() as i32 - 0x40,
                v[1].chars().nth(0).unwrap() as i32 - 0x57,
            )
        })
        .collect::<Vec<(i32, i32)>>();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<(i32, i32)>) {
    let mut total_score = 0;
    for (one, two) in lines {
        total_score += two;

        // tie
        if one == two {
            total_score += 3;
            continue;
        }

        if (*two == 1 && *one == 3) || (*two == 2 && *one == 1) || (*two == 3 && *one == 2) {
            total_score += 6;
        }
    }
    println!("Part 1: {total_score}");
}

fn part_two(lines: &Vec<(i32, i32)>) {
    let mut total_score = 0;
    for (one, two) in lines {
        // lose
        if *two == 1 {
            match one {
                1 => {
                    total_score += 3;
                }
                2 => {
                    total_score += 1;
                }
                3 => {
                    total_score += 2;
                }
                _ => {}
            }
            continue;
        }

        // tie
        if *two == 2 {
            total_score += one;
            total_score += 3;
            continue;
        }

        // win
        if *two == 3 {
            match one {
                1 => {
                    total_score += 2;
                }
                2 => {
                    total_score += 3;
                }
                3 => {
                    total_score += 1;
                }
                _ => {}
            }
            total_score += 6;
            continue;
        }
    }
    println!("Part 2: {total_score}");
}
