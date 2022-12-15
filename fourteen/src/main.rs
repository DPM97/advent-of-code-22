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
    println!("part 2: {} ({:?})", part_two(&lines), now.elapsed());
}

fn part_one(lines: &Vec<String>) -> u32 {
    let lines = lines
        .into_iter()
        .map(|l| l.split(" -> ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let [x_min, x_max, y_min, y_max] = lines.iter().fold(
        [500, 500, 0, 0],
        |[mut x_min, mut x_max, mut y_min, mut y_max], vectors| {
            for &v in vectors {
                let v = v
                    .split(",")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let [cur_x, cur_y] = [v[0], v[1]];

                x_min = std::cmp::min(x_min, cur_x);
                x_max = std::cmp::max(x_max, cur_x);
                y_min = std::cmp::min(y_min, cur_y);
                y_max = std::cmp::max(y_max, cur_y);
            }
            [x_min, x_max, y_min, y_max]
        },
    );

    let mut grid = vec![vec!['.'; (x_max - x_min + 1) as usize]; (y_max - y_min + 1) as usize];
    grid[0][(500 - x_min) as usize] = '+';

    lines.iter().for_each(|l| {
        l.windows(2).for_each(|v| {
            let [start, end] = [
                &v[0].split(",").collect::<Vec<&str>>(),
                &v[1].split(",").collect::<Vec<&str>>(),
            ];

            let [x1, x2] = [
                start[0].parse::<u32>().unwrap(),
                end[0].parse::<u32>().unwrap(),
            ];
            let [y1, y2] = [
                start[1].parse::<u32>().unwrap(),
                end[1].parse::<u32>().unwrap(),
            ];

            let [vx_min, vx_max, vy_min, vy_max] = [
                std::cmp::min(x1, x2),
                std::cmp::max(x1, x2),
                std::cmp::min(y1, y2),
                std::cmp::max(y1, y2),
            ];

            for x in vx_min..=vx_max {
                for y in vy_min..=vy_max {
                    grid[(y - y_min) as usize][(x - x_min) as usize] = '#';
                }
            }
        })
    });

    let mut grain_count = 0;
    loop {
        let [mut grain_x, mut grain_y] = [(500 - x_min) as usize, 0 as usize];
        'inner: loop {
            // if fell to the bottom!
            if grain_y == (y_max - y_min) as usize {
                return grain_count;
            }

            let get_below = || grid[grain_y + 1][grain_x];

            let get_bottom_left = || {
                if grain_x >= 1 {
                    Some(grid[grain_y + 1][grain_x - 1])
                } else {
                    None
                }
            };

            let get_bottom_right = || {
                if grain_x < (x_max - x_min) as usize {
                    Some(grid[grain_y + 1][grain_x + 1])
                } else {
                    None
                }
            };

            if get_below() == '.' {
                grain_y += 1;
                continue 'inner;
            }

            if let Some(left) = get_bottom_left() {
                if left == '.' {
                    grain_x -= 1;
                    grain_y += 1;
                    continue 'inner;
                }
            } else {
                return grain_count;
            }

            if let Some(right) = get_bottom_right() {
                if right == '.' {
                    grain_x += 1;
                    grain_y += 1;
                    continue 'inner;
                }
            } else {
                return grain_count;
            }

            break 'inner;
        }
        grid[grain_y][grain_x] = 'O';
        grain_count += 1;
    }
}

fn part_two(lines: &Vec<String>) -> u32 {
    let lines = lines
        .into_iter()
        .map(|l| l.split(" -> ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let [y_min, mut y_max] = lines
        .iter()
        .fold([0, 0], |[mut y_min, mut y_max], vectors| {
            for &v in vectors {
                let v = v
                    .split(",")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let cur_y = v[1];

                y_min = std::cmp::min(y_min, cur_y);
                y_max = std::cmp::max(y_max, cur_y);
            }
            [y_min, y_max]
        });

    y_max += 2;

    /*
        on each side of the sand source, we could technically have y_height - 1
        bloom on each side (e.g., in the case sand is able to fall to the right or left
        for each y-value until the bottom)

        this means that we have to set the x bounds
        x_min_new = min{src_x - (y_height - 1), x_min}
        x_max_new = max{src_x + (y_height - 1), x_min}

        # Note: we can ignore the -1 as we don't want any oob errors when checking
                bottom-lefts and bottom-rights.

        after this has been computed, we know that grains can't fall out of bounds,
        and can compute until the 3 locations under the source are sand grains
    */

    let [mut src_x, src_y] = [500, 0];

    let x_min = src_x - (y_max - y_min);
    let x_max = src_x + (y_max - y_min);

    let mut grid = vec![vec!['.'; (x_max - x_min + 1) as usize]; (y_max - y_min + 1) as usize];

    for i in 0..=(x_max - x_min) {
        grid[(y_max - y_min) as usize][i as usize] = '#';
    }

    lines.iter().for_each(|l| {
        l.windows(2).for_each(|v| {
            let [start, end] = [
                &v[0].split(",").collect::<Vec<&str>>(),
                &v[1].split(",").collect::<Vec<&str>>(),
            ];

            let [x1, x2] = [
                start[0].parse::<u32>().unwrap(),
                end[0].parse::<u32>().unwrap(),
            ];
            let [y1, y2] = [
                start[1].parse::<u32>().unwrap(),
                end[1].parse::<u32>().unwrap(),
            ];

            let [vx_min, vx_max, vy_min, vy_max] = [
                std::cmp::min(x1, x2),
                std::cmp::max(x1, x2),
                std::cmp::min(y1, y2),
                std::cmp::max(y1, y2),
            ];

            for x in vx_min..=vx_max {
                for y in vy_min..=vy_max {
                    grid[(y - y_min) as usize][(x - x_min) as usize] = '#';
                }
            }
        })
    });

    src_x = (x_max - x_min) / 2;

    let mut grain_count = 0;
    loop {
        let [mut grain_x, mut grain_y] = [src_x as usize, src_y as usize];
        'inner: loop {
            // if fell to the bottom!
            if grain_y == (y_max - y_min) as usize {
                return grain_count;
            }

            let [below, bottom_left, bottom_right] = [
                grid[grain_y + 1][grain_x],
                grid[grain_y + 1][grain_x - 1],
                grid[grain_y + 1][grain_x + 1],
            ];

            match [below, bottom_left, bottom_right] {
                ['O', 'O', 'O'] => {
                    if grain_x == (500 - x_min as usize) && grain_y == 0 {
                        return grain_count + 1;
                    }
                }
                _ => {}
            }

            if below == '.' {
                grain_y += 1;
                continue 'inner;
            }

            if bottom_left == '.' {
                grain_x -= 1;
                grain_y += 1;
                continue 'inner;
            }

            if bottom_right == '.' {
                grain_x += 1;
                grain_y += 1;
                continue 'inner;
            }

            break 'inner;
        }
        grid[grain_y][grain_x] = 'O';
        grain_count += 1;
    }
}
