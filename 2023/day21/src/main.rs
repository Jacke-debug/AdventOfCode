use std::{collections::{HashMap, VecDeque, HashSet}, os::windows::raw::HANDLE};


fn part_a(input: &str) -> (usize, isize) {
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    let mut reachable: HashSet<(isize, isize)> = HashSet::new();
    let mut ans_a = 0;
    let mut ans_b = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let x = x as isize;
            let y = y as isize;
            map.insert((x, y), ch);
            if ch == 'S' {
                reachable.insert((x, y));
            }
        }
    }

    
    let dirs: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for step in 0..64 {
        let mut new_reacable = HashSet::new();
        for pos in reachable.iter() {
            for dir in dirs.iter() {
                let new_pos = (pos.0+dir.0, pos.1+dir.1);
                match map.get(&new_pos) {
                    Some('#') => {continue;}
                    Some(_) =>  {
                        new_reacable.insert(new_pos);
                    }
                    None => {continue;}
                }
            }
        }
        reachable = new_reacable;
    }

    ans_a = reachable.iter().count();
    return (ans_a, ans_b)
}

fn main() {
    let input = include_str!("input.txt");
    let (ans_a, ans_b)= part_a(input);
    println!("Part A: {}", ans_a);
    println!("Part B: {}", ans_b);
    // 807069600
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), (32000000, 0));
    }
}