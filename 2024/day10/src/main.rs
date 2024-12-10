use std::{collections::HashSet, time::Instant};

fn _print_map(map: &[Vec<isize>]) {
    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn check_pos(map: &[Vec<isize>], pos: (usize, usize), i: isize, score: &mut usize) {
    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let x_new = pos.0 as isize + dir.0;
        let y_new = pos.1 as isize + dir.1;
        if x_new >= 0 && x_new < map.len() as isize &&
        y_new >= 0 && y_new < map[0].len() as isize {
            let x_new = x_new as usize;
            let y_new = y_new as usize;
            match map[x_new][y_new] {
                n => {
                    if n == 9 && i == 9 {
                        *score += 1;
                    } else if n == i {
                        check_pos(&map, (x_new, y_new), i + 1, score);
                    }
                }
            }
        }
    }
}

fn check_pos_a(map: &[Vec<isize>], pos: (usize, usize), i: isize, peaks: &mut HashSet<(usize, usize)>) {
    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let x_new = pos.0 as isize + dir.0;
        let y_new = pos.1 as isize + dir.1;
        if x_new >= 0 && x_new < map.len() as isize &&
        y_new >= 0 && y_new < map[0].len() as isize {
            let x_new = x_new as usize;
            let y_new = y_new as usize;
            match map[x_new][y_new] {
                n => {
                    if n == 9 && i == 9 {
                        peaks.insert((x_new, y_new));
                    } else if n == i {
                        println!("checking {} {} {}", x_new, y_new, i);
                        check_pos_a(&map, (x_new, y_new), i + 1, peaks);
                    }
                }
            }
        }
    }
}

fn part_a(input: &str) -> isize {
    let mut map = Vec::new();
    for line in input.lines() {
        let row: Vec<isize> = line.chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect();
        map.push(row);
    }

    let mut paths = 0;
    _print_map(&map);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[x][y] != 0 {
                continue;
            }
            let pos = (x, y);
            let mut peaks = HashSet::new();
            check_pos_a(&map, pos, 1, &mut peaks);
            paths += peaks.len();
            println!("Score {}", paths);
        }
    }
    paths.try_into().unwrap()
}

fn part_b(input: &str) -> isize {
    let mut map = Vec::new();
    for line in input.lines() {
        let row: Vec<isize> = line.chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect();
        map.push(row);
    }

    let mut paths = 0;
    _print_map(&map);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[x][y] != 0 {
                continue;
            }
            let pos = (x, y);
            check_pos(&map, pos, 1, &mut paths);
        }
    }
    paths.try_into().unwrap()
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = part_a(input);
    assert_eq!(ans, 789);

    let ans = part_b(input);
    assert_eq!(ans, 1735);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(input);
        assert_eq!(ans, 36);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = part_b(input);
        assert_eq!(ans, 81);
    }
}
