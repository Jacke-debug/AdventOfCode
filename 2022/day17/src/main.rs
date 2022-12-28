use std::collections::{BTreeSet, BTreeMap};

fn parse_rocks() -> Vec<Vec<(i64, i64)>> {
    let mut rocks: Vec<Vec<(i64, i64)>> = Vec::new();
    let mut data = include_str!("rocks.txt").trim().split("\r\n");
    
    let mut y = 0;
    let mut rock: Vec<(i64, i64)> = Vec::new();
    while let Some(mut line) = data.next() {
        if line.is_empty() {
            line = data.next().unwrap();
            rocks.push(rock);
            rock = Vec::new();
            y = 0;
        }
        let mut x = 0;
        for shape in line.chars() {
            match shape {
                '#' => {
                    rock.push((x, y));
                    x += 1;
                }
                _ => {x += 1;},
            }
        }
        y += 1;
    }
    rocks.push(rock);
    return rocks
}

fn free(settled: &BTreeSet<(i64, i64)>, x: i64, y: i64) -> bool {
    return (x >= 0) && (x < WIDTH) && (y > 0) && ! settled.contains(&(x, y));
}

fn can_move(settled: &BTreeSet<(i64, i64)>, piece: i64, x: i64, y: i64, rocks: &Vec<Vec<(i64, i64)>>) -> bool {
    return rocks[piece as usize].iter().all(|(dx, dy)|  free(settled, x + dx, y + dy));
}

fn place(settled: &mut BTreeSet<(i64, i64)>, jet: i64, piece: i64, max_y: i64, jets: &Vec<char>, rocks:  &Vec<Vec<(i64, i64)>>) -> (i64, i64, i64) {
    let mut x = 2;
    let mut y = max_y + 5;
    let mut new_jet = jet;
    while can_move(settled, piece, x, y - 1, rocks) {
        y -= 1;
        match jets.get(new_jet as usize) {
            Some('<') => {
                if can_move(settled, piece, x-1, y, rocks) {
                    x -= 1;
                }
            },
            Some('>') => {
                if can_move(settled, piece, x+1, y, rocks) {
                    x += 1;
                }
            },
            _ => {panic!()},
        }
        new_jet = (new_jet + 1) % (jets.len() as i64);
    }
    let new_cells: Vec<(i64, i64)> = rocks[piece as usize].iter().map(|(dx, dy)| (x + dx, y + dy)).collect();
    new_cells.iter().for_each(|cell| { settled.insert(*cell); });
    return (new_jet, (piece + 1) % rocks.len() as i64, max_y.max(new_cells.iter().map(|(_, y)| *y).max().unwrap()));
}

fn ground_shape(settled: &BTreeSet<(i64, i64)>, max_y: i64) -> Option<Vec<(i64, i64)>> {
    let mut state: BTreeSet<(i64, i64)> = BTreeSet::new();
    for x in 0..WIDTH {
        search(x, 0, &mut state, max_y, settled);
    }
    if state.len() <= CACHE_LEN {
        return Some(state.into_iter().collect::<Vec<(i64, i64)>>());
    } else {
        return None;
    }
}

fn search(x: i64, y: i64, visited: &mut BTreeSet<(i64, i64)>, max_y: i64, settled: &BTreeSet<(i64, i64)>) {
    if (! free(settled, x, max_y + y)) || visited.contains(&(x, y)) || visited.len() > CACHE_LEN {
        return;
    }
    visited.insert((x, y));
    vec![(x - 1, y), (x + 1, y), (x, y - 1)].iter().for_each(|(nx, ny)| {
        search(*nx, *ny, visited, max_y, settled);
    })
}

fn solve(num_rocks: i64, jets: &Vec<char>, rocks: &Vec<Vec<(i64, i64)>>) -> i64 {
    let mut settled: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut cycles: BTreeMap<(i64, i64, Vec<(i64, i64)>), (i64, i64)> = BTreeMap::new();
    let mut jet = 0;
    let mut max_y = 0;
    let mut piece = 0;
    let mut addl = 0;
    let mut count = num_rocks;

    while count > 0 {
        (jet, piece, max_y) = place(&mut settled, jet, piece, max_y, jets, rocks);
        count -= 1;
        if let Some(ground) = ground_shape(&settled, max_y) {
            if cycles.contains_key(&(jet, piece, ground.clone())) {
                let (old_max_y, old_count) = cycles.get(&(jet, piece, ground.clone())).unwrap();
                addl += (max_y - old_max_y) * (count / (old_count - count));
                count %= old_count - count;
            }
            cycles.insert((jet, piece, ground), (max_y, count));
        } else {
            continue;
        }
    }
    return max_y + addl;
}

const WIDTH: i64 = 7;
const CACHE_LEN: usize = 20;

fn main() {
    // solution based code from JuniorBirdman1115
    //let num_rocks = 2022;
    let num_rocks = 1000000000000;
    let rocks = parse_rocks();
    let input = include_str!("input.txt");
    let mut data = input.trim().split("\r\n");
    let line = data.next().unwrap();
    let jets: Vec<char> = line.chars().collect();
    let score = solve(num_rocks, &jets, &rocks);
    println!("score {}", score);
}