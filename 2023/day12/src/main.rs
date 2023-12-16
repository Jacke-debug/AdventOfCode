use core::panic;
use std::{collections::{VecDeque, HashMap}, time::Instant};
use rayon::prelude::*;


fn solve(map: & VecDeque<char>, groups: &mut VecDeque<usize>, nr_arrangements: &mut usize){
    let group = match groups.pop_front() {
        Some(x) => {x},
        None => {
            *nr_arrangements +=1;
            return;
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


fn part_a(input: &str, nr_folds: usize) -> usize {
    let mut answer = 0;
    let lines: Vec<(String, Vec<usize>)> = input
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
        }).collect();

    
    let answer: usize = lines
    .par_iter() // Use par_iter() instead of iter() for parallel iteration
    .enumerate()
    .map(|(idx, (map, groups))| {
        println!("idx {}", idx);
        let mut nr_arrangements: usize = 0;
        let mut map = map.chars().collect();
        let mut groups: VecDeque<usize> = groups.clone().into();
        solve(&map, &mut groups, &mut nr_arrangements);
        nr_arrangements
    })
    .sum();
    return answer;
}

fn main() {
    let input = include_str!("input.txt");
    //let ans_a = part_a(input, 1);
    //println!("Part A: {:?}", ans_a);

    let start_time = Instant::now();
    let ans_b = part_a(input, 5);
    println!("Part B: {:?}", ans_b);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0() {
        let input = "???.### 1,1,3";
        assert_eq!(part_a(input, 1), 1);
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(part_a(input, 1), 4);
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(part_a(input, 1), 1);
        let input = "????.#...#... 4,1,1";
        assert_eq!(part_a(input, 1), 1);
        let input = "????.######..#####. 1,6,5";
        assert_eq!(part_a(input, 1), 4);
        let input = "?###???????? 3,2,1";
        assert_eq!(part_a(input, 1), 10);
        let input = "#?#.??????#.??????? 3,2,1,1,7";
        assert_eq!(part_a(input, 1), 3);
        let input = "????? 2,1";
        assert_eq!(part_a(input, 1), 3);
        let input = "??#?#????#?#????. 8,7";
        assert_eq!(part_a(input, 1), 1);
        let input = "..????..?????? 1,3";
        assert_eq!(part_a(input, 1), 19);
        let input = "?#????#???????? 2,1,3,1";
        assert_eq!(part_a(input, 1), 33);
        let input = "?.?????#???????? 1,4,2,1";
        assert_eq!(part_a(input, 1), 61);
    }
    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input, 1), 21);
    }
    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input, 5), 525152);
    }

}