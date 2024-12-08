use std::{collections::VecDeque, time::Instant, vec};

fn solve(ans: &usize, sum: &usize, vec: &VecDeque<usize>, part_b: bool) -> bool {
    let operators = match part_b {
        false => vec!['+', '*'],
        true => vec!['+', '*', '|'],
    };

    let mut new_vec = vec.clone();

    if let Some(val) = new_vec.pop_front() {
        for operator in &operators {
            let new_sum = match operator {
                '+' => sum + val,
                '*' => sum * val,
                '|' => (sum.to_string() + &val.to_string()).parse().unwrap(),
                _ => unreachable!(),
            };
            if solve(ans, &new_sum, &new_vec, part_b) {
                return true;
            }
        }
        return false;
    }
    *sum == *ans
}

fn part_a(input: &str, part_b: bool) -> usize {
    let mut score = 0;
    for line in input.lines() {
        let (ans, vec) = line.split_once(": ").unwrap();
        let ans = ans.parse::<usize>().unwrap();
        let mut vec = vec
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<VecDeque<_>>();
        let sum = vec.pop_front().unwrap();
        if solve(&ans, &sum, &vec, part_b) {
            score += ans;
        }
    }
    score
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = part_a(input, false);
    assert_eq!(ans, 2654749936343);

    let ans = part_a(input, true);
    assert_eq!(ans, 124060392153684);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(&input, false);
        assert_eq!(ans, 3749);
    }

    #[test]
    fn example_a_2() {
        let input = "3267: 81 40 27";
        let ans = part_a(&input, false);
        assert_eq!(ans, 3267);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = part_a(&input, true);
        assert_eq!(ans, 11387);
    }
}
