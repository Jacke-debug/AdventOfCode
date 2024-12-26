use std::time::Instant;

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = vec![vec![' '; rows]; cols];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }
    transposed
}

fn solve(input: &str) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for item in input.split("\r\n\r\n") {
        let mut lines = item.lines();
        let is_lock = lines.next().unwrap().starts_with('.');
        let s: Vec<Vec<char>> = (0..5)
            .map(|_| lines.next().unwrap().chars().collect())
            .collect();
        let d: Vec<usize> = transpose(s)
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .collect();
    
        match is_lock {
            true => locks.push(d),
            false => keys.push(d),
        }
    }
    let mut fitting_keys = 0;
    for lock in locks.iter() {
        fitting_keys += keys.iter().filter(|key| {
            (0..5).all(|i| lock[i] + key[i] <= 5)
        }).count();
    }
    fitting_keys
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = solve(input);
    assert_eq!(ans, 3356);
    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert!(ans == 3);
    }
}
