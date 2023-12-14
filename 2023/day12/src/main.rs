use core::panic;
use std::collections::VecDeque;

fn reduce_problem(map: &mut VecDeque<char>, groups: &mut VecDeque<usize>, nr_arrangements: &mut usize) -> bool {
    let mut size_changed = true;
    while size_changed && map.len()>0{
        let orig_size = map.len();

        if map.front()== Some(&'#') {
            println!("popping front");
            let len = match groups.pop_front() {
                Some(x) => {x},
                None => {
                    return true}, // no more ponds to place, done
            };
            let map_size = map.len();
            if len > map_size {
                return true;
            }
            map.drain(..len); // drain specifies indices to remove...
        }
        
        if map.back() == Some(&'#') {
            println!("popping back");
            let len = match groups.pop_back() {
                Some(x) => {x},
                None => {return true}, // no more ponds to place, done
            };
            let map_size = map.len();
            if len > map_size {
                return true;
            }
            map.drain(map_size-len..);
        }
        
        while let Some('.') = map.back() {
            map.pop_back();
        }
        while let Some('.') = map.front() {
            map.pop_front();
        }

        size_changed = map.len() < orig_size;
    }
    return false;
}

fn solve(map: &mut VecDeque<char>, groups: &mut VecDeque<usize>, nr_arrangements: &mut usize){
    let no_solution = reduce_problem(map, groups, nr_arrangements);
    if no_solution {
        return;
    }
    if map.is_empty() && groups.is_empty() {
        *nr_arrangements += 1; //?
        println!("Reduction solved {}", nr_arrangements);
        return;
    }
    println!("After reduction {:?}, {:?}", map, groups);
    // check posibilities for first remaining group
    let group = match groups.pop_front() {
        Some(x) => {x},
        None => {
            *nr_arrangements += 1; //?
            println!(" nr_arrangements {}", nr_arrangements);
            return;
        },
    };
    let required_size: i32 = groups.len() as i32 + groups.iter().sum::<usize>() as i32;
    if (map.len() as i32) < required_size {
        println!("Remaing map too small");
        return;
    }

    'place_next: for place_index in 0..map.len()-required_size as usize {
        println!("place index {}, for group {}", place_index, group);
        for idx in 0..place_index{
            match map.get(idx) {
                Some('#') => {continue 'place_next},
                _ => {}
            }
        }
        for idx in place_index..place_index+group {
            match map.get(idx) {
                Some('.') => {continue 'place_next},
                Some(_) => continue,
                None => {continue 'place_next},
            }
        }
        if groups.len() == 0 {
            *nr_arrangements+=1;
            println!("No more groups to place, nr_arrangements {}", nr_arrangements);
            continue 'place_next;
        }
        match map.get(place_index+group) {
            Some('#') => {
                println!("Can't insert . after group ");
                continue;
             } 
            Some(_) => {
                let mut new_map = map.clone();
                let mut new_groups = groups.clone();
                if place_index+group+1 >= new_map.len() {
                    continue 'place_next;
                }
                new_map.drain(..place_index+group+1);
                solve(&mut new_map, &mut new_groups, nr_arrangements);
            }
            None => {
                continue 'place_next;
            }
        }
    }

}

fn part_a(input: &str) -> usize {
    let mut answer = 0;
    for (nr, line) in input.trim().split("\r\n").enumerate() {
        let (map, groups) = line.split_once(' ').unwrap();
        let mut groups: VecDeque<usize> = groups.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        let mut map = map.to_string()
            .trim_start_matches('.')
            .trim_end_matches('.')
            .chars().collect();
        let mut nr_arrangements: usize = 0;

        solve(&mut map, &mut groups, &mut nr_arrangements);
        println!("Nr {} Solutions {}", nr, nr_arrangements);
        answer += nr_arrangements;
        if nr ==1 {
            continue;
        }
    }
    return answer
}


fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {:?}", ans_a);
    // 5055 low, 7957 wrong, 9495 wrong, 8929 wrong, 9437, 8451
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0() {
        let input = "???.### 1,1,3";
        assert_eq!(part_a(input), 1);
    }
    #[test]
    fn ex1() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(part_a(input), 4);
    }
    #[test]
    fn ex2() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(part_a(input), 1);
    }
    #[test]
    fn ex3() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(part_a(input), 1);
    }
    #[test]
    fn ex4() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(part_a(input), 4);
    }
    #[test]
    fn ex5() {
        let input = "?###???????? 3,2,1";
        assert_eq!(part_a(input), 10);
    }
    #[test]
    fn in0() {
        let input = "#?#.??????#.??????? 3,2,1,1,7";
        assert_eq!(part_a(input), 3);
    }
    #[test]
    fn in1() {
        let input = "????? 2,1";
        assert_eq!(part_a(input), 3);
    }
    #[test]
    fn in2() {
        let input = "??#?#????#?#????. 8,7";
        assert_eq!(part_a(input), 1);
    }
}