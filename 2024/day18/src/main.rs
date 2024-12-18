use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;
use std::ops::Add;
use std::time::Instant;

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    fn new(x: isize, y: isize) -> Pos {
        Pos { x, y }
    }
    fn from_str(str: (&str, &str)) -> Pos {
        Pos {
            x: str.0.parse().unwrap(),
            y: str.1.parse().unwrap(),
        }
    }
}
impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn _print_map(map: &HashMap<Pos, char>, size: (isize, isize), path: &BTreeSet<Pos>) {
    for y in 0..=size.1 {
        for x in 0..=size.0 {
            let pos = Pos::new(x, y);
            if path.contains(&pos) {
                print!("O");
            } else {
                print!("{}", map.get(&pos).unwrap());
            }
        }
        println!();
    }
    println!();
}

fn find_path(memory: &HashMap<Pos, char>) -> Option<BTreeSet<Pos>> {
    const POSSIBLE_MOVES: [Pos; 4] = [
        Pos { x: 1, y: 0 },
        Pos { x: -1, y: 0 },
        Pos { x: 0, y: 1 },
        Pos { x: 0, y: -1 },
    ];

    let start = Pos::new(0, 0);
    let mut queue = VecDeque::new();
    let mut visited = BTreeSet::new();

    queue.push_back((start, BTreeSet::from([start])));

    while let Some((pos, path)) = queue.pop_front() {
        for act in &POSSIBLE_MOVES {
            let new_pos = pos + *act;
            if path.contains(&new_pos) || visited.contains(&new_pos) {
                continue;
            }

            match memory.get(&new_pos) {
                Some('#') => continue,
                Some('.') => {
                    let mut new_path = path.clone();
                    new_path.insert(new_pos);
                    queue.push_back((new_pos, new_path));
                    visited.insert(new_pos);
                }
                Some('E') => {
                    let mut new_path = path.clone();
                    new_path.insert(new_pos);
                    return Some(new_path);
                }
                _ => continue,
            }
        }
    }

    None
}

fn part_a(input: &str, size: (isize, isize), len: usize) -> usize {
    let mut corrupted: Vec<Pos> = input
        .lines()
        .map(|l| Pos::from_str(l.split_once(",").unwrap()))
        .collect();
    corrupted.truncate(len);
    let mut memory = HashMap::new();
    for y in 0..=size.1 {
        for x in 0..=size.0 {
            if corrupted.iter().any(|p| p.x == x && p.y == y) {
                memory.insert(Pos::new(x, y), '#');
            } else {
                memory.insert(Pos::new(x, y), '.');
            }
        }
    }
    memory.insert(Pos::new(size.0, size.1), 'E');

    let solutions = find_path(&memory);
    let best_path = solutions.unwrap();
    best_path.len() - 1 // Nr of steps is path len minus one
}

fn part_b(input: &str, size: (isize, isize), len: usize) -> Pos {
    let corrupted: Vec<Pos> = input
        .lines()
        .map(|l| Pos::from_str(l.split_once(",").unwrap()))
        .collect();
    let mut count = len;
    let start = &corrupted[0..count];
    let mut memory = HashMap::new();
    for y in 0..=size.1 {
        for x in 0..=size.0 {
            if start.iter().any(|p| p.x == x && p.y == y) {
                memory.insert(Pos::new(x, y), '#');
            } else {
                memory.insert(Pos::new(x, y), '.');
            }
        }
    }
    memory.insert(Pos::new(size.0, size.1), 'E');

    let mut last_piece = Pos::new(0, 0);
    while let Some(_solution) = find_path(&memory) {
        last_piece = *corrupted.get(count).unwrap();
        memory.insert(last_piece, '#');
        count += 1
    }
    last_piece
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = part_a(input, (70, 70), 1024);
    assert_eq!(ans, 310);
    let ans = part_b(input, (70, 70), 2048); // Assume at lest 2kB are needed
    assert_eq!(ans, Pos::new(16, 46));

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(input, (6, 6), 12);
        assert_eq!(ans, 22);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = part_b(input, (6, 6), 12);
        assert_eq!(ans, Pos::new(6, 1));
    }
}
