use std::collections::HashMap;
use std::time::Instant;

fn _print_path_and_map(map: &[Vec<char>], path: &HashMap<(usize, usize), usize>) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if path.contains_key(&(x, y)) {
                print!("{}", path.get(&(x, y)).unwrap());
            } else {
                print!("{}{}", c, c);
            }
        }
        println!()
    }
}

fn generate_path(map: &[Vec<char>]) -> HashMap<(usize, usize), usize> {
    let start = map
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .position(|&c| c == 'S')
                .map(|col_idx| (col_idx, row_idx))
        })
        .unwrap();

    let mut path = HashMap::new();
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut queue = vec![(start, 0)];
    while let Some((pos, steps)) = queue.pop() {
        if path.contains_key(&pos) {
            continue;
        }
        path.insert(pos, steps);
        for (dy, dx) in dirs {
            let new_pos = (pos.0 as isize + dx, pos.1 as isize + dy);
            if new_pos.0 < 0
                || new_pos.0 >= map.len() as isize
                || new_pos.1 < 0
                || new_pos.1 >= map[0].len() as isize
            {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            match map[new_pos.1][new_pos.0] {
                '#' => continue,
                'E' => {
                    path.insert(new_pos, steps + 1);
                    break;
                }
                'S' => continue,
                '.' => {
                    queue.push((new_pos, steps + 1));
                }
                _ => {}
            }
        }
    }
    path
}

fn straight_distance(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn solve(input: &str, min_saved: usize, max_disable: usize) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let path = generate_path(&map);
    //_print_path_and_map(&map, &path);
    let mut short_cuts = HashMap::new();
    for (p1, p2) in path
        .keys()
        .flat_map(|p1| path.keys().map(move |p2| (p1, p2)))
    {
        if path.get(p1).unwrap().abs_diff(*path.get(p2).unwrap()) <= straight_distance(p1, p2) {
            continue;
        }

        if straight_distance(p1, p2) <= max_disable {
            let diff = path.get(p1).unwrap().abs_diff(*path.get(p2).unwrap());
            let saved = diff - straight_distance(p1, p2);
            if saved >= min_saved {
                *short_cuts.entry(saved).or_insert(0) += 1;
            }
        }
    }
    short_cuts.values_mut().for_each(|v| *v /= 2);
    short_cuts.values().sum()
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans_a = solve(input, 100, 2);
    assert_eq!(ans_a, 1393);
    let ans_b = solve(input, 100, 20);
    assert_eq!(ans_b, 990096);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(input, 0, 2);
        let expected = 14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1;
        assert!(ans == expected);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = solve(input, 50, 20);
        let expected = 32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3;
        assert_eq!(ans, expected);
    }
}
