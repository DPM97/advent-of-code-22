use core::fmt;
use std::cell::RefCell;
use std::fmt::Debug;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

struct Monkey {
    id: i64,
    inspected_ct: i64,
    items: Vec<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> bool>,
    test_val: i64,
    if_test_true: i64,
    if_test_false: i64,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ id: {}, items: {:?}, if_test_true: {}, if_test_false: {} }}",
            self.id, self.items, self.if_test_true, self.if_test_false
        )
    }
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
    println!("part 1: {} ({:?})", part_one(&lines, 10000), now.elapsed());
    let now = Instant::now();
    println!("part 2: {} ({:?})", part_two(&lines, 10000), now.elapsed());
}

fn part_one(lines: &Vec<String>, rounds: i64) -> i64 {
    let mut monkeys: Vec<RefCell<Monkey>> = vec![];

    let mut cur_monkey = Monkey {
        id: 0,
        inspected_ct: 0,
        items: vec![],
        op: Box::new(|_x| 0),
        test: Box::new(|_x| true),
        test_val: 0,
        if_test_false: 0,
        if_test_true: 0,
    };

    for l in lines {
        if l == "" {
            monkeys.push(RefCell::new(cur_monkey));
            cur_monkey = Monkey {
                id: 0,
                inspected_ct: 0,
                items: vec![],
                op: Box::new(|_x| 0),
                test: Box::new(|_x| true),
                test_val: 0,
                if_test_false: 0,
                if_test_true: 0,
            };
        }

        if l.starts_with("Monkey ") {
            cur_monkey.id = l.replace(":", "").split(" ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            continue;
        }

        if l.starts_with("  Starting items: ") {
            cur_monkey.items = l.replace(" ", "").split(":").collect::<Vec<&str>>()[1]
                .split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            continue;
        }

        if l.starts_with("  Test: ") {
            let split = l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            cur_monkey.test_val = split[split.len() - 1].parse::<i64>().unwrap();
            cur_monkey.test = Box::new(move |x: i64| -> bool {
                x as f64 / split[split.len() - 1].parse::<f64>().unwrap() % 1.0 == 0.0
            });
            continue;
        }

        if l.starts_with("  Operation: ") {
            let split = l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            cur_monkey.op = Box::new(move |x: i64| -> i64 {
                let v = if let Ok(s) = split[split.len() - 1].parse::<i64>() {
                    s
                } else {
                    x
                };
                match split[split.len() - 2].as_ref() {
                    "+" => x + v,
                    "-" => x - v,
                    "*" => x * v,
                    "/" => x / v,
                    _ => panic!("invalid *_*"),
                }
            });
            continue;
        }

        if l.starts_with("    If true: ") {
            cur_monkey.if_test_true = l.split(" ").last().unwrap().parse::<i64>().unwrap();
            continue;
        }

        if l.starts_with("    If false: ") {
            cur_monkey.if_test_false = l.split(" ").last().unwrap().parse::<i64>().unwrap();
            continue;
        }
    }

    monkeys.push(RefCell::new(cur_monkey));

    for _ in 0..rounds {
        for k in 0..monkeys.len() {
            let mut monkey = monkeys[k].borrow_mut();
            for v in monkey.items.iter() {
                // inspection
                let mut v = (*monkey.op)(*v);
                // div by 3
                v = (v as f64 / 3.0).floor() as i64;
                // test
                if (monkey.test)(v) {
                    monkeys[monkey.if_test_true as usize]
                        .borrow_mut()
                        .items
                        .push(v);
                } else {
                    monkeys[monkey.if_test_false as usize]
                        .borrow_mut()
                        .items
                        .push(v);
                }
            }
            // increment inspected ct
            monkey.inspected_ct += monkey.items.len() as i64;
            monkey.items.clear();
        }
    }

    let mut cts = monkeys
        .into_iter()
        .map(|x| x.borrow().inspected_ct)
        .collect::<Vec<i64>>();
    cts.sort_by(|a, b| b.cmp(&a));
    cts[0] * cts[1]
}

fn part_two(lines: &Vec<String>, rounds: i64) -> i64 {
    let mut monkeys: Vec<RefCell<Monkey>> = vec![];

    let mut cur_monkey = Monkey {
        id: 0,
        inspected_ct: 0,
        items: vec![],
        op: Box::new(|_x| 0),
        test: Box::new(|_x| true),
        test_val: 0,
        if_test_false: 0,
        if_test_true: 0,
    };

    for l in lines {
        if l == "" {
            monkeys.push(RefCell::new(cur_monkey));
            cur_monkey = Monkey {
                id: 0,
                inspected_ct: 0,
                items: vec![],
                op: Box::new(|_x| 0),
                test: Box::new(|_x| true),
                test_val: 0,
                if_test_false: 0,
                if_test_true: 0,
            };
        }

        if l.starts_with("Monkey ") {
            cur_monkey.id = l.replace(":", "").split(" ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            continue;
        }

        if l.starts_with("  Starting items: ") {
            cur_monkey.items = l.replace(" ", "").split(":").collect::<Vec<&str>>()[1]
                .split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            continue;
        }

        if l.starts_with("  Test: ") {
            let split = l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            cur_monkey.test_val = split[split.len() - 1].parse::<i64>().unwrap();
            cur_monkey.test = Box::new(move |x: i64| -> bool {
                x % split[split.len() - 1].parse::<i64>().unwrap() == 0
            });
            continue;
        }

        if l.starts_with("  Operation: ") {
            let split = l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            cur_monkey.op = Box::new(move |x: i64| -> i64 {
                let v = if let Ok(s) = split[split.len() - 1].parse::<i64>() {
                    s
                } else {
                    x
                };
                match split[split.len() - 2].as_ref() {
                    "+" => x + v,
                    "-" => x - v,
                    "*" => x * v,
                    "/" => x / v,
                    _ => panic!("invalid *_*"),
                }
            });
            continue;
        }

        if l.starts_with("    If true: ") {
            cur_monkey.if_test_true = l.split(" ").last().unwrap().parse::<i64>().unwrap();
            continue;
        }

        if l.starts_with("    If false: ") {
            cur_monkey.if_test_false = l.split(" ").last().unwrap().parse::<i64>().unwrap();
            continue;
        }
    }

    monkeys.push(RefCell::new(cur_monkey));

    // calculate lcm (just product since all prime :P)
    let lcm = monkeys
        .iter()
        .fold(1, |accum, m| accum * m.borrow().test_val);

    for _ in 0..rounds {
        for k in 0..monkeys.len() {
            let mut monkey = monkeys[k].borrow_mut();

            for v in monkey.items.iter() {
                // inspection
                let mut v = (*monkey.op)(*v);
                // mod by lcm
                v %= lcm;
                // test
                if (*monkey.test)(v) {
                    monkeys[monkey.if_test_true as usize]
                        .borrow_mut()
                        .items
                        .push(v);
                } else {
                    monkeys[monkey.if_test_false as usize]
                        .borrow_mut()
                        .items
                        .push(v);
                }
            }
            // increment inspected ct
            monkey.inspected_ct += monkey.items.len() as i64;
            monkey.items.clear();
        }
    }

    let mut cts = monkeys
        .into_iter()
        .map(|x| x.borrow().inspected_ct)
        .collect::<Vec<i64>>();
    cts.sort_by(|a, b| b.cmp(&a));
    println!("{:?}", cts);
    cts[0] * cts[1]
}
