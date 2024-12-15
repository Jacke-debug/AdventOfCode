use std::collections::HashMap;
use std::ops::Add;
use std::time::Instant;

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    fn new(x: isize, y: isize) -> Pos {
        Pos { x, y }
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

fn _print_map(map: &HashMap<Pos, char>) {
    let (max_x, max_y) = _get_max_pos(map);
    for y in 0..=max_y {
        for x in 0..=max_x {
            match map.get(&Pos::new(x, y)) {
                Some(x) => {
                    print!("{}", x)
                }
                None => {
                    unreachable!()
                }
            };
        }
        println!();
    }
    println!();
}

fn read_map(init: &str) -> (HashMap<Pos, char>, Pos) {
    let mut map = HashMap::new();
    let mut start = Pos::new(0, 0);
    for (y, row) in init.lines().enumerate() {
        for (x, char) in row.chars().enumerate() {
            let pos = Pos::new(x as isize, y as isize);
            if char == '#' || char == 'O' {
                map.insert(pos, char);
            } else if char == '@' {
                start = pos;
                map.insert(pos, '.'); // Keep track of position separately
            } else {
                map.insert(pos, '.');
            }
        }
    }
    (map, start)
}

fn read_map_wide(init: &str) -> (HashMap<Pos, char>, Pos) {
    let mut map = HashMap::new();
    let mut start = Pos::new(0, 0);
    for (y, row) in init.lines().enumerate() {
        let mut x = 0;
        for char in row.chars() {
            let pos = Pos::new(x as isize, y as isize);
            if char == '@' {
                start = pos;
            }
            if char == 'O' {
                map.insert(pos, '[');
                map.insert(pos + Pos::new(1, 0), ']');
            } else if char == '#' {
                map.insert(pos, '#');
                map.insert(pos + Pos::new(1, 0), '#');
            } else {
                map.insert(pos, '.');
                map.insert(pos + Pos::new(1, 0), '.');
            }
            x += 2;
        }
    }
    (map, start)
}

fn get_gps_coord_sum(map: &HashMap<Pos, char>) -> isize {
    map.iter()
        .map(|(p, c)| {
            if ['O', '['].contains(c) {
                p.x + p.y * 100
            } else {
                0
            }
        })
        .sum()
}

fn part_a(input: &str) -> isize {
    let (map, instructions) = input.split_once("\r\n\r\n").unwrap();
    let moves: Vec<Pos> = instructions
        .lines()
        .flat_map(|line| {
            line.chars().map(|char| match char {
                '^' => Pos::new(0, -1),
                'v' => Pos::new(0, 1),
                '<' => Pos::new(-1, 0),
                '>' => Pos::new(1, 0),
                _ => unreachable!(),
            })
        })
        .collect();
    let (mut map, mut pos) = read_map(map);

    'moves: for dir in moves {
        let new_pos = pos + dir;
        if let Some('.') = map.get(&new_pos) {
            pos = new_pos;
            continue 'moves;
        }
        let mut move_pos = new_pos;
        loop {
            match map.get(&move_pos) {
                Some('#') => {
                    // Move not possible'
                    continue 'moves;
                }
                Some('.') => {
                    map.insert(move_pos, 'O');
                    map.insert(new_pos, '.');
                    pos = new_pos;
                    continue 'moves;
                }
                Some(_) => {}
                None => {
                    unreachable!(); // Edges have #
                }
            }
            move_pos = move_pos + dir;
        }
    }
    get_gps_coord_sum(&map)
}

fn get_dir(mv: char) -> Pos {
    match mv {
        '^' => Pos::new(0, -1),
        'v' => Pos::new(0, 1),
        '<' => Pos::new(-1, 0),
        '>' => Pos::new(1, 0),
        _ => unreachable!(),
    }
}

fn check_if_box(
    boxes_to_moves: &mut HashMap<Pos, char>,
    map: &mut HashMap<Pos, char>,
    pos_to_check: Vec<Pos>,
    dir: Pos,
) -> bool {
    let mut new_pos_to_check = Vec::new();
    for pos in pos_to_check {
        match map.get(&pos) {
            Some('#') => {
                // Move not possible
                return false;
            }
            Some('[') => {
                boxes_to_moves.insert(pos, '[');
                boxes_to_moves.insert(pos + Pos::new(1, 0), ']');
                if dir.x == 0 {
                    new_pos_to_check.push(pos + dir);
                }
                new_pos_to_check.push(pos + Pos::new(1, 0) + dir);
            }
            Some(']') => {
                boxes_to_moves.insert(pos, ']');
                boxes_to_moves.insert(pos + Pos::new(-1, 0), '[');
                if dir.x == 0 {
                    new_pos_to_check.push(pos + dir);
                }
                new_pos_to_check.push(pos + Pos::new(-1, 0) + dir);
            }
            Some('.') => {
                continue;
            }
            _ => unreachable!(),
        }
    }
    if !new_pos_to_check.is_empty() {
        return check_if_box(boxes_to_moves, map, new_pos_to_check, dir);
    } else {
        for pos in boxes_to_moves.keys() {
            map.insert(*pos, '.');
        }
        for (pos, c) in boxes_to_moves {
            map.insert(*pos + dir, *c);
        }
    }
    true
}

fn part_b(input: &str) -> isize {
    let (map, instructions) = input.split_once("\r\n\r\n").unwrap();
    let moves: Vec<char> = instructions.lines().flat_map(|line| line.chars()).collect();
    let (mut map, mut pos) = read_map_wide(map);
    'moves: for mv in moves {
        let new_pos = pos + get_dir(mv);
        if let Some('.') = map.get(&new_pos) {
            pos = new_pos;
            continue 'moves;
        }
        let mut boxes_to_moves = HashMap::new();
        let pos_to_check = vec![new_pos];
        if check_if_box(&mut boxes_to_moves, &mut map, pos_to_check, get_dir(mv)) {
            pos = new_pos;
        }
    }
    //_print_map(&map);
    get_gps_coord_sum(&map)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = part_a(input);
    assert_eq!(ans, 1517819);

    let ans = part_b(input);
    assert_eq!(ans, 1538862);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(input);
        assert_eq!(ans, 10092);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = part_b(input);
        assert_eq!(ans, 9021);
    }

    #[test]
    fn example_b_small() {
        let input = include_str!("example_small.txt");
        let _ans = part_b(input);
    }
}
