use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("hello.txt").unwrap();
    let reader = BufReader::new(file);

    let mut num_lines: HashMap<(i64, i64), i64> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(" -> ");
        let start: Vec<i64> = parts.next().unwrap().split(",").map(|p| p.parse().unwrap()).collect();
        let end: Vec<i64> = parts.next().unwrap().split(",").map(|p| p.parse().unwrap()).collect();
        let x0 = start[0];
        let y0 = start[1];
        let x1 = end[0];
        let y1 = end[1];

        if x0 == x1 {
            for y in y0.min(y1)..=y0.max(y1) {
                *num_lines.entry((x0, y)).or_default() += 1;
            }
        } else if y0 == y1 {
            for x in x0.min(x1)..=x0.max(x1) {
                *num_lines.entry((x, y0)).or_default() += 1;
            }
        } else {
            let dx = if x1 - x0 > 0 {1} else {-1};
            let dy = if y1 - y0 > 0 {1} else {-1};
            let mut x = x0;
            let mut y = y0;
            *num_lines.entry((x, y)).or_default() += 1;
            while x != x1 {
                x += dx;
                y += dy;
                *num_lines.entry((x, y)).or_default() += 1;
            }
            assert!(y == y1);
        }
    }
    let intersections = num_lines.values().filter(|f| **f >= 2).count();
    println!("{}", intersections)
}
