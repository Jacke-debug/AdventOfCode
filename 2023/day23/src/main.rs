use std::collections::{HashMap, VecDeque};

fn part_a(input: &str) -> usize {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let x = x as isize;
            let y = y as isize;
            map.insert((x, y), ch);
        }   
    }

    let directions: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut paths_to_end = Vec::new();

    let mut path0 = Vec::new();
    path0.push((1, 0));
    let mut stack = VecDeque::new();
    stack.push_back(path0);
    

    while let Some(path) = stack.pop_front() {
        let pos = path.last().unwrap();
        'dirs: for dir in directions.iter() {
            let mut new_pos = (pos.0+dir.0, pos.1+dir.1);
            if path.contains(&new_pos) {
                continue 'dirs;
            }
            let mut this_path = path.clone();
            match map.get(&new_pos) {
                Some('#') => continue 'dirs, // step not possible
                Some('.') => {
                    this_path.push(new_pos); // step possible
                }
                Some('>') => {
                    this_path.push(new_pos);
                    new_pos = (new_pos.0+1, new_pos.1);
                    this_path.push(new_pos);
                    if path.contains(&new_pos) {
                        continue 'dirs;
                    }
                }
                Some('v') => {
                    this_path.push(new_pos);
                    new_pos = (new_pos.0, new_pos.1+1);
                    this_path.push(new_pos);
                    if path.contains(&new_pos) {
                        continue 'dirs;
                    }
                }
                Some(_) => unreachable!(),
                None => {
                    paths_to_end.push(this_path);
                    continue 'dirs;
                }
            }
            stack.push_back(this_path);
        }
    }
    // -1 for start
    paths_to_end.iter().map(|path: &Vec<(isize, isize)>| path.len()).max().unwrap()-1
}


fn part_b_optimized(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let x = x as isize;
            let y = y as isize;
            map.insert((x, y), ch);
            x_max = x;
            y_max = y;
        }   
    }
    let mut crossings = HashMap::new();
    let directions: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for y in 0..=y_max {
        for x in 0..=x_max {
            if map.get(&(x, y)).unwrap() == &'#' {
                continue;
            } 
            let mut dirs = Vec::new();
            for dir in directions.iter() {
                let new_pos = (x+dir.0, y+dir.1);
                match map.get(&new_pos) {
                    Some('#') => {
                        continue;
                    }
                    Some(_) => dirs.push(dir),
                    None => continue, // nr_choices?
                }
            }
            if dirs.len() > 2 {
                crossings.insert((x, y), Vec::new());
            }
        }
    }
    let end_pos = (x_max-1, y_max);
    let start_pos = (1, 0);
    crossings.insert(start_pos, Vec::new()); // start
    crossings.insert(end_pos, Vec::new()); // end

    let keys = crossings.clone().into_keys();
    for start in keys {
        'dirs: for dir in directions.iter() {
            let new_pos = (start.0+dir.0, start.1+dir.1);
            match map.get(&new_pos) {
                Some('#') => continue, // step not possible
                Some(_) => {}
                None => continue,
            }
            let mut path = Vec::new();
            path.push(start);
            path.push(new_pos);
            let mut pos = new_pos;
            'next: loop {
                for dir in directions.iter() {
                    let new_pos = (pos.0+dir.0, pos.1+dir.1);
                    if path.contains(&new_pos) {
                        continue; // can't go back
                    }
                    if crossings.contains_key(&new_pos) {
                        let v = crossings.get_mut(&start).unwrap();
                        v.push((new_pos.clone(), path.len()));
                        continue 'dirs;
                    };
                    match map.get(&new_pos) {
                        Some('#') => continue, // step not possible
                        Some(_) => {
                            path.push(new_pos); // step possible
                            pos = new_pos;
                            continue 'next;
                        }
                        None => {
                            continue 'dirs;
                        }
                    }
                }
            }
        }
    };

    let mut stack: VecDeque<(Vec<(isize, isize)>, usize)> = VecDeque::new();
    stack.push_back((vec![start_pos], 0));
    let mut paths_to_end = Vec::new();

    while let Some((path, dist)) = stack.pop_front() {
        let pos = path.last().unwrap();
        let alternatives = crossings.get(pos).unwrap();
        for (new_pos, steps) in alternatives{
            if path.contains(&new_pos) {
                continue; // can't go back
            }
            let mut this_path = path.clone();
            this_path.push(*new_pos);
            let this_dist = steps + dist;
            if new_pos == &end_pos {
                paths_to_end.push((this_path.clone(), this_dist));
                continue;
            }
            stack.push_back((this_path, this_dist));
        }
    }
    return *paths_to_end.iter().map(|(path, length)| length).max().unwrap()
}


fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {}", ans_a); 
    let ans_b = part_b_optimized(input);
    println!("Part B: {}", ans_b); 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), 94);
    }
    #[test]
    fn test_b_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_b_optimized(input), 154);
    }
}