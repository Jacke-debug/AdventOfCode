use std::collections::HashMap;

fn part_a(input: &str) -> usize {
    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<i64> = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();
        v1.push(a.trim().parse().unwrap());
        v2.push(b.trim().parse().unwrap());
    }
    v1.sort();
    v2.sort();

    v1.into_iter()
        .zip(v2)
        .map(|(a, b)| (b - a).abs() as usize)
        .sum::<usize>()
}

fn part_b(input: &str) -> usize {
    let mut v1: Vec<usize> = Vec::new();
    let mut v2: HashMap<usize, usize> = HashMap::new(); //<i64> = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();

        v1.push(a.trim().parse().unwrap());
        v2.entry(b.trim().parse().unwrap())
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }
    v1.into_iter()
        .map(|x| if let Some(y) = v2.get(&x) { x * y } else { 0 })
        .sum()
}

fn main() {
    let input = include_str!("input_a.txt");
    let ans = part_a(input);
    assert_eq!(ans, 3714264);

    let ans = part_b(input);
    assert_eq!(ans, 18805872);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        let ans = part_a(input);
        assert_eq!(ans, 11);

        let ans = part_b(input);
        assert_eq!(ans, 31);
    }
}
