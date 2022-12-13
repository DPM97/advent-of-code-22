use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
                l.chars().collect::<Vec<char>>()
            } else {
                panic!("failed to parse line :)")
            }
        })
        .collect::<Vec<Vec<char>>>();

    let now = Instant::now();
    println!("part 1: {} ({:?})", part_one(lines.clone()), now.elapsed());

    let now = Instant::now();
    println!("part 2: {} ({:?})", part_two(lines), now.elapsed());
}

struct GraphNode {
    value: i32,
    neighbors: Vec<(i32, i32)>,
}

fn part_one(map: Vec<Vec<char>>) -> i32 {
    let mut graph_nodes = HashMap::<(i32, i32), GraphNode>::new();
    let (mut start_node, mut end_node) = (None, None);

    for (i, r) in map.iter().enumerate() {
        for (j, v) in r.iter().enumerate() {
            let value = {
                if *v == 'E' {
                    25
                } else if *v == 'S' {
                    0
                } else {
                    *v as i32 - 0x61
                }
            };

            let n = GraphNode {
                value,
                neighbors: vec![],
            };

            if *v == 'S' {
                start_node = Some((i as i32, j as i32));
            } else if *v == 'E' {
                end_node = Some((i as i32, j as i32));
            }

            graph_nodes.insert((i as i32, j as i32), n);
        }
    }

    let start_node = start_node.unwrap();
    let end_node = end_node.unwrap();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let (i, j) = (i as i32, j as i32);
            let node_value = graph_nodes.get(&(i, j)).unwrap().value;
            let mut neighbors = vec![];

            for coords in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if let Some(neighbor) = graph_nodes.get(&coords) {
                    if neighbor.value <= node_value + 1 {
                        neighbors.push(coords);
                    }
                }
            }

            graph_nodes
                .get_mut(&(i, j))
                .unwrap()
                .neighbors
                .append(&mut neighbors);
        }
    }

    let mut distances = HashMap::new();
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut to_visit = BinaryHeap::new();
    distances.insert(start_node, 0);
    to_visit.push((Reverse(0), start_node));

    while let Some((Reverse(dist), coords)) = to_visit.pop() {
        if visited.contains(&coords) {
            continue;
        }

        visited.insert(coords);

        if let Some(node) = graph_nodes.get(&coords) {
            for neighbor_coords in node.neighbors.iter() {
                if dist + 1 < *distances.get(&neighbor_coords).unwrap_or(&i32::MAX) {
                    distances.insert(*neighbor_coords, dist + 1);
                    to_visit.push((Reverse(dist + 1), *neighbor_coords));
                }
            }
        }
    }

    *distances.get(&end_node).unwrap()
}

fn part_two(map: Vec<Vec<char>>) -> i32 {
    let mut graph_nodes = HashMap::<(i32, i32), GraphNode>::new();
    let (mut start_nodes, mut end_node) = (vec![], None);

    for (i, r) in map.iter().enumerate() {
        for (j, v) in r.iter().enumerate() {
            let value = {
                if *v == 'E' {
                    25
                } else if *v == 'S' {
                    0
                } else {
                    *v as i32 - 0x61
                }
            };

            let n = GraphNode {
                value,
                neighbors: vec![],
            };

            if *v == 'S' || *v == 'a' {
                start_nodes.push((i as i32, j as i32));
            } else if *v == 'E' {
                end_node = Some((i as i32, j as i32));
            }

            graph_nodes.insert((i as i32, j as i32), n);
        }
    }

    let end_node = end_node.unwrap();

    start_nodes
        .iter()
        .map(|start_node| {
            for i in 0..map.len() {
                for j in 0..map[0].len() {
                    let (i, j) = (i as i32, j as i32);
                    let node_value = graph_nodes.get(&(i, j)).unwrap().value;
                    let mut neighbors = vec![];

                    for coords in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                        if let Some(neighbor) = graph_nodes.get(&coords) {
                            if neighbor.value <= node_value + 1 {
                                neighbors.push(coords);
                            }
                        }
                    }

                    graph_nodes
                        .get_mut(&(i, j))
                        .unwrap()
                        .neighbors
                        .append(&mut neighbors);
                }
            }

            let mut distances = HashMap::new();
            let mut visited = HashSet::<(i32, i32)>::new();
            let mut to_visit = BinaryHeap::new();
            distances.insert(start_node, 0);
            to_visit.push((Reverse(0), start_node));

            while let Some((Reverse(dist), coords)) = to_visit.pop() {
                if visited.contains(&coords) {
                    continue;
                }

                visited.insert(*coords);

                if let Some(node) = graph_nodes.get(&coords) {
                    for neighbor_coords in node.neighbors.iter() {
                        if dist + 1 < *distances.get(&neighbor_coords).unwrap_or(&i32::MAX) {
                            distances.insert(neighbor_coords, dist + 1);
                            to_visit.push((Reverse(dist + 1), neighbor_coords));
                        }
                    }
                }
            }

            *distances.get(&end_node).unwrap_or(&i32::MAX)
        })
        .min()
        .unwrap()
}
