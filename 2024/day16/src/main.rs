use std::collections::{HashMap, HashSet, BinaryHeap};
use std::isize;
use std::ops::Add;
use std::time::Instant;
use std::cmp::Reverse;

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    fn new(x: isize, y: isize) -> Pos {
        Pos { x, y }
    }


    // let straight = Move{rotation: Pos::new(1, 0), cost: 1};
    // let left = Move{rotation: Pos::new(0, 1), cost: 1001};
    // let right = Move{rotation: Pos::new(0, -1), cost: 1001};
    fn rotate(self, other: Pos) -> Pos {
        if other.x > 0 {
            Pos {
                x: self.x * other.x,
                y: self.y * other.x,
            }
        } else {
            Pos {
                x: self.y * other.y,
                y: self.x * other.y,
            }
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

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
struct Reindeer {
    pos: Pos,
    dir: Pos,
}

fn _get_max_pos(map: &HashMap<Pos, char>) -> (isize, isize) {
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;
    for pos in map.keys() {
        if pos.x > max_x {
            max_x = pos.x;
        }
        if pos.y > max_y {
            max_y = pos.y;
        }
    }
    (max_x, max_y)
}

fn _print_map(map: &HashMap<Pos, char>, visited: &HashSet<Pos>) {
    let (max_x, max_y) = _get_max_pos(map);
    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = Pos::new(x, y);
            if visited.contains(&pos) {
                print!("O")
            } else {
                print!("{}", map.get(&pos).unwrap())
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
struct Move {
    rotation: Pos,
    cost: isize
}

fn read_map(input: &str) -> (HashMap<Pos, char>, Reindeer, Pos) {
    let mut map = HashMap::new();
    let mut start = Pos::new(0, 0);
    let mut end = Pos::new(0, 0);
    for (y, row) in input.lines().enumerate() {
        for (x, char) in row.chars().enumerate() {
            let pos = Pos::new(x as isize, y as isize);
            map.insert(pos, char);
            if char == 'S' {
                start = pos;
            } else if  char == 'E' {
                end = pos;
            }
        }
    }
    let reindeer = Reindeer {
        pos: start,
        dir: Pos::new(1, 0), // Facing East
    };
    (map, reindeer, end)
}

fn update_visited(
    visited: &mut HashMap<Reindeer, (isize, Vec<Pos>)>,
    reindeer: &Reindeer,
    points: isize,
    path: &Vec<Pos>
) -> bool {
    if let Some(existing) = visited.get_mut(&reindeer) {
        if points < existing.0 {
            *existing = (points, path.to_vec());
            return true;
        } else if points == existing.0 {
            existing.1.extend(path);
            return true;
        }
    } else {
        visited.insert(*reindeer, (points, path.to_vec()));
        return true;
    }
    false
}

fn check_moves(
    map: &HashMap<Pos, char>,
    start_reindeer: Reindeer,
    start_points: isize,
    visited: &mut HashMap<Reindeer, (isize, Vec<Pos>)>,
    start_path: Vec<Pos>,
) {
    const POSSIBLE_MOVES: [Move; 3] = [
        Move {
            rotation: Pos { x: 1, y: 0 },
            cost: 1,
        },
        Move {
            rotation: Pos { x: 0, y: 1 },
            cost: 1001,
        },
        Move {
            rotation: Pos { x: 0, y: -1 },
            cost: 1001,
        },
    ];

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((start_points, start_reindeer, start_path)));

    while let Some(Reverse((points, current_reindeer, current_path))) = heap.pop() {
        for act in &POSSIBLE_MOVES {
            let mut new_reindeer = current_reindeer;
            new_reindeer.dir = new_reindeer.dir.rotate(act.rotation);
            new_reindeer.pos = new_reindeer.pos + new_reindeer.dir;
            let new_points = points + act.cost;

            match map.get(&new_reindeer.pos) {
                Some('#') => continue,
                Some('E') => {
                    let mut new_path = current_path.clone();
                    new_path.push(new_reindeer.pos);
                    update_visited(visited, &new_reindeer, new_points, &new_path);
                }
                Some('S') => continue,
                Some('.') => {
                    let mut new_path = current_path.clone();
                    new_path.push(new_reindeer.pos);

                    if update_visited(visited, &new_reindeer, new_points, &new_path) {
                        heap.push(Reverse((new_points, new_reindeer, new_path)));
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}



fn solve(input: &str) -> (isize, isize) {
    let (map, reindeer, end) = read_map(input);
    let mut visited = HashMap::new();
    let points = 0;
    let path = vec![reindeer.pos];
    visited.insert(reindeer, (points, path.clone()));

    check_moves(&map, reindeer, points, &mut visited, path);

    let ans_a = visited
        .iter()
        .filter_map(|(&reindeer, points)| if reindeer.pos == end { Some(points.0) } else { None })
        .min()
        .unwrap_or(isize::MAX);
    let ans_b: HashSet<Pos> = visited.iter().filter_map(|(reindeer, (points, path))| {
        if reindeer.pos == end && *points == ans_a {
            Some(path.clone())
        } else {
            None
        }
    }).into_iter().flatten().collect();
    _print_map(&map, &ans_b);
    let ans_b = ans_b.len().try_into().unwrap();

    (ans_a, ans_b)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = solve(input);
    assert_eq!(ans.0, 98416);
    assert_eq!(ans.1, 471);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a1() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert_eq!(ans.0, 7036);
    }

    #[test]
    fn example_a2() {
        let input = include_str!("example_2.txt");
        let ans = solve(input);
        assert_eq!(ans.0, 11048);
    }

    #[test]
    fn example_b1() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert_eq!(ans.1, 45);
    }

    #[test]
    fn example_b2() {
        let input = include_str!("example_2.txt");
        let ans = solve(input);
        assert_eq!(ans.1, 64);
    }
}
