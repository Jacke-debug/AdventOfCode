use std::time::Instant;
use std::collections::HashSet;

fn _print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn generate_map(input: &str) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        map.push(row);
    }
    map
}

fn is_outside(map: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
    x < 0 || x >= map.len() as isize || y < 0 || y >= map[0].len() as isize
}

fn solve(input: &str, part_b: bool) -> isize {
    let map = generate_map(input);
    let mut frequencies: HashSet<char> = map.clone().into_iter().flatten().collect();
    frequencies.remove(&'.');

    let mut antinodes = HashSet::new();
    for frequency in frequencies {
        let coords: Vec<(isize, isize)> = map.iter()
            .enumerate()
            .flat_map(|(row, vec)| {
                vec.iter()
                    .enumerate()
                    .filter_map(move |(col, &ch)| if ch == frequency { Some((row as isize, col as isize)) } else { None })
            }).collect();
        for p1 in &coords {
            for p2 in &coords {
                if p1 == p2 {
                    continue;
                }
                let dx = p1.1 - p2.1;
                let dy = p1.0 - p2.0;
                let len = if part_b { map.len() } else { 1 };
                if part_b {
                    antinodes.insert(*p1);
                    antinodes.insert(*p2);
                }
                for i in 1..=len as isize{
                    let antinode = (p1.0 + i*dy, p1.1 + i*dx);
                    if is_outside(&map, antinode.0, antinode.1) {
                        continue;
                    }
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len().try_into().unwrap()
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = solve(input, false);
    assert_eq!(ans, 344);

    let ans = solve(input, true);
    assert_eq!(ans, 1182);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(&input, false);
        assert_eq!(ans, 14);
    }
    
    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = solve(&input, true);
        assert_eq!(ans, 34);
    }
}
