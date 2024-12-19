use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn check_if_design_possible(
    towels: &HashSet<&str>,
    design: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&cached_result) = cache.get(design) {
        return cached_result;
    }

    let mut solutions = 0;
    let longest_towel = towels.iter().map(|towel| towel.len()).max().unwrap_or(0);
    for i in 1..=design.len().min(longest_towel) {
        let prefix = &design[..i];
        let postfix = &design[i..];

        if towels.contains(&prefix) {
            solutions += check_if_design_possible(towels, postfix, cache);
        }
    }
    cache.insert(design.to_owned(), solutions);

    solutions
}

fn solve(input: &str) -> (usize, usize) {
    let (towels, designs) = input.split_once("\r\n\r\n").unwrap();
    let towels: HashSet<&str> = towels.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();

    let mut cache = HashMap::new();

    let mut possible_designs = 0;
    let mut configurations = 0;
    for design in designs {
        let solutions = check_if_design_possible(&towels, design, &mut cache);
        configurations += solutions;
        if solutions > 0 {
            possible_designs += 1
        }
    }
    (possible_designs, configurations)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let (ans_a, ans_b) = solve(input);
    assert_eq!(ans_a, 228);
    assert_eq!(ans_b, 584553405070389);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert_eq!(ans.0, 6);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert_eq!(ans.1, 16);
    }
}
