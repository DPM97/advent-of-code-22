use std::collections::HashSet;
use std::fmt::Debug;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Line {
    beacon: (i32, i32),
    sensor: (i32, i32),
}

fn main() {
    let lines = io::BufReader::new(File::open("./input.txt").unwrap()).lines();
    let lines = lines
        .into_iter()
        .map(|line| {
            if let Ok(l) = line {
                let l = l.split("=").collect::<Vec<&str>>();
                let mut nums: [i32; 4] = [0; 4];
                for i in 1..5 {
                    let mut num: i32 = 0;
                    let mut is_negative = false;
                    for c in l[i].chars() {
                        if c.is_numeric() {
                            num *= 10;
                            num += c.to_digit(10).unwrap() as i32;
                        } else if c == '-' {
                            is_negative = true;
                        } else {
                            continue;
                        }
                    }
                    nums[i - 1] = if is_negative { -1 * num } else { num };
                }

                Line {
                    sensor: (nums[0], nums[1]),
                    beacon: (nums[2], nums[3]),
                }
            } else {
                panic!("failed to parse line :)")
            }
        })
        .collect::<Vec<Line>>();

    let now = Instant::now();
    println!(
        "part 1: {} ({:?})",
        part_one(&lines, 2000000),
        now.elapsed()
    );

    let now = Instant::now();
    println!(
        "part 2: {} ({:?})",
        part_two(&lines, (0, 4000000)),
        now.elapsed()
    );
}

fn calc_manhattan_distance(one: (i32, i32), two: (i32, i32)) -> i32 {
    (one.0 - two.0).abs() + (one.1 - two.1).abs()
}

fn part_one(lines: &Vec<Line>, line_to_inspect: i32) -> u32 {
    let obstacles =
        HashSet::<(i32, i32)>::from_iter(lines.iter().flat_map(|l| [l.sensor, l.beacon]));

    let mut hashes = HashSet::<i32>::new();

    lines.iter().for_each(|l| {
        let sensor_to_beacon = calc_manhattan_distance(l.sensor, l.beacon);

        if !(l.sensor.1 - sensor_to_beacon < line_to_inspect
            && l.sensor.1 + sensor_to_beacon > line_to_inspect)
        {
            return;
        }

        for x in l.sensor.0 - sensor_to_beacon..=l.sensor.0 + sensor_to_beacon {
            if obstacles.contains(&(x, line_to_inspect)) {
                continue;
            }

            if calc_manhattan_distance(l.sensor, (x, line_to_inspect)) <= sensor_to_beacon {
                hashes.insert(x);
            }
        }
    });

    hashes.len() as _
}

fn is_hash(lines: &Vec<Line>, y: i32, x: i32) -> bool {
    let obstacles =
        HashSet::<(i32, i32)>::from_iter(lines.iter().flat_map(|l| [l.sensor, l.beacon]));

    for l in lines {
        let sensor_to_beacon = calc_manhattan_distance(l.sensor, l.beacon);

        if !(l.sensor.1 - sensor_to_beacon < y && l.sensor.1 + sensor_to_beacon > y) {
            continue;
        }

        if x >= l.sensor.0 - sensor_to_beacon && x <= l.sensor.0 + sensor_to_beacon {
            if obstacles.contains(&(x, y)) {
                continue;
            }

            if calc_manhattan_distance(l.sensor, (x, y)) <= sensor_to_beacon {
                return true;
            }
        }
    }

    false
}

fn in_range((x, y): (i32, i32), (min, max): (i32, i32)) -> bool {
    !(x < min || y < min || x > max || y > max)
}

fn part_two(lines: &Vec<Line>, (min, max): (i32, i32)) -> i64 {
    let obstacles =
        HashSet::<(i32, i32)>::from_iter(lines.iter().flat_map(|l| [l.sensor, l.beacon]));

    let mut visited = HashSet::<(i32, i32)>::new();
    for l in lines {
        let rad = calc_manhattan_distance(l.sensor, l.beacon);

        let mut width = 0;
        let mut cur_y = l.sensor.1 - rad - 1;
        'l: loop {
            // left, right
            for (x, y) in [(l.sensor.0 - width, cur_y), (l.sensor.0 + width, cur_y)] {
                if visited.contains(&(x, y)) && !is_hash(lines, y, x) {
                    return (x as i64 * 4000000) + y as i64;
                } else if in_range((x, y), (min, max)) && !obstacles.contains(&(x, y)) {
                    visited.insert((x, y));
                }
            }

            cur_y += 1;

            if cur_y >= l.sensor.1 {
                width -= 1;
            } else {
                width += 1;
            }

            if width == -1 {
                break 'l;
            }
        }
    }

    panic!("oop");
}
