use core::panic;
use std::{collections::{HashMap, VecDeque, BinaryHeap}, isize, cmp::Reverse};


fn part_a() {
    let input = include_str!("input.txt");
    let mut map = HashMap::new();
    let mut best = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.trim().split("\r\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((y, x), c.to_digit(10).unwrap() as i32);
            best.insert((y, x), i32::MAX);
            max_x = x;
            max_y = y;
        }
    }

    let mut visit = VecDeque::new();
    visit.push_back(((0, 0), 0));
    while let Some(((y, x), cost)) = visit.pop_front() {
        // println!("Visiting: {}, {}, at cost: {}", x, y, cost);
        if cost < best[&(y, x)] {
            best.insert((y, x), cost);
            for (dy, dx) in [(1isize, 0), (0, 1), (-1, 0), (0, -1)] {
                let y = (y as isize) + dy;
                let x = (x as isize) + dx;
                if y >= 0 && x >= 0 && y <=max_y as isize && x <= max_x as isize {
                    visit.push_back(((y as usize, x as usize), cost + map[&(y as usize, x as usize)]));
                }
            }
        }
    }

    println!("lowest cost: {}", best[&(max_x, max_y)]);
}

fn wrap(i: i32) -> i32 {
    let i = i %10;
    if i == 0 {1} else {i}
}

fn main() {
    let input = include_str!("input.txt");
    let mut map = HashMap::new();
    let mut best = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.trim().split("\r\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((y, x), c.to_digit(10).unwrap() as i32);
            best.insert((y, x), i32::MAX);
            max_x = x;
            max_y = y;
        }
    }

    let tile_width = max_x + 1;
    let tile_height = max_y +1;
    for y_tile in 0..5 {
        for x_tile in 0..5 {
            if y_tile == 0 && x_tile == 0 {
                continue;
            }
            for y in 0..=max_y {
                for x in 0..=max_x {
                    if x_tile == 0 {
                        map.insert((y + y_tile * tile_height, x + x_tile * tile_width ), 
                        wrap(map[&(y + (y_tile - 1) * tile_height, x + x_tile * tile_width)]+1 % 10));
                    } else {
                        map.insert((y + y_tile * tile_height, x + x_tile * tile_width ), 
                        wrap(map[&(y + y_tile * tile_height, x + (x_tile - 1) * tile_width)]+1 % 10));
                    }
                    best.insert((y + y_tile * tile_height, x + x_tile * tile_width), i32::MAX);
                }
            }
        }
    }

    //for y in 0..tile_height*5 {
    //    for x in 0..tile_width*5 {
    //        eprint!("{}", map[&(y, x)]);
    //    }
    //    eprintln!()
    //}

    let max_y = 5 * tile_height - 1;
    let max_x = 5 * tile_height - 1;

    let mut visit = BinaryHeap::new();
    visit.push((Reverse(0), (0, 0)));
    while let Some((Reverse(cost), (y, x))) = visit.pop() {
        // println!("Visiting: {}, {}, at cost: {}", x, y, cost);
        if cost < best[&(y, x)] {
            best.insert((y, x), cost);
            for (dy, dx) in [(1isize, 0), (0, 1), (-1, 0), (0, -1)] {
                let y = (y as isize) + dy;
                let x = (x as isize) + dx;
                if y >= 0 && x >= 0 && y <=max_y as isize && x <= max_x as isize {
                    visit.push((Reverse( cost + map[&(y as usize, x as usize)]), ((y as usize, x as usize))));
                }
            }
        }
    }

    println!("lowest cost: {}", best[&(max_y, max_x)]);
}
