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
                l
            } else {
                panic!("failed to parse line :)")
            }
        })
        .collect::<Vec<String>>();

    let now = Instant::now();
    println!("part 1: {} ({:?})", part_one(&lines), now.elapsed());
    let now = Instant::now();
    println!(
        "part 1 (alternative): {} ({:?})",
        part_one_alternative(&lines),
        now.elapsed()
    );
    part_two(&lines);
    println!("\n\npart 2 --> ({:?})", now.elapsed());
}

fn part_one(lines: &Vec<String>) -> i32 {
    let vector = [20, 60, 100, 140, 180, 220];

    let mut ct = 1;
    let mut result = 0;
    let mut sum = 1;

    for l in lines {
        if l == "noop" {
            if vector.contains(&ct) {
                result += ct * sum;
            }
            ct += 1;
        } else {
            let v = l.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();

            if vector.contains(&ct) {
                result += ct * sum;
            } else if vector.contains(&(ct + 1)) {
                result += (ct + 1) * sum;
            }

            sum += v;
            ct += 2;
        }
    }
    result
}

fn part_one_alternative(lines: &Vec<String>) -> i32 {
    let vector = [20, 60, 100, 140, 180, 220];
    lines
        .iter()
        .fold([0, 1, 1], |[result, ct, sum], l| match l.as_ref() {
            "noop" => [
                result + if vector.contains(&ct) { ct * sum } else { 0 },
                ct + 1,
                sum,
            ],
            _ => [
                result
                    + if vector.contains(&ct) {
                        ct * sum
                    } else if vector.contains(&(ct + 1)) {
                        (ct + 1) * sum
                    } else {
                        0
                    },
                ct + 2,
                sum + l.split(" ").collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap(),
            ],
        })[0]
}

fn part_two(lines: &Vec<String>) {
    lines.iter().fold([0, 1], |[cur_cycle, cur_register], l| {
        for c in cur_cycle..=cur_cycle + (if l == "noop" { 0 } else { 1 }) {
            if c % 40 == 0 {
                print!("\n");
            }
            if (cur_register - 1..=cur_register + 1).contains(&c) {
                print!("#");
            } else {
                print!(".");
            }
        }
        match l.as_ref() {
            "noop" => [(cur_cycle + 1) % 40, cur_register],
            _ => [
                (cur_cycle + 2) % 40,
                cur_register
                    + l.split(" ").collect::<Vec<&str>>()[1]
                        .parse::<i32>()
                        .unwrap(),
            ],
        }
    });
}
