use std::{collections::HashMap, time::Instant};

fn solve(input: &str, iters: usize) -> usize {
    let mut stone_map: HashMap<usize, usize> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .fold(HashMap::new(), |mut map, stone| {
            *map.entry(stone).or_insert(0) += 1;
            map
        });
    for _ in 0..iters {
        let mut new_map = HashMap::new();
        for (stone, cnt) in stone_map.iter_mut() {
            match *stone {
                0 => {
                    *new_map.entry(1).or_insert(0) += *cnt;
                }
                n => {
                    let n_digits = n.to_string().len();
                    if n_digits % 2 == 0 {
                        let a: usize = n.to_string()[0..n_digits / 2].parse().unwrap();
                        let b: usize = n.to_string()[n_digits / 2..n_digits].parse().unwrap();
                        *new_map.entry(a).or_insert(0) += *cnt;
                        *new_map.entry(b).or_insert(0) += *cnt;
                    } else {
                        *new_map.entry(*stone * 2024).or_insert(0) += *cnt;
                    }
                }
            }
        }
        stone_map = new_map;
    }
    stone_map.values().sum()
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = solve(input, 25);
    assert_eq!(ans, 183248);

    let ans = solve(input, 75);
    assert_eq!(ans, 218811774248729);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a_1() {
        let input = include_str!("example.txt");
        let ans = solve(input, 6);
        assert_eq!(ans, 22);
    }

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(input, 25);
        assert_eq!(ans, 55312);
    }
}
