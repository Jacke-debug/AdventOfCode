fn is_safe(row: Vec<i64>) -> bool {
    row.windows(2).all(|w| w[0] - w[1] <= 3 && w[0] - w[1] > 0)
        || row.windows(2).all(|w| w[0] - w[1] >= -3 && w[0] - w[1] < 0)
}

fn part_a(input: &str) -> usize {
    let mut safe_levels = 0;
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.

    for line in input.lines() {
        let row: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        if is_safe(row) {
            safe_levels += 1;
        }
    }
    safe_levels
}

fn part_b(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let row: Vec<i64> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (0..row.len()).any(|i| {
                let mut modified_row = row.clone();
                modified_row.remove(i);
                is_safe(modified_row)
            })
        })
        .count()
}

fn main() {
    let input = include_str!("input.txt");
    let ans = part_a(input);
    assert_eq!(ans, 341);

    let ans = part_b(input);
    assert_eq!(ans, 404);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(input);
        assert_eq!(ans, 2);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = part_b(input);
        assert_eq!(ans, 4);
    }
}
