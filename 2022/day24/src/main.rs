use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blizzards {
    north: Vec<(i32, i32)>,
    south: Vec<(i32, i32)>, 
    east: Vec<(i32, i32)>,
    west: Vec<(i32, i32)>,
}
fn parse_input(input: &str) -> (Blizzards, (i32, i32)) {
    let mut blizzards = Blizzards{
        north: Vec::new(),
        south: Vec::new(),
        east: Vec::new(),
        west: Vec::new()
    };
    // read positions
    let mut max_row = 0;
    let mut max_col = 0;
    for (row, line) in input.split("\r\n").enumerate(){
        for (col, char) in line.chars().enumerate() {
            max_row = row as i32;
            max_col = col as i32;
            match char {
                '^' => {
                    blizzards.north.push((max_row, max_col))
                }
                'v' => {
                    blizzards.south.push((max_row, max_col))
                }
                '>' => {
                    blizzards.east.push((max_row, max_col))
                }
                '<' => {
                    blizzards.west.push((max_row, max_col))
                }
                '.' => {}
                _ => panic!()
            }
        }
    }
    return (blizzards, (max_row, max_col));
}

fn print_map(map: &HashMap<(i32, i32), char>, blizzards: &Blizzards, max_row: &i32, max_col: &i32) {
    for row in 0..=*max_row {
        for col in 0..=*max_col {
            match map.get(&(row, col)) {
                Some(_) => {print!("#")},
                None => {
                    if blizzards.north.contains(&(row, col)) {
                        print!("^")
                    } else if blizzards.south.contains(&(row, col)) {
                        print!("v")
                    } else if blizzards.east.contains(&(row, col)) {
                        print!(">")
                    } else if blizzards.west.contains(&(row, col)) {
                        print!("<")
                    } else {
                        print!(".")
                    }
                },
            }
        }
        println!()
    }
    println!()
}

fn advance_blizzards(blizzards: &mut Blizzards, max_row: &i32, max_col: &i32) {
    for bliz in blizzards.north.iter_mut() {
        let dx = -1;
        bliz.0 = (bliz.0 + dx).rem_euclid(max_row+1);
    }
    for bliz in blizzards.south.iter_mut() {
        let dx = 1;
        bliz.0 = (bliz.0 + dx).rem_euclid(max_row+1);
    }
    for bliz in blizzards.east.iter_mut() {
        let dy = 1;
        bliz.1 = (bliz.1 + dy).rem_euclid(max_col+1);
    }
    for bliz in blizzards.west.iter_mut() {
        let dy = -1;
        bliz.1 = (bliz.1 + dy).rem_euclid(max_col+1);
    }
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn take_step(
    blizzards_vec: &Vec<Blizzards>,
    pos: (i32, i32),
    goal: (i32, i32),
    max_row: &i32,
    max_col: &i32,
    step: i32,
    best: &mut i32,
) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(Step {
        pos,
        step,
    }); 
    
    while let Some(Step { pos, step}) = queue.pop_front() {
        // check if the current state has already been visited

        if step > *best {
            // save best solution, don't evaluate anything longer than that
            continue;
        }
        let this_step = step + 1;
        
        let bliz_index = this_step.rem_euclid(blizzards_vec.len() as i32) as usize;
        if visited.contains(&(pos, bliz_index)) {
            continue;
        }
        visited.insert((pos, bliz_index));

        let blizzards = blizzards_vec.get(bliz_index).unwrap();

        // check possible moves
        let possible_moves = vec![(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)];
        for mv in possible_moves {
            let new_pos = (pos.0 + mv.0, pos.1 + mv.1);
            if new_pos == goal {
                if this_step < *best {
                    *best = this_step;
                    println!("Solution found at {}", this_step);
                }
                return;
            }
            // eliminate invalid moves
            if (new_pos.0 < 0 || new_pos.1 < 0) && new_pos != (-1, 0){
                continue; // move not possible
            } else if (new_pos.0 > *max_row || new_pos.1 > *max_col) && new_pos != (max_row + 1, *max_col) {
                continue; // move not possible
            } else if blizzards.north.contains(&new_pos)
                || blizzards.south.contains(&new_pos)
                || blizzards.east.contains(&new_pos)
                || blizzards.west.contains(&new_pos)
            {
                continue; // move not possible, blizzard in the way
            } else {
                queue.push_back(Step {
                    pos: new_pos,
                    step: this_step,
                });
            }
        }
    }
}

struct Step {
    pos: (i32, i32),
    step: i32,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse the order so that the smallest steps are at the top of the heap
        other.step.cmp(&self.step)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.step == other.step
    }
}

impl Eq for Step {}

fn get_all_states(blizzards: &mut Blizzards, max_row: &i32, max_col: &i32) -> Vec<Blizzards> {
    let mut states = Vec::new();
    states.push(blizzards.clone());
    loop {
        advance_blizzards(blizzards, max_row, max_col);
        if states.contains(blizzards) {
            return states;
        }
        states.push(blizzards.clone());
    }
}

fn part_b(states: &Vec<Blizzards>, max_row: &i32, max_col: &i32) {
    let mut best = i32::MAX; 
    take_step(&states, (-1, 0), (max_row + 1, *max_col), &max_row, &max_col, 0, &mut best);
    let mut best2 = i32::MAX; 
    take_step(&states, (max_row + 1, *max_col), (-1, 0),&max_row, &max_col, best, &mut best2);
    let mut best3 = i32::MAX; 
    take_step(&states, (-1, 0), (max_row + 1, *max_col), &max_row, &max_col, best2, &mut best3);
    println!("Score B: {}", best3)  // 828

}
fn part_a(states: &Vec<Blizzards>, max_row: &i32, max_col: &i32) {
    let mut best = i32::MAX; 
    take_step(&states, (-1, 0), (max_row + 1, *max_col), &max_row, &max_col, 0, &mut best);
    println!("Best {}", best);  // 257
}
fn main() {
    let input = include_str!("input.txt");
    // parse input
    let (mut blizzards, (max_row, max_col)) = parse_input(input);
    //print_map(&map, &blizzards, &max_row, &max_col);
    let states = get_all_states(&mut blizzards, &max_row, &max_col); 

    part_a(&states, &max_row, &max_col);  
    part_b(&states, &max_row, &max_col);    
}
