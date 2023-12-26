use pathfinding::directed::bfs::bfs_reach;
use std::collections::{HashMap, VecDeque, HashSet};

fn find_bridge<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> (&'a str, &'a str) {
    let mut paths: HashMap<(&str, &str), usize> = HashMap::new();
    for start in graph.keys().copied() {
        let mut to_see = VecDeque::new();
        to_see.push_back(start);
        let mut seen = HashSet::new();
        seen.insert(start);
        while let Some(node) = to_see.pop_front() {
            for n in graph[&node].iter().copied() {
                if !seen.contains(&n) {
                    to_see.push_back(&n);
                    seen.insert(n);
                    let edge = if n < node {(n, node)} else {(node, n)};
                    *paths.entry(edge).or_default() += 1;
                }
            }
        }
    }
    paths.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}

fn part_a(input: &str) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (inp, out) = line.split_once(": ").unwrap();
        for output in out.split_ascii_whitespace() {
            graph.entry(inp).or_default().insert(output);
            graph.entry(output).or_default().insert(inp);
        }
    }

    for _ in 0..3 {
        let bridge = find_bridge(&graph);
        graph.get_mut(&bridge.0).unwrap().remove(bridge.1);
        graph.get_mut(&bridge.1).unwrap().remove(bridge.0);
    }
    let gl = bfs_reach(*graph.keys().next().unwrap(), |n| graph[n].iter().copied()).count();
    gl * (graph.len() - gl)
}

fn main() {
    // cred https://gist.github.com/samueltardieu/
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {}", ans_a); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), 54);
    }
}