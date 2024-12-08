use std::collections::{HashMap, HashSet};

fn part_b(updates: &Vec<Vec<usize>>, rules: &Vec<(usize, usize)>) -> usize {
    let mut orderings = HashMap::<usize, HashSet<usize>>::new();
    for (x, y) in rules {
        orderings.entry(*y).or_default().insert(*x);
    }

    let mut ans_b = 0;
    let mut updates = updates.clone();
    for update in updates.iter_mut() {
        update.sort_by(|a, b| orderings[b].contains(a).cmp(&true));
        ans_b += update[update.len() / 2];
    }
    ans_b
}

fn solve(input: &str) -> (usize, usize) {
    let mut rules = vec![];
    let mut updates: Vec<Vec<usize>> = vec![];
    for line in input.lines() {
        if let Some((first, second)) = line.split_once('|') {
            if let (Ok(first), Ok(second)) = (
                first.trim().parse::<usize>(),
                second.trim().parse::<usize>(),
            ) {
                rules.push((first, second));
            }
        } else if let Ok(update_vec) = line
            .split(',')
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
        {
            updates.push(update_vec);
        } else {
            continue; // Skip blank lines
        }
    }

    let mut incorrect_updates = vec![];
    let mut ans_a = 0;
    'outer: for update in updates {
        for (first, second) in &rules {
            if update.contains(first) && update.contains(second) {
                let p1 = update.iter().position(|&x| x == *first);
                let p2 = update.iter().position(|&x| x == *second);
                if p1 > p2 {
                    incorrect_updates.push(update);
                    continue 'outer;
                }
            }
        }
        ans_a += update[update.len() / 2];
    }
    let ans_b = part_b(&incorrect_updates, &rules);
    (ans_a, ans_b)
}

fn main() {
    let input = include_str!("input.txt");
    let (ans_a, ans_b) = solve(input);
    assert_eq!(ans_a, 5452);
    assert_eq!(ans_b, 4598);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let (ans_a, ans_b) = solve(input);
        assert_eq!(ans_a, 143);
        assert_eq!(ans_b, 123);
    }
}
