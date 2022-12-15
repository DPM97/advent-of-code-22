use std::cmp::Ordering;
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
    println!("part 1: {} ({:?})", part_one(lines.clone()), now.elapsed());

    let now = Instant::now();
    println!("part 2: {} ({:?})", part_two(lines), now.elapsed());
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum TokenType {
    Integer,
    Array,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Token {
    value: Option<u32>,
    children: Vec<Token>,
    t: TokenType,
}

fn read_integer(l: &Vec<char>, e: &mut usize) -> Option<u32> {
    let s = *e;
    *e += 1;
    while *e < l.len() && l[*e] >= '0' && l[*e] <= '9' {
        *e += 1;
    }

    if let Ok(int) = l[s..*e].into_iter().collect::<String>().parse::<u32>() {
        Some(int)
    } else {
        *e = s;
        None
    }
}

fn build_structure(chars: &Vec<char>, cur: &mut usize) -> Token {
    let mut children = vec![];

    *cur += 1;

    loop {
        if chars[*cur] == ']' {
            break;
        }

        if chars[*cur] == '[' {
            children.push(build_structure(chars, cur));
        } else if let Some(int) = read_integer(chars, cur) {
            children.push(Token {
                value: Some(int),
                children: vec![],
                t: TokenType::Integer,
            });
        } else {
            *cur += 1;
        }
    }

    *cur += 1;

    Token {
        value: None,
        children,
        t: TokenType::Array,
    }
}

fn validate(t1: &Token, t2: &Token) -> Option<bool> {
    match (t1.t, t2.t) {
        (TokenType::Integer, TokenType::Integer) => {
            match t1.value.unwrap().cmp(&t2.value.unwrap()) {
                Ordering::Greater => Some(false),
                Ordering::Less => Some(true),
                Ordering::Equal => None,
            }
        }
        (TokenType::Array, TokenType::Array) => {
            for (t3, t4) in t1.children.iter().zip(t2.children.iter()) {
                if let Some(r) = validate(t3, t4) {
                    return Some(r);
                }
            }

            match t1.children.len().cmp(&t2.children.len()) {
                Ordering::Greater => Some(false),
                Ordering::Less => Some(true),
                Ordering::Equal => None,
            }
        }
        (TokenType::Integer, TokenType::Array) => validate(
            &Token {
                value: None,
                children: vec![t1.clone()],
                t: TokenType::Array,
            },
            t2,
        ),
        (TokenType::Array, TokenType::Integer) => validate(
            t1,
            &Token {
                value: None,
                children: vec![t2.clone()],
                t: TokenType::Array,
            },
        ),
    }
}

fn sort(t1: &Token, t2: &Token) -> Ordering {
    match (t1.t, t2.t) {
        (TokenType::Integer, TokenType::Integer) => t1.value.unwrap().cmp(&t2.value.unwrap()),
        (TokenType::Array, TokenType::Array) => {
            for (t3, t4) in t1.children.iter().zip(t2.children.iter()) {
                let v = sort(t3, t4);
                if v == Ordering::Greater || v == Ordering::Less {
                    return v;
                }
            }

            match t1.children.len().cmp(&t2.children.len()) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
            }
        }
        (TokenType::Integer, TokenType::Array) => sort(
            &Token {
                value: None,
                children: vec![t1.clone()],
                t: TokenType::Array,
            },
            t2,
        ),
        (TokenType::Array, TokenType::Integer) => sort(
            t1,
            &Token {
                value: None,
                children: vec![t2.clone()],
                t: TokenType::Array,
            },
        ),
    }
}

fn part_one(lines: Vec<String>) -> i32 {
    let mut l = 0;
    let mut cur_pair = 1;
    let mut sum = 0;

    while l < lines.len() {
        let one = &lines[l].chars().collect::<Vec<char>>();
        let two = &lines[l + 1].chars().collect::<Vec<char>>();

        let mut cur = 0;
        let one = build_structure(one, &mut cur);
        cur = 0;
        let two = build_structure(two, &mut cur);

        if validate(&one, &two).unwrap() {
            sum += cur_pair;
        }

        cur_pair += 1;
        l += 3;
    }

    sum
}

fn part_two(lines: Vec<String>) -> i32 {
    let starting_tokens = vec![
        Token {
            value: None,
            children: vec![Token {
                value: Some(2),
                children: vec![],
                t: TokenType::Integer,
            }],
            t: TokenType::Array,
        },
        Token {
            value: None,
            children: vec![Token {
                value: Some(6),
                children: vec![],
                t: TokenType::Integer,
            }],
            t: TokenType::Array,
        },
    ];

    let mut tokens = starting_tokens.clone();

    for l in lines {
        if l == "" {
            continue;
        }
        tokens.push(build_structure(&l.chars().collect::<Vec<char>>(), &mut 0));
    }

    tokens.sort_by(|t1, t2| sort(t1, t2));

    starting_tokens.iter().fold(1, |i, t1| {
        i * (tokens.iter().position(|t2| t1 == t2).unwrap() as i32 + 1)
    })
}
