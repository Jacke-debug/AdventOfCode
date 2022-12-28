use std::{collections::{BinaryHeap, HashSet, HashMap, BTreeSet, VecDeque}, cmp::Ordering};
use itertools::Itertools;

struct Valve<'a> {
    flow: u32, 
    neighbours: HashSet<&'a str>,
}

#[derive(PartialEq, Eq)]
struct Node<'a> {
    cost: u32, 
    curr: &'a str,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_cost(from: &str, to: &str, map: &HashMap<&str, Valve>) -> u32 {
    let mut priority_que = BinaryHeap::new();
    let mut seen = HashSet::new();

    priority_que.push(Node {
        cost: 0,
        curr: from,
    });
    seen.insert(from);

    while let Some(Node {cost, curr}) = priority_que.pop() {
        if curr == to {
            return cost;
        }

        for neighbour in map[curr].neighbours.iter() {
            if seen.insert(neighbour) {
                priority_que.push(Node {
                    cost: cost + 1,
                    curr: neighbour,
                });
            }
        }
    }
    u32::MAX
}

fn parse(input: &str) -> HashMap<&str, Valve> {
    let mut data = input.trim().split("\r\n");
    let mut map: HashMap<&str, Valve> = HashMap::new();

    while let Some(line) = data.next() {
        let (name, rest) = line.split_once(" has").unwrap();
        let (_, name) = name.split_once(' ').unwrap();
        let (rate, rest) = rest.split_once(";").unwrap();
        let (_, rate) = rate.split_once("=").unwrap();
        let rate: u32 = rate.parse().unwrap();
        let (_, con_str) = rest.split_once("valves ").unwrap();

        map.insert(name,  
            Valve{
                flow: rate,
                neighbours: con_str.split(", ").collect(),
            }
        );
    }
    return map;
}

fn min_distances<'a>(map: &'a HashMap<&str, Valve>) -> HashMap<(&'a str, &'a str), u32> {
    map.iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            acc.entry(("AA", name1))
                .or_insert_with(|| min_cost("AA", name1, map));
            acc.entry(("AA", name2))
                .or_insert_with(|| min_cost("AA", name2, map));

            let dist = min_cost(name1, name2, map);
            acc.insert((name1, name2), dist);
            acc.insert((name2, name1), dist);

            acc
        })
}

struct State<'a> {
    opened: BTreeSet<&'a str>,
    curr: &'a str, 
    elapsed: u32, 
    relieved: u32,
}

fn wait_until_ending(
    max_time: u32,
    elapsed: u32,
    relieved: u32,
    opened: &BTreeSet<&str>,
    map: &HashMap<&str, Valve>,
) -> u32 {
    let time_left = max_time - elapsed;
    let relieved_per_min: u32 = opened.iter().map(|name| &map[name].flow).sum();
    relieved + (relieved_per_min * time_left)
}

fn part_a(input: &str) -> u32 {
    let max_time = 30;
    let map = parse(input);
    let dist_map = min_distances(&map);
    let flowing: HashSet<_> = map.iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .collect();
    let mut max_relieved = 0;
    let mut que = VecDeque::new();
    let mut seen = HashSet::new();

    que.push_back(State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });
    seen.insert((BTreeSet::new(), 0, 0));

    while let Some(State {
        opened, 
        curr, 
        elapsed, 
        relieved,
    }) = que.pop_front() {
        if opened.len() == flowing.len() || elapsed >= max_time {
            let relieved_at_end = wait_until_ending(max_time, elapsed, relieved, &opened, &map);
            max_relieved = max_relieved.max(relieved_at_end);
            continue;
        }

        let unopened = flowing.iter().filter(|name| !opened.contains(*name));
        for dest in unopened {
            let cost = dist_map[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;

            if new_elapsed >= max_time {
                let relieved_at_end = wait_until_ending(max_time, elapsed, relieved, &opened, &map);
                max_relieved = max_relieved.max(relieved_at_end);
                continue;
            }

            let relieved_per_min: u32 = opened.iter().map(|name| &map[name].flow).sum();
            let new_relieved = relieved + (relieved_per_min * cost);

            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            if seen.insert((new_opened.clone(), new_elapsed, new_relieved)) {
                que.push_back(State {
                    opened: new_opened,
                    curr: dest,
                    elapsed: new_elapsed,
                    relieved: new_relieved,
                });
            }
        }
    }
    return max_relieved;
}

fn part_b(input: &str) -> u32 {
    let max_time = 26;
    let map = parse(input);
    let dist_map = min_distances(&map);


    let flowing: HashSet<_> = map.iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .collect();
    let mut max_relieved_states: HashMap<BTreeSet<&str>, u32> = HashMap::new();
    
    let mut que = VecDeque::new();
    que.push_back(State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });

    while let Some(State {
        opened, 
        curr, 
        elapsed, 
        relieved,
    }) = que.pop_front() {
        let relieved_at_end = wait_until_ending(max_time, elapsed, relieved, &opened, &map);
        max_relieved_states
            .entry(opened.clone())
            .and_modify(|val| *val = relieved_at_end.max(*val))
            .or_insert(relieved_at_end);

        let unopened = flowing.iter().filter(|name| !opened.contains(*name));
        for dest in unopened {
            let cost = dist_map[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;

            if new_elapsed >= max_time {
                continue;
            }

            let relieved_per_min: u32 = opened.iter().map(|name| &map[name].flow).sum();
            let new_relieved = relieved + (relieved_per_min * cost);

            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            que.push_back(State {
                opened: new_opened,
                curr: dest,
                elapsed: new_elapsed,
                relieved: new_relieved,
            });
        }
    }
    max_relieved_states
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap()
}

fn main() {
    // code based on NickyMeuleman's solution
    let input = include_str!("input.txt");
    let score_a = part_a(input);
    println!("Score A {}", score_a);
    let score_b= part_b(input);
    println!("Score B {}", score_b)
}