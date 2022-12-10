use std::collections::HashSet;
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
                l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>()
            } else {
                panic!("failed to parse line :)")
            }
        })
        .collect::<Vec<Vec<String>>>();

    let now = Instant::now();
    println!("part 1: {} ({:?})", part_one(&lines), now.elapsed());
    let now = Instant::now();
    println!("part 2: {} ({:?})", part_two(&lines, 10), now.elapsed());
}

fn part_one(lines: &Vec<Vec<String>>) -> usize {
    let (mut h_x, mut h_y, mut t_x, mut t_y) = (0, 0, 0, 0);
    let mut visited = HashSet::<(i32, i32)>::from([(0, 0)]);

    for m in lines {
        let t = m[0].as_ref();
        let ct = m[1].parse::<i32>().unwrap();

        fn move_left(x: &mut i32, _y: &mut i32) {
            *x -= 1;
        }

        fn move_right(x: &mut i32, _y: &mut i32) {
            *x += 1;
        }

        fn move_up(_x: &mut i32, y: &mut i32) {
            *y += 1;
        }

        fn move_down(_x: &mut i32, y: &mut i32) {
            *y -= 1;
        }

        for _ in 0..ct {
            let (tmp_h_x, tmp_h_y) = (h_x, h_y);

            match t {
                "R" => {
                    move_right(&mut h_x, &mut h_y);
                }
                "L" => {
                    move_left(&mut h_x, &mut h_y);
                }
                "U" => {
                    move_up(&mut h_x, &mut h_y);
                }
                "D" => {
                    move_down(&mut h_x, &mut h_y);
                }
                _ => {}
            }

            if i32::abs(h_x - t_x) > 1 || i32::abs(h_y - t_y) > 1 {
                t_x = tmp_h_x;
                t_y = tmp_h_y;
                visited.insert((t_x, t_y));
            }
        }
    }

    visited.len()
}

fn part_two(lines: &Vec<Vec<String>>, rope_size: usize) -> usize {
    let mut rope = vec![(0, 0); rope_size];
    let mut visited = HashSet::<(i32, i32)>::from([(0, 0)]);

    fn move_left((x, _y): &mut (i32, i32)) {
        *x -= 1;
    }

    fn move_right((x, _y): &mut (i32, i32)) {
        *x += 1;
    }

    fn move_up((_x, y): &mut (i32, i32)) {
        *y += 1;
    }

    fn move_down((_x, y): &mut (i32, i32)) {
        *y -= 1;
    }

    for m in lines {
        let t = m[0].as_ref();
        let ct = m[1].parse::<i32>().unwrap();

        for _ in 0..ct {
            match t {
                "R" => {
                    move_right(&mut rope[0]);
                }
                "L" => {
                    move_left(&mut rope[0]);
                }
                "U" => {
                    move_up(&mut rope[0]);
                }
                "D" => {
                    move_down(&mut rope[0]);
                }
                _ => {}
            }

            for knot in 1..rope.len() {
                let (prev_x, prev_y) = rope[knot - 1];
                let (cur_x, cur_y) = rope[knot];
                let (delta_x, delta_y) = (prev_x - cur_x, prev_y - cur_y);

                if i32::abs(delta_x) > 1 || i32::abs(delta_y) > 1 {
                    rope[knot] = (
                        cur_x
                            + if delta_x > 1 {
                                1
                            } else if delta_x < -1 {
                                -1
                            } else {
                                delta_x
                            },
                        cur_y
                            + if delta_y > 1 {
                                1
                            } else if delta_y < -1 {
                                -1
                            } else {
                                delta_y
                            },
                    );

                    if knot == rope.len() - 1 {
                        visited.insert(rope[knot]);
                    }
                } else {
                    break;
                }
            }
        }
    }

    visited.len()
}
