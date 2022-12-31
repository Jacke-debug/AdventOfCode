use std::{collections::{HashMap, HashSet}, ops::Deref};

#[derive(Debug)]
struct Direction {
    name: char,
    pos: Vec<(i32, i32)>
}

fn get_area(map: &HashSet<(i32, i32)>) -> i32 {
    let min_row = map.iter().map(|(x, _)| x).min().unwrap();
    let max_row = map.iter().map(|(x, _)| x).max().unwrap();
    let min_col = map.iter().map(|(_, y)| y).min().unwrap();
    let max_col = map.iter().map(|(_, y)| y).max().unwrap();
    let num_elves = map.len() as i32;
    let res = (((max_row-min_row)+1) * ((max_col-min_col)+1)) - num_elves;
    return res
}
fn print_map(map: &HashSet<(i32, i32)>){
    let min_row = map.iter().map(|(x, _)| x).min().unwrap();
    let max_row = map.iter().map(|(x, _)| x).max().unwrap();
    let min_col = map.iter().map(|(_, y)| y).min().unwrap();
    let max_col = map.iter().map(|(_, y)| y).max().unwrap();
    for row in *min_row..=*max_row {
        for col in *min_col..=*max_col {
            match map.get(&(row, col)) {
                Some(_) => {print!("#")},
                None => {print!(".")},
            }
        }
        println!()
    }
    println!()
}
fn get_directions() -> Vec<Direction>{
    let north = Direction{
        name: 'N',
        pos: vec![(-1, -1), (-1, 0), (-1, 1)],
    };
    let south = Direction{
        name: 'S',
        pos: vec![(1, -1), (1, 0), (1, 1)],
    };
    let west = Direction{
        name: 'W',
        pos: vec![(-1, -1), (0, -1), (1, -1)],
    };
    let east = Direction{
        name: 'E',
        pos: vec![(-1, 1), (0, 1), (1, 1)],
    };
    return vec![north, south, west, east];
}
fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    let mut map:HashSet<(i32, i32)> = HashSet::new();
    // read positions
    for (row, line) in input.split("\r\n").enumerate(){
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                map.insert((row as i32, col as i32));
            }
        }
    }
    return  map;
}
fn run_steps(map: &mut HashSet<(i32, i32)>, steps: i32) -> (i32, i32){
    let mut directions = get_directions();
    let mut count = 0;
    'outer: for _ in 0..steps {
        count += 1;
        // check where each can move
        let mut moves = HashMap::new();
        for (x, y) in map.iter() {
            // check if all squares around are free, then do nothing
            let mut neighbours = 0;
            for row in x-1..=x+1 {
                for col in y-1..=y+1 {
                    match map.get(&(row, col)) {
                        Some(_) => {neighbours+=1},
                        None => {},
                    }
                }
            }
            if neighbours == 1 {
                // no need to move, all squares around are free
                continue;
            }

            // save locations to move to and associated elf
            'dir: for dir in &directions {
                for (dx, dy) in &dir.pos {
                    let x_new = x+dx;
                    let y_new = y+dy;
                    match map.get(&(x_new, y_new)) {
                        Some(_) => {
                            // check next direction
                            continue 'dir;
                        },
                        None => {
                        },
                    }
                }
                moves.insert((*x, *y), (x+dir.pos[1].0, y+dir.pos[1].1));
                break;
            }
        }
        if moves.is_empty() {
            break 'outer;
        }

        // only move unique
        let pos_to_move_to: HashMap<(i32, i32), usize> = moves.values()
            .fold(HashMap::new(), |mut count, v| {
                *count.entry(*v).or_insert(0) += 1;
                count
            });
        let move_to_make: HashMap<_, _> = moves.into_iter()
            .filter(|(_, v)| pos_to_move_to[v] == 1)
            .collect();

        for ((row, col), to)  in move_to_make {
            map.remove(&(row, col));
            map.insert(to);
        }

        // rotate direction list
        directions.rotate_left(1);
    }
    let score = get_area(&map);
    return (count, score)
}

fn main() {
    let input =  include_str!("input.txt");
    let steps = i32::MAX; // part_a => 10,  part_b => i32::MAX
    let mut map = parse_input(input);
    let (count, score) = run_steps(&mut map, steps);
    println!("Score: {}, in {}", score, count); // 3966 high
}
