use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
const INPUT: &str = aoc_utils::get_input!("23");
#[cfg(not(test))]
const INPUT: &str = aoc_utils::get_input!("23");

fn get_interconnected_nodes(
    visited: &std::collections::HashSet<&'static str>,
    previously_checked: &mut std::collections::HashSet<Vec<&'static str>>,
    common: &std::collections::HashSet<&'static str>,
    network: &std::collections::HashMap<&'static str, std::collections::HashSet<&'static str>>,
    interconnected_nodes: &mut std::collections::HashSet<Vec<&'static str>>,
) {
    // avoid rechecking previously seen nodes
    let mut visited_as_vec = visited.iter().copied().collect::<Vec<_>>();
    visited_as_vec.sort();
    if !previously_checked.insert(visited_as_vec) {
        return;
    }

    if common.is_empty() {
        let mut cycle = visited.iter().copied().collect::<Vec<&str>>();
        cycle.sort();
        interconnected_nodes.insert(cycle);
        return;
    }

    // continue with all interconnected nodes we encountered so far
    for node in common {
        let mut my_visited = visited.clone();
        if !my_visited.insert(node) {
            continue;
        }

        let my_common = common.intersection(&network[node]).copied().collect();
        get_interconnected_nodes(
            &my_visited,
            previously_checked,
            &my_common,
            network,
            interconnected_nodes,
        );
    }
}

pub fn solve() -> SolutionPair {
    let network_nodes = INPUT.lines();

    let mut network =
        std::collections::HashMap::<&'static str, std::collections::HashSet<&'static str>>::new();

    for pair in network_nodes {
        let mut nodes = pair.split('-');
        let node_a = nodes.next().unwrap();
        let node_b = nodes.next().unwrap();

        network.entry(node_a).or_default().insert(node_b);
        network.entry(node_b).or_default().insert(node_a);
    }

    let mut interconnected_nodes = std::collections::HashSet::<Vec<&str>>::new();
    let mut previously_checked = std::collections::HashSet::<Vec<&str>>::new();
    for (node, edges) in network.iter() {
        let mut visited = std::collections::HashSet::<&str>::new();
        visited.insert(node);
        get_interconnected_nodes(
            &visited,
            &mut previously_checked,
            edges,
            &network,
            &mut interconnected_nodes,
        );
    }

    let sol1 = interconnected_nodes
        .iter()
        .filter(|x| x.len() == 3 && x.iter().any(|n| n.starts_with('t')))
        .count();

    let sol2 = interconnected_nodes
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .join(",");
    (Solution::from(sol1), Solution::from(sol2))
}
