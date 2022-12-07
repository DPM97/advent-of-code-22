use std::collections::{HashMap, HashSet};
use std::io::Error;
use std::{
    fs::File,
    io::{self, BufRead},
};

struct TreeNode {
    name: String,
    size: u32,
    children: HashSet<String>,
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

    let mut nodes = HashMap::<String, TreeNode>::from([(
        "".to_string(),
        TreeNode {
            name: "".to_string(),
            is_dir: true,
            size: 0,
            parent: None,
            children: HashSet::<String>::new(),
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
                        let mut new_path = cur_path.clone();
                        new_path.push_str("/");
                        new_path.push_str(dir);

                        nodes.entry(new_path.clone()).or_insert(TreeNode {
                            is_dir: true,
                            name: dir.to_string(),
                            size: 0,
                            children: HashSet::new(),
                            parent: Some(cur_path.clone()),
                        });

                        nodes.entry(cur_path.clone()).and_modify(|n| {
                            n.children.insert(new_path.clone());
                        });

                        cur_path = new_path;
                    }
                },
                "ls" => { /* skip (next line we can actually process) */ }
                _ => {
                    panic!("invalid command!")
                }
            },
            _ => {
                match l[0] {
                    "dir" => {
                        // nada
                    }
                    x => {
                        let mut new_path = cur_path.clone();
                        new_path.push_str("/");
                        new_path.push_str(l[1]);

                        let file_size = x.parse::<u32>().unwrap();

                        let mut parent = nodes
                            .entry(new_path)
                            .or_insert(TreeNode {
                                is_dir: false,
                                name: l[1].to_string(),
                                size: file_size,
                                children: HashSet::new(),
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
                }
            }
        }
    }

    part_one(&nodes);
    part_two(&nodes);
}

fn part_one(nodes: &HashMap<String, TreeNode>) {
    println!(
        "sum: {}",
        nodes
            .iter()
            .filter_map(|(_, node)| {
                if !node.is_dir || node.size > 100000 {
                    return None;
                }
                Some(node.size as u64)
            })
            .sum::<u64>()
    );
}

fn part_two(nodes: &HashMap<String, TreeNode>) {
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

    let mut sizes = nodes
        .values()
        .filter(|n| n.is_dir && n.size >= target)
        .map(|n| n.size)
        .collect::<Vec<u32>>();
    sizes.sort();
    println!("smallest: {}", sizes[0]);
}
