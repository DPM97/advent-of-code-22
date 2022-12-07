use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

struct TreeNode {
    size: u32,
    parent: Option<String>,
    is_dir: bool,
}

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

    let mut nodes = HashMap::<String, TreeNode>::from([(
        "".to_string(),
        TreeNode {
            is_dir: true,
            size: 0,
            parent: None,
        },
    )]);

    let mut cur_path = "".to_string();

    for l in lines {
        let l = l.split(" ").collect::<Vec<&str>>();
        match l[0] {
            "$" => match l[1] {
                "cd" => match l[2] {
                    ".." => {
                        cur_path = nodes.get(&cur_path).unwrap().parent.clone().unwrap();
                    }
                    dir => {
                        let new_path = format!("{cur_path}/{dir}");

                        nodes.entry(new_path.clone()).or_insert(TreeNode {
                            is_dir: true,
                            size: 0,
                            parent: Some(cur_path.clone()),
                        });

                        cur_path = new_path;
                    }
                },
                _ => {}
            },
            _ => match l[0] {
                "dir" => {}
                x => {
                    let new_path = format!("{cur_path}/{}", l[1]);
                    let file_size = x.parse::<u32>().unwrap();

                    let mut parent = nodes
                        .entry(new_path)
                        .or_insert(TreeNode {
                            is_dir: false,
                            size: file_size,
                            parent: Some(cur_path.clone()),
                        })
                        .parent
                        .clone();

                    while let Some(p) = parent {
                        let n = nodes.get_mut(&p).unwrap();
                        n.size += file_size;
                        parent = n.parent.clone();
                    }
                }
            },
        }
    }

    println!("tree built in {:?}", now.elapsed());

    let now = Instant::now();
    part_one(&nodes);
    println!("part 1: {} ({:?})", part_one(&nodes), now.elapsed());
    let now = Instant::now();
    part_two(&nodes);
    println!("part 2: {} ({:?})", part_two(&nodes), now.elapsed());
}

fn part_one(nodes: &HashMap<String, TreeNode>) -> u64 {
    nodes
        .iter()
        .filter_map(|(_, node)| {
            if !node.is_dir || node.size > 100000 {
                return None;
            }
            Some(node.size as u64)
        })
        .sum::<u64>()
}

fn part_two(nodes: &HashMap<String, TreeNode>) -> u32 {
    let total_space = 70000000;
    let needed_space = 30000000;

    let space_used = nodes
        .iter()
        .filter_map(|(_, node)| {
            if node.is_dir {
                return None;
            }
            Some(node.size as u64)
        })
        .sum::<u64>();

    let target = (needed_space - (total_space - space_used)) as u32;

    nodes
        .values()
        .min_by(|&a, &b| match (a.is_dir, b.is_dir) {
            (false, false) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (true, true) => match (a.size > target, b.size > target) {
                (false, false) => Ordering::Equal,
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                (true, true) => match a.size < b.size {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                },
            },
        })
        .unwrap()
        .size
}
