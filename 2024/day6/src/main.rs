use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn generate_map(input: &str) -> (HashMap<(isize, isize), char>, usize, usize, (isize, isize)) {
    let mut map = HashMap::new();
    let mut len_x = 0;
    let mut len_y = 0;
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x as isize, y as isize), char);
            len_x = x;
            if char == '^' {
                start = (x as isize, y as isize);
                map.insert((x as isize, y as isize), 'X');
            }
        }
        len_y = y;
    }
    (map, len_x, len_y, start)
}

fn _print_map(map: &HashMap<(isize, isize), char>, len_x: usize, len_y: usize) {
    for y in 0..len_y + 1 {
        for x in 0..len_x + 1 {
            print!("{}", map.get(&(x as isize, y as isize)).unwrap());
        }
        println!();
    }
    println!();
}

fn rotate_dir(dir: &mut (isize, isize)) {
    match dir {
        (0, -1) => *dir = (1, 0),
        (1, 0) => *dir = (0, 1),
        (0, 1) => *dir = (-1, 0),
        (-1, 0) => *dir = (0, -1),
        _ => unreachable!(),
    }
}

fn solve_map(
    map: &mut HashMap<(isize, isize), char>,
    dir: &(isize, isize),
    pos: &(isize, isize),
) -> usize {
    let mut pos = *pos;
    let mut dir = *dir;
    let mut vistied = HashSet::new();
    loop {
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        match map.get(&next_pos) {
            Some('#') => {
                if vistied.contains(&(pos, dir)) {
                    return 1;
                }
                vistied.insert((pos, dir));
                rotate_dir(&mut dir);
            }
            Some(_) => {
                pos = next_pos;
            }
            None => {
                break;
            }
        }
    }
    0
}

fn part_a(map: &HashMap<(isize, isize), char>, pos: &(isize, isize)) -> usize {
    let mut map = map.clone();
    let mut pos = *pos;
    let mut dir = (0, -1);
    loop {
        match map.get(&(pos.0 + dir.0, pos.1 + dir.1)) {
            Some('#') => {
                rotate_dir(&mut dir);
            }
            Some(_) => {
                map.insert(pos, 'X');
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            }
            None => {
                map.insert(pos, 'X');
                break;
            }
        }
    }
    map.iter().filter(|(_, v)| **v == 'X').count()
}

fn part_b(
    map: &HashMap<(isize, isize), char>,
    pos: &(isize, isize),
    len_x: usize,
    len_y: usize,
) -> usize {
    let dir = (0, -1);
    let mut loops = 0;
    for x in 0..len_x + 1 {
        for y in 0..len_y + 1 {
            let mut new_map = map.clone();
            let c = new_map.insert((x as isize, y as isize), '#');
            if c != Some('#') {
                loops += solve_map(&mut new_map, &dir, pos);
            }
        }
    }
    loops
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let (map, len_x, len_y, pos) = generate_map(input);
    let ans = part_a(&map, &pos);
    assert_eq!(ans, 4778);

    let ans = part_b(&map, &pos, len_x, len_y);
    assert_eq!(ans, 1618);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let (map, _len_x, _len_y, pos) = generate_map(input);
        let ans = part_a(&map, &pos);
        assert_eq!(ans, 41);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let (map, len_x, len_y, pos) = generate_map(input);
        let ans = part_b(&map, &pos, len_x, len_y);
        assert_eq!(ans, 6);
    }
}
