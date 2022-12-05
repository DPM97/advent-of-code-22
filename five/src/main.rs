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
    let mut width = 0;

    let mut stacks: Vec<Vec<char>> = vec![];

    for l in lines {
        let line = l.replace("    ", "*").replace(" ", "");
        let line = line
            .split("][")
            .map(|s| s.to_string().replace("]", "").replace("[", ""))
            .collect::<Vec<String>>();

        if let Ok(_) = line[0].parse::<i32>() {
            break;
        }

        if width == 0 {
            width = line.len() + line.iter().map(|x| x.chars().count() - 1).sum::<usize>();
        }

        for _ in stacks.len()..width {
            stacks.push(vec![]);
        }

        let mut cur_stack = 0;
        for l in line {
            for c in l.chars() {
                if c != '*' {
                    stacks[cur_stack].push(c);
                }
                cur_stack += 1;
            }
        }
    }

    stacks.iter_mut().for_each(|x| x.reverse());

    let mut is_instructions = false;
    for l in lines {
        if l == "" {
            is_instructions = true;
            continue;
        }

        if !is_instructions {
            continue;
        }

        let instruction = l.split(" ").collect::<Vec<&str>>();
        let (ct, from, to) = (
            instruction[1].parse::<i32>().unwrap(),
            instruction[3].parse::<usize>().unwrap() - 1,
            instruction[5].parse::<usize>().unwrap() - 1,
        );

        for _ in 0..ct {
            let from = stacks[from].pop().unwrap();
            stacks[to].push(from);
        }
    }

    println!(
        "Part 1: {}",
        stacks
            .iter_mut()
            .map(|s| s.pop().unwrap())
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>()
    );
}

fn part_two(lines: &Vec<String>) {
    let mut width = 0;

    let mut stacks: Vec<Vec<char>> = vec![];

    for l in lines {
        let line = l.replace("    ", "*").replace(" ", "");
        let line = line
            .split("][")
            .map(|s| s.to_string().replace("]", "").replace("[", ""))
            .collect::<Vec<String>>();

        if let Ok(_) = line[0].parse::<i32>() {
            break;
        }

        if width == 0 {
            width = line.len() + line.iter().map(|x| x.chars().count() - 1).sum::<usize>();
        }

        for _ in stacks.len()..width {
            stacks.push(vec![]);
        }

        let mut cur_stack = 0;
        for l in line {
            for c in l.chars() {
                if c != '*' {
                    stacks[cur_stack].push(c);
                }
                cur_stack += 1;
            }
        }
    }

    stacks.iter_mut().for_each(|x| x.reverse());

    let mut is_instructions = false;
    for l in lines {
        if l == "" {
            is_instructions = true;
            continue;
        }

        if !is_instructions {
            continue;
        }

        let instruction = l.split(" ").collect::<Vec<&str>>();
        let (ct, from, to) = (
            instruction[1].parse::<i32>().unwrap(),
            instruction[3].parse::<usize>().unwrap() - 1,
            instruction[5].parse::<usize>().unwrap() - 1,
        );

        let mut tmp_stack = vec![];
        for _ in 0..ct {
            let from = stacks[from].pop().unwrap();
            tmp_stack.push(from);
        }

        for v in tmp_stack.into_iter().rev() {
            stacks[to].push(v);
        }
    }
    println!(
        "Part 2: {}",
        stacks
            .iter_mut()
            .map(|s| s.pop().unwrap())
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>()
    );
}
