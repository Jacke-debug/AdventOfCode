use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    let mut paper: HashSet<(i64, i64)> = HashSet::new();
    let mut count = 0;
    for line in input.trim().split("\n") {
        let line = line.replace("\r", "");

        if line.is_empty() {
            println!("is empty");
        } else if line.starts_with("fold along ") {
            count += 1;
            let (_, instruction) = line.split_once("fold along ").unwrap();
            let (axis, split) = instruction.split_once('=').unwrap();
            let split: i64 = split.parse().unwrap();
            match axis {
                "x" => {
                    let mut new_points = HashSet::new();
                    for (y, x) in paper.iter() {
                        if *x == split {
                            continue;
                        }

                        if *x < split {
                            new_points.insert((*y, *x));
                        } else {
                            new_points.insert((*y, split - (*x - split)));
                        }
                    }
                    paper = new_points;
                }
                "y" => {
                    let mut new_points = HashSet::new();
                    for (y, x) in paper.iter() {
                        if *y == split {
                            continue;
                        }

                        if *y < split {
                            new_points.insert((*y, *x));
                        } else {
                            new_points.insert((split - (*y - split), *x));
                        }
                    }
                    paper = new_points;
                }
                _ => {
                    panic!();
                }
            }
            if count == 1 {
                println!("{:?}", paper.len());
            }
        } else {
            let (x, y) = line.split_once(",").unwrap();
            paper.insert((y.parse().unwrap(), x.parse().unwrap()));
        }
    }
    

    let mut message = String::new();
    let min_x = paper.iter().map(|(_, x)| *x).min().unwrap();
    let max_x = paper.iter().map(|(_, x)| *x).max().unwrap();
    let min_y = paper.iter().map(|(y, _)| *y).min().unwrap();
    let max_y = paper.iter().map(|(y, _)| *y).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if paper.contains(&(y, x)) {
                message.push('#');
            } else {
                message.push('.');
            }
        }
        message.push('\n');
    }

    println!("{:?}", paper.len());
    println!("{}", message);
}


