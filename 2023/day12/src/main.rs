use core::panic;
use std::collections::{VecDeque, HashMap};

fn solve(map: & VecDeque<char>, groups: &mut VecDeque<usize>, nr_arrangements: &mut usize){
    let group = match groups.pop_front() {
        Some(x) => {x},
        None => {
            panic!();
        },
    };
    'place_next: for place_index in 0..=map.len() as usize {
        for idx in 0..place_index{
            match map.get(idx) {
                Some('#') => {return}, // # can't be skipped
                Some('.') | Some('?') => {}, 
                _ => {panic!()}
            }
        }
        for idx in place_index..place_index+group {
            match map.get(idx) {
                Some('.') => {continue 'place_next}, // not possible there
                Some('#') | Some('?') => continue,
                Some(_) => panic!(),
                _ => {return},
            }
        }
        if groups.is_empty() {
            for idx in place_index+group..=map.len() {
                match map.get(idx) {
                    Some('#') => continue 'place_next, // solution not valid
                    _ => {}
                }
            }
            *nr_arrangements += 1;
            continue 'place_next;
        }
        match map.get(place_index+group) {
            Some('#') => {
                continue 'place_next;
             } 
            Some(_) => {
                let mut new_map = map.clone();
                let mut new_groups = groups.clone();
                new_map.drain(..place_index+group+1);
                solve(&mut new_map, &mut new_groups, nr_arrangements);
            }
            None => {
                return;
            }
        }
    }

}

const OPERATIONAL: u8 = b'.';
const DAMAGNED: u8 = b'#';
const UNKOWN: u8 = b'?';

fn arragments(data: &[u8], groups: &[u32])-> u64 {
    let mut cache = HashMap::new();
    let ans = dfs(&mut cache, data, groups, 0, 0, 0);
    ans
}

fn dfs(
    cache: &mut HashMap<(usize, usize, u32), u64>,
    data: &[u8],
    groups: &[u32],
    from: usize,
    group: usize,
    size: u32,
) -> u64 {
    if from >= data.len() {
        if group >= groups.len() {
            return 1;
        }

        if group == groups.len() - 1 && groups[group]==size {
            return 1;
        }

        return 0;
    }

    match data[from] {
        OPERATIONAL => {
            if size == 0 {
                return dfs(cache, data, groups, from+1, group, size);
            }

            if group >= groups.len() || size != groups[group] {
                return 0;
            }
            return dfs(cache, data, groups, from+1, group+1, 0);
        }
        DAMAGNED => {
            if group >= groups.len() || size + 1 > groups[group]{
                return 0;
            }

            return dfs(cache, data, groups, from + 1 , group, size+1);
        }
        UNKOWN => {
            if let Some(answer) = cache.get(&(from, group, size)).copied() {
                return answer;
            }

            let mut ways = 0;
            if size == 0 {
                ways += dfs(cache, data, groups, from +1, group, size);
            }

            if group < groups.len() && size < groups[group] {
                ways += dfs(cache, data, groups, from +1, group, size +1);
            }

            if group < groups.len() && size == groups[group] {
                ways += dfs(cache, data, groups, from +1, group +1, 0);
            }
            cache.insert((from, group, size), ways);
            return ways;
        }
        _ => unreachable!()
        
    }
}

fn part_a(input: &str, nr_folds: usize) -> u64 {
    input.lines()
        .map(|l| l.rsplit_once(' ').unwrap())
        .map(|(l, r)| {
            (
                l.trim(),
                r.trim()
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(data, groups)| {
            let mut expanded_data = String::new();
            for _ in 0..nr_folds-1 {
                expanded_data.push_str(data);
                expanded_data.push_str("?");
            }
            expanded_data.push_str(data);
 
            let expanded_groups = groups.repeat(nr_folds);
            (expanded_data, expanded_groups)
        })
        .map(|(data, groups)| arragments(data.as_bytes(), &groups))
        .sum()
}


fn part_a_old(input: &str, nr_folds: usize) -> usize {
    let mut answer = 0;
    let lines = input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap())
        .map(|(l, r)| {
            (
                l.trim(),
                r.trim()
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(data, groups)| {
            let mut expanded_data = String::new();
            for _ in 0..nr_folds-1 {
                expanded_data.push_str(data);
                expanded_data.push_str("?");
            }
            expanded_data.push_str(data);
 
            let expanded_groups = groups.repeat(nr_folds);
            (expanded_data, expanded_groups)
        });

        
    for (idx, (map, groups)) in lines.enumerate() {
        println!("idx {}", idx);
        let mut nr_arrangements: usize = 0;
        let mut map = map.chars().collect();
        let mut groups:VecDeque<usize> = groups.clone().into();
        solve(&map, &mut groups, &mut nr_arrangements);
        answer += nr_arrangements;
    }
    return answer;
}

fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a_old(input, 1);
    println!("Part A: {:?}", ans_a);

    let ans_b = part_a_old(input, 5);
    println!("Part B: {:?}", ans_b);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0() {
        let input = "???.### 1,1,3";
        assert_eq!(part_a(input, 1), 1);
    }
    #[test]
    fn ex1() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(part_a(input, 1), 4);
    }
    #[test]
    fn ex2() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(part_a(input, 1), 1);
    }
    #[test]
    fn ex3() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(part_a(input, 1), 1);
    }
    #[test]
    fn ex4() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(part_a(input, 1), 4);
    }
    #[test]
    fn ex5() {
        let input = "?###???????? 3,2,1";
        assert_eq!(part_a(input, 1), 10);
    }
    #[test]
    fn in0() {
        let input = "#?#.??????#.??????? 3,2,1,1,7";
        assert_eq!(part_a(input, 1), 3);
    }
    #[test]
    fn in1() {
        let input = "????? 2,1";
        assert_eq!(part_a(input, 1), 3);
    }
    #[test]
    fn in2() {
        let input = "??#?#????#?#????. 8,7";
        assert_eq!(part_a(input, 1), 1);
        let input = "..????..?????? 1,3";
        assert_eq!(part_a(input, 1), 19);
        let input = "?#????#???????? 2,1,3,1";
        assert_eq!(part_a(input, 1), 33);
        let input = "?.?????#???????? 1,4,2,1";
        assert_eq!(part_a(input, 1), 61);
    }
    

}