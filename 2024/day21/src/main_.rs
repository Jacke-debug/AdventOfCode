use std::collections::{HashMap, VecDeque};
use std::time::Instant;

pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn find_shortest_paths(keypad: &[[u8; 3]], from: u8, to: u8) -> Vec<Vec<u8>> {
    // find 'from' and 'to' on keypad
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in keypad.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == from {
                start = (x, y);
            }
            if c == to {
                end = (x, y);
            }
        }
    }
    if start == end {
        return vec![vec![b'A']];
    }

    // flood fill keypad to find the shortest distances
    let mut dists = vec![[usize::MAX; 3]; keypad.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        dists[y][x] = steps;
        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < 3
                && ny < keypad.len() as i32
                && keypad[ny as usize][nx as usize] != b' '
                && dists[ny as usize][nx as usize] == usize::MAX
            {
                queue.push_back((nx as usize, ny as usize, steps + 1));
            }
        }
    }

    // backtrack from 'end' back to 'start' and collect all paths
    let mut paths = Vec::new();
    let mut stack = Vec::new();
    stack.push((end.0, end.1, vec![b'A']));
    while let Some((x, y, path)) = stack.pop() {
        if x == start.0 && y == start.1 {
            paths.push(path);
            continue;
        }
        for (i, (dx, dy)) in DIRS.iter().enumerate() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < 3
                && ny < keypad.len() as i32
                && dists[ny as usize][nx as usize] < dists[y][x]
            {
                // do everything in reverse
                let c = match i {
                    0 => b'<',
                    1 => b'^',
                    2 => b'>',
                    3 => b'v',
                    _ => panic!(),
                };
                let mut new_path = vec![c];
                new_path.extend(&path);
                stack.push((nx as usize, ny as usize, new_path));
            }
        }
    }

    paths
}

fn find_shortest_sequence(
    s: &[u8],
    depth: usize,
    highest: bool,
    cursors: &mut Vec<u8>,
    numpad: &[[u8; 3]],
    arrows: &[[u8; 3]],
    cache: &mut HashMap<(Vec<u8>, usize, u8), usize>,
) -> usize {
    let cache_key = (s.to_vec(), depth, cursors[depth]);
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    let mut result = 0;
    for &c in s {
        let paths =
            find_shortest_paths(if highest { numpad } else { arrows }, cursors[depth], c);
        if depth == 0 {
            result += paths.into_iter().map(|l| l.len()).min().unwrap();
        } else {
            result += paths
                .into_iter()
                .map(|p| {
                    find_shortest_sequence(&p, depth - 1, false, cursors, numpad, arrows, cache)
                })
                .min()
                .unwrap();
        }
        cursors[depth] = c;
    }

    cache.insert(cache_key, result);

    result
}

fn solve(input: &str, depth: usize) -> usize {
    let numpad = vec![
        [b'7', b'8', b'9'],
        [b'4', b'5', b'6'],
        [b'1', b'2', b'3'],
        [b' ', b'0', b'A'],
    ];

    let arrows = vec![[b' ', b'^', b'A'], [b'<', b'v', b'>']];

    let lines = input.lines().collect::<Vec<_>>();
    let mut cache = HashMap::new();

    let mut total = 0;
    for l in &lines {
        let mut cursors = vec![b'A'; depth + 1];
        let len = find_shortest_sequence(
            l.as_bytes(),
            depth,
            true,
            &mut cursors,
            &numpad,
            &arrows,
            &mut cache,
        );

        let n = l[0..3].parse::<usize>().unwrap();
        total += n * len;
    }
    total
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans_a = solve(input, 2);
    assert_eq!(ans_a, 157908);
    let ans_b = solve(input, 25);
    assert_eq!(ans_b, 196910339808654);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a1() {
        let input = "029A";
        let ans = solve(input, 2);
        println!("Human answer  : <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
        assert!(ans == 68*29);
    }
    
    #[test]
    fn example_a2() {
        let input = "980A";
        let ans = solve(input, 2);
        assert!(ans == 60*980);
    }

    #[test]
    fn example_a3() {
        let input = "179A";
        let ans = solve(input, 2);
        assert!(ans == 68*179);
    }

    #[test]
    fn example_a4() {
        let input = "456A";
        let ans = solve(input, 2);
        assert!(ans == 64*456);
    }

    #[test]
    fn example_a5() {
        let input = "379A";
        let ans = solve(input, 2);
        println!("Human answer  : <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");
        assert!(ans == 64*379);
    }
}
