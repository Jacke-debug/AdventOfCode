use std::collections::{HashMap, HashSet};

fn is_small(cave: &str) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

fn enumerate<'a>(map: &'a HashMap<&'a str, Vec<&'a str>>, mut path: Vec<&'a str>, from: &'a str, visited: HashSet<&'a str>, supports_dup_small_cave: bool) -> usize {
    path.push(from);

    if from.eq("end") {
        //println!("path: {:?}", path);
        return 1;
    }
    let mut count = 0;
    for to in map.get(from).unwrap() {
        let mut visited = visited.clone();
        let mut supports_dup_small_cave = supports_dup_small_cave;
        if is_small(to) {
            if visited.contains(to) {
                if supports_dup_small_cave && *to != "start" && *to != "end" {
                    supports_dup_small_cave = false;
                } else {
                    continue;
                }
            } else {
                visited.insert(*to);
            }
        } 

        count += enumerate(map, path.clone(), to, visited.clone(), supports_dup_small_cave);
    }
    count
}

fn main() {
    let input = include_str!("input.txt");
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.trim().split("\r\n") {
        let (from, to) = line.split_once("-").unwrap();
        map.entry(from).or_default().push(to);
        map.entry(to).or_default().push(from);
    }

    println!("map: {:?}", map);
    let mut visited = HashSet::new();
    visited.insert("start");
    let supports_dup_small_cave = true;
    let count = enumerate(&map, Vec::new(), "start", visited, supports_dup_small_cave);

    println!("count: {}", count)
}
