use std::{collections::HashMap, borrow::Borrow, hash::Hash};

fn char_to_num(letter: char) -> u32 {

    let num: u32 = letter.into();
    let mut m: u32 = 'a'.into();    // 97 

    let score: u32 = (num - m) + 1;
    // println!("Letter {}, number {:?}", letter, score);
    return score
}

fn part_a(input: &str) -> i32 {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut map_shortest = HashMap::new();

    let mut num_x = 0;
    let mut num_y = 0;
    let mut goal = (0, 0);
    let mut start = (0, 0);
    for (y, line) in data.enumerate() {
        num_y += 1;
        num_x = 0;
        for (x, char) in line.chars().enumerate() {
            num_x += 1;
            if char == 'S' {
                start = (x as i32, y as i32);
                map.insert((x as i32, y as i32), char_to_num('a') as i32);
                map_shortest.insert((x as i32, y as i32), 0);
            } else if char == 'E' {
                goal = (x as i32, y as i32);
                map.insert(goal, char_to_num('z') as i32);
            } else {
                map.insert((x as i32, y as i32), char_to_num(char) as i32);
            }
        }
    }
    
    for y in 0..num_y {
        for x in 0..num_x {
            print!("{:#02} ", map.get(&(x,y)).unwrap());
        }
        println!("");
    }
    println!("");


    let mut shortest_path = 999;
    let mut current_distance = 1;
    // walk the path

    take_step(&mut map, &mut map_shortest, &start, current_distance, goal, &mut shortest_path);

    println!("{}, {}", num_x, num_y);
    print_map(map_shortest.clone());
    
    score = *map_shortest.get(&goal).unwrap();
    // Not: 474
    return score
}

fn print_map(map: HashMap<(i32, i32), i32>) {
    for y in 0..41 { // 41
        for x in 0..60 { // 154
            if let Some(val) = map.get(&(x,y)) {
                print!("{:#02} ", val);
            } else {
                print!("¤¤ ");
            }
        }
        println!("")
    }
}

fn take_step(map: &mut HashMap<(i32, i32), i32>, map_shortest: &mut HashMap<(i32, i32), i32>, 
pos: &(i32, i32), current_distance: i32, goal: (i32, i32), shortest_path: &mut i32) {
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        //print_map(old_map.clone());
        //let mut map_shortest = old_map.clone();
        
        let (dx, dy) = dir;
        let new_pos = (pos.0 + dx, pos.1 + dy);
        let current_height = map.get(pos).unwrap();
        
        if let Some(new_heigth) = map.get(&new_pos) {
            if new_heigth <= &(current_height + 1) {
                // move possible
                if let Some(best_known_way_here) = map_shortest.get(&new_pos) {
                    // we have been here before, but is our current path better?
                    if current_distance < *best_known_way_here {
                        map_shortest.insert((new_pos), current_distance);
                    } else {
                        // path is worse, we can stop 
                        continue;
                    }
                } else {
                    // we have not been here before
                    map_shortest.insert((new_pos), current_distance);
                }
                
                if new_pos == goal {
                    continue;
                }
                
                take_step(map, map_shortest, &new_pos, current_distance+1, goal, shortest_path);
            }
        } else {
            // new step is outside of map
            continue;
        }
    }
}

fn part_b(input: &str) -> i32{
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut map_shortest = HashMap::new();

    let mut num_x = 0;
    let mut num_y = 0;
    let mut goal = (0, 0);
    let mut start = (0, 0);
    for (y, line) in data.enumerate() {
        num_y += 1;
        num_x = 0;
        for (x, char) in line.chars().enumerate() {
            num_x += 1;
            if char == 'S' {
                start = (x as i32, y as i32);
                map.insert((x as i32, y as i32), char_to_num('a') as i32);
                map_shortest.insert((x as i32, y as i32), 0);
            } else if char == 'E' {
                goal = (x as i32, y as i32);
                map.insert(goal, char_to_num('z') as i32);
            } else {
                map.insert((x as i32, y as i32), char_to_num(char) as i32);
            }
        }
    }

    let mut shortest_path = 999;
    let mut current_distance = 1;
    // walk the path
    let starting_pos: Vec<(i32, i32)> = map.iter().filter_map(|(key, val)| if *val == 1 {Some(*key)} else {None}).collect();
    for start in starting_pos {
        take_step(&mut map, &mut map_shortest, &start, current_distance, goal, &mut shortest_path);
    }

    //println!("{}, {}", num_x, num_y);
    //print_map(map_shortest.clone());
    
    score = *map_shortest.get(&goal).unwrap();
    // Not: 474
    return score
}



fn main(){
    let input = include_str!("input.txt");
    let score_a = part_a(input);
    println!();
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: i32 = 31;
    const SOLVE_A: i32 = 472;

    const EXAMPLE_B: i32 = 29;
    const SOLVE_B: i32 = 465;

    use super::*;
    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), EXAMPLE_A);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), EXAMPLE_B);
    }

    #[test]
    fn solve_a() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), SOLVE_A);
    }

    #[test]
    fn solve_b() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), SOLVE_B);
    }
}