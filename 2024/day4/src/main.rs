use std::collections::HashMap;

fn generate_map(input: &str) -> (HashMap<(i64, i64), char>, usize, usize) {
    let mut map = HashMap::new();
    let mut len_x = 0;
    let mut len_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x as i64, y as i64), char);
            //print!("{}", map.get(&(x as i64, y as i64)).unwrap());
            len_x = x;
        }
        len_y = y;
        //println!()
    }
    (map, len_x, len_y)
}

fn part_a(input: &str) -> usize {
    let (map, len_x, len_y) = generate_map(input);
    let mut ans = 0;
    let xmas = ['X', 'M', 'A', 'S'];
    let dirs: Vec<(i64, i64)> = [-1, 0, 1]
        .iter()
        .flat_map(|&x| [-1, 0, 1].iter().map(move |&y| (x, y)))
        .collect();

    for x in 0..=len_x {
        for y in 0..=len_y {
            'dirs: for dir in dirs.clone() {
                for (index, char) in xmas.iter().enumerate() {
                    let index = index as i64;
                    let coord = (x as i64 + dir.0 * index, y as i64 + dir.1 * index);
                    if map.get(&coord) != Some(char) {
                        continue 'dirs;
                    }
                }
                ans += 1;
            }
        }
    }
    ans
}

fn part_b(input: &str) -> usize {
    // Basically all A's surrounded by 2M's and 2S's that are not on opposite corners
    let (map, len_x, len_y) = generate_map(input);
    let mut ans = 0;
    let corners: Vec<(i64, i64)> = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for x in 0..=len_x {
        'next: for y in 0..=len_y {
            if map.get(&(x as i64, y as i64)) != Some(&'A') {
                continue;
            }
            let mut ms = Vec::with_capacity(2);
            for &dir in &corners {
                let coord = (x as i64 + dir.0, y as i64 + dir.1);
                match map.get(&coord) {
                    Some('S') => ms.push(dir),
                    Some('M') => continue,
                    Some(_) => continue 'next,
                    None => continue 'next,
                }
            }
            if ms.len() == 2 && ms[0] != (-ms[1].0, -ms[1].1) {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    let input = include_str!("input.txt");
    let ans = part_a(input);
    assert_eq!(ans, 2507);

    let ans = part_b(input);
    assert_eq!(ans, 1969);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(input);
        assert_eq!(ans, 18);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = part_b(input);
        assert_eq!(ans, 9);
    }
}
