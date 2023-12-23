use std::{collections::{HashMap, HashSet}};


fn parse_map(input: &str) -> HashMap<(isize, isize), char> {
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let x = x as isize;
            let y = y as isize;
            map.insert((x, y), ch);
        }
    }
    return map;
}

fn part_a(input: &str, nr_steps: isize) -> isize {
    let map: HashMap<(isize, isize), char> = parse_map(input);
    let mut position = (0, 0);
    for (key, val) in map.iter() {
        if *val == 'S' {
            position = *key;
            break;
        }
    }
    walk_map(&map, nr_steps, position)
}

fn walk_map(map: & HashMap<(isize, isize), char>, nr_steps: isize, position: (isize, isize)) -> isize{
    let mut reachable: HashSet<(isize, isize)> = HashSet::new();
    reachable.insert(position);
    let dirs: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for step in 0..nr_steps {
        let mut new_reacable = HashSet::new();
        for pos in reachable.iter() {
            for dir in dirs.iter() {
                let new_pos = (pos.0+dir.0, pos.1+dir.1);
                match map.get(&new_pos) {
                    Some('#') => {continue;}
                    Some(_) =>  {
                        new_reacable.insert(new_pos);
                    }
                    None => {continue;}
                }
            }
        }
        reachable = new_reacable;
    };
    return reachable.len() as isize
}

fn part_b(input: &str, nr_steps: isize) -> isize {
    let map = parse_map(input);
    let mut expanded_map = HashMap::new();
    let y_max = input.lines().count() as isize;
    let x_max = input.lines().next().unwrap().chars().count() as isize;

    for (pos, plot) in map.iter() {
        match plot {
            c => {
                let mut c = c;
                if c == &'S' {
                    c = &'.';
                }
                for row in 0..5 {
                    for col in 0..5 {
                        expanded_map.insert(
                            (pos.0 + (x_max*row), pos.1 +(y_max*col)), *c
                        );
                    };
                }
            }
        }
    }

    println!("{}, {}", x_max, y_max);
    let position = (x_max*5/2, y_max*5/2);
    let b0 = walk_map(&expanded_map, x_max/2, position) as f64;
    let b1 = walk_map(&expanded_map, x_max/2+x_max, position) as f64;
    let b2 = walk_map(&expanded_map, x_max/2+2*x_max, position) as f64;
    
    let n = nr_steps/x_max;

    let det_a = -2.0;
    let det_a0 = -b0 + 2.0*b1 - b2;
    let det_a1 = 3.0*b0 - 4.0*b1 + b2;
    let det_a2 = -2.0*b0;
    let x0 = (det_a0 / det_a) as isize;
    let x1 = (det_a1 / det_a) as isize;
    let x2 = (det_a2 / det_a) as isize;
    println!("{} {} {}", b0, b1, b2);
    let ans_b = x0 * n * n + x1 * n + x2;

    return  ans_b as isize
}
fn main() {
    let input = include_str!("input.txt");
    let ans_a= part_a(input, 64);
    println!("Part A: {}", ans_a);
    let ans_b = part_b(input, 26501365);
    println!("Part B: {}", ans_b); 
    //      608152828731262
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input, 6), 16);
    }
}