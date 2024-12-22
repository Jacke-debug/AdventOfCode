use std::collections::HashMap;
use std::iter::Sum;
use std::time::Instant;

struct SecretNumber {
    n: isize,
}

impl SecretNumber {
    fn new(n: isize) -> Self {
        Self { n }
    }
    fn update(&mut self) {
        self.mix(self.n * 64);
        self.prune();
        self.mix(self.n / 32);
        self.prune();
        self.mix(self.n * 2048);
        self.prune();
    }
    fn mix(&mut self, other: isize) {
        self.n ^= other;
    }
    fn prune(&mut self) {
        self.n %= 16_777_216;
    }
}

impl<'a> Sum<&'a SecretNumber> for isize {
    fn sum<I: Iterator<Item = &'a SecretNumber>>(iter: I) -> Self {
        iter.map(|secret| secret.n).sum()
    }
}

fn solve(input: &str, iters: usize) -> (isize, isize) {
    let mut secret_numbers = input
        .lines()
        .map(|l| SecretNumber::new(l.parse::<isize>().unwrap()))
        .collect::<Vec<_>>();
    let mut sequences = HashMap::new();

    for n in secret_numbers.iter_mut() {
        let mut first_match = HashMap::new();
        let mut derivatives = Vec::with_capacity(iters);
        let mut cost = Vec::with_capacity(iters);
        for _ in 0..iters {
            let prev = n.n % 10;
            n.update();
            derivatives.push(n.n % 10 - prev);
            cost.push(n.n % 10);
        }
        for i in 0..derivatives.len() - 3 {
            let sequence = derivatives[i..i + 4].to_vec();
            first_match.entry(sequence).or_insert(cost[i + 3]);
        }

        for (key, value) in first_match {
            sequences
                .entry(key)
                .and_modify(|e| *e += value)
                .or_insert(value);
        }
    }

    (
        secret_numbers.iter().sum(),
        *sequences.values().max().unwrap(),
    )
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = solve(input, 2000);
    assert_eq!(ans.0, 14623556510);
    assert_eq!(ans.1, 1701);
    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let mut secret_number = SecretNumber::new(42);
        let other = 15;
        secret_number.mix(other);
        assert!(secret_number.n == 37);
    }

    #[test]
    fn test_prune() {
        let mut secret_number = SecretNumber::new(100000000);
        secret_number.prune();
        assert!(secret_number.n == 16113920);
    }

    #[test]
    fn example_a1() {
        let input = "123";
        let ans = solve(input, 10);
        println!("ans: {}", ans.0);
        assert!(ans.0 == 59082541);
    }

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(input, 2000);
        assert!(ans.0 == 37327623);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example_b.txt");
        let ans = solve(input, 2000);
        assert_eq!(ans.1, 23);
    }
}
