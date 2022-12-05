use std::collections::{HashMap, HashSet};

type Cuboid = ((i64, i64), (i64, i64), (i64, i64));

fn parse_dim(range: &str) -> (i64, i64) {
    let min_lim =i64::MIN;
    let max_lim = i64::MAX;

    let (min, max) = range[2..].split_once("..").unwrap();
    (min_lim.max(min.parse().unwrap()), max_lim.min(max.parse().unwrap()))
}

fn intersection<'a>(mut a: &'a Cuboid, mut b: &'a Cuboid) -> Option<Cuboid> {
    if a.0 .0 > b.0 .0 {
        std::mem::swap(&mut a, &mut b);
    }
    if a.1 .0 > b.1 .0 {
        std::mem::swap(&mut a, &mut b);
    }
    if a.2 .0 > b.2 .0 {
        std::mem::swap(&mut a, &mut b);
    }

    let c = (
        (b.0 .0.max(a.0 .0), b.0 .1.min(a.0 .1)), 
        (b.1 .0.max(a.1 .0), b.1 .1.min(a.1 .1)),
        (b.2 .0.max(a.2 .0), b.2 .1.min(a.2 .1)),
    );

    if !is_valid(&c) {
        return  None;
    }
    Some(c)
}

fn subtract(a: &Cuboid, b: &Cuboid) -> Vec<Cuboid> {
    if let Some(c) = intersection(a, b) {
        let mut v = Vec::new();
        for (i, x) in [(a.0 .0, c.0 .0 -1), (c.0 .0, c.0 .1), (c.0 .1 + 1, a.0 .1)].iter().enumerate() {
            for (j, y) in [(a.1 .0, c.1 .0 -1), (c.1 .0, c.1 .1), (c.1 .1 + 1, a.1 .1)].iter().enumerate() {
                for (k, z) in [(a.2 .0, c.2 .0 -1), (c.2 .0, c.2 .1), (c.2 .1 + 1, a.2 .1)].iter().enumerate() {
                    if i == 1 && j == 1 && k == 1 {
                        continue;
                    }
                    let cuboid = (*x, *y, *z);
                    if is_valid(&cuboid) {
                        v.push(cuboid)
                    }
                }
            }
        }
        v
    } else {
        vec![*a]
    }
}

fn get_volume(a: &Cuboid) -> usize {
    let v = (a.0 .1 - a.0 .0 +1) * (a.1 .1 -a.1 .0 +1) *(a.2 .1 - a.2 .0 +1);
    v as usize
} 

fn is_valid(a: &Cuboid) -> bool {
    a.0 .1 - a.0 .0 >= 0 && a.1 .1 - a.1 .0 >= 0 && a.2 .1 - a.2 .0 >= 0
}

fn main() {
    let input = include_str!("input.txt");
    let mut commands = Vec::new();

    for line in input.trim().split("\r\n") {
        let (state, cuboid)  = line.split_once(" ").unwrap();
        
        let mut dims = cuboid.split(",");
        let (x_min, x_max) = parse_dim(dims.next().unwrap());
        let (y_min, y_max) = parse_dim(dims.next().unwrap());
        let (z_min, z_max) = parse_dim(dims.next().unwrap());

        let cuboid = ((x_min, x_max), (y_min, y_max), (z_min, z_max));
        if state == "on" {
            commands.push((cuboid, true));
        } else {
            commands.push((cuboid, false));
        }
    }
    
    let mut grid = Vec::new();
    let mut count = 0;
    for (new_cuboid, state) in &commands {
        count += 1;
        println!("{}", count);
        let mut next_grid = Vec::new();
        for cuboid in grid {
            for cuboid in subtract(&cuboid, new_cuboid) {
                next_grid.push(cuboid);
            }
        }
        if *state {
            next_grid.push(*new_cuboid);
        }
        grid = next_grid;
    }
    
    let code: usize = grid.iter().map(|c| get_volume(&c)).sum();
    println!("code: {}", code)
}
