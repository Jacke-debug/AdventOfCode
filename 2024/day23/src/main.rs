use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn part_a(nodes: &HashMap<&str, HashSet<&str>>) -> usize {
    let mut triplets = Vec::new();
    for (node, neighbors) in nodes.iter() {
        if !node.starts_with('t') {
            continue;
        }
        let neighbors_vec: Vec<_> = neighbors.iter().collect();
        for i in 0..neighbors_vec.len() {
            let n0 = neighbors_vec[i];
            for n1 in neighbors_vec.iter().skip(i) {
                if nodes.get(n0).unwrap().contains(*n1) {
                    let set = HashSet::from([node, n0, n1]);
                    if !triplets.contains(&set) {
                        triplets.push(set);
                    }
                }
            }
        }
    }
    triplets.len()
}
fn get_interconnected_nodes<'a>(
    visited: HashSet<&'a str>,
    checked: &mut HashSet<Vec<&'a str>>,
    remaining_nodes: HashSet<&'a str>,
    network: &'a HashMap<&'a str, HashSet<&'a str>>,
    interconnected_nodes: &mut HashSet<Vec<&'a str>>,
) {
    let mut visited_vec = visited.iter().copied().collect::<Vec<_>>();
    visited_vec.sort();

    if !checked.insert(visited_vec) {
        return;
    }

    if remaining_nodes.is_empty() {
        let mut cycle = visited.iter().copied().collect::<Vec<_>>();
        cycle.sort();
        interconnected_nodes.insert(cycle);
        return;
    }

    for &node in remaining_nodes.iter() {
        let mut new_visited = visited.clone();
        if !new_visited.insert(node) {
            continue;
        }
        let new_common: HashSet<_> = remaining_nodes
            .intersection(&network[node])
            .copied()
            .collect();
        get_interconnected_nodes(
            new_visited,
            checked,
            new_common,
            network,
            interconnected_nodes,
        );
    }
}

fn solve(input: &str) -> (usize, String) {
    let connections: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();
    let mut network = HashMap::new();
    for &(a, b) in &connections {
        network.entry(a).or_insert_with(HashSet::new).insert(b);
        network.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    let ans_a = part_a(&network);
    let mut connected_nodes = HashSet::new();
    let mut checked = HashSet::new();
    for (node, edges) in network.iter() {
        let mut visited = HashSet::new();
        visited.insert(*node);
        get_interconnected_nodes(
            visited,
            &mut checked,
            edges.clone(),
            &network,
            &mut connected_nodes,
        );
    }
    let ans_b = connected_nodes
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .join(",");
    (ans_a, ans_b)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = solve(input);
    assert_eq!(ans.0, 1075);
    assert_eq!(ans.1, "az,cg,ei,hz,jc,km,kt,mv,sv,sx,wc,wq,xy");
    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert!(ans.0 == 7);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert!(ans.1 == "co,de,ka,ta");
    }
}
