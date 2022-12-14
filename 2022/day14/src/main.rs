use std::{collections::HashMap, borrow::Borrow, hash::Hash};


fn part_a(input: &str) -> i32 {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut map = HashMap::new();
    let mut x_min = 999;
    let mut x_max = -999;
    let mut y_min = 0;
    let mut y_max = -999;
    while let Some(line) = data.next() {
        let mut nodes = line.split(" -> ");
        let coordinate = nodes.next().unwrap();
        let (x_old, y_old) = coordinate.split_once(',').unwrap();
        let mut x_old: i32 = x_old.parse().unwrap();
        let mut y_old: i32 = y_old.parse().unwrap();

        while let Some(coordinate) = nodes.next() {
            let (x_new, y_new) = coordinate.split_once(',').unwrap();
            let x_new = x_new.parse().unwrap();
            let y_new: i32 = y_new.parse().unwrap();
            
            for x in x_old.min(x_new)..=x_old.max(x_new) {
                map.insert((x, y_old), '#');
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
            for y in y_old.min(y_new)..=y_old.max(y_new) {
                map.insert((x_old, y), '#');
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
            x_old = x_new;
            y_old = y_new;
        }
        
    }

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match map.get(&(x,y)) {
                Some(x) => {print!("{}", x)},
                None => {
                    print!(".")
                },
            };
        }
        println!("");
    }
    'outer: loop {
        score += 1;
        let mut sand_pos = (500, 0);
        // check down 
        let mut moving = true;
        while moving {
            sand_pos.1 += 1;
            match map.get(&(sand_pos)) {
                Some(_) => {
                    sand_pos.0 -=1;
                    match map.get(&sand_pos) {
                        Some(_) => {
                            sand_pos.0 +=2;
                            match map.get(&sand_pos) {
                                Some(_) => {
                                    // reset position
                                    sand_pos.0 -= 1;
                                    sand_pos.1 -= 1;
                                    moving = false;
                                }
                                None => {},
                            }
                        },
                        None => {},
                    }
                },
                None => {},
            }
            if sand_pos.1 > y_max {
                break 'outer;
            }
        }
        map.insert(sand_pos, 'o');
        
    }
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match map.get(&(x,y)) {
                Some(x) => {print!("{}", x)},
                None => {
                    print!(".")
                },
            };
        }
        println!("");
    }
    score -= 1;
    return score
}

fn part_b(input: &str) -> i32{
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut map = HashMap::new();
    let mut x_min = 999;
    let mut x_max = -999;
    let mut y_min = 0;
    let mut y_max = -999;
    while let Some(line) = data.next() {
        let mut nodes = line.split(" -> ");
        let coordinate = nodes.next().unwrap();
        let (x_old, y_old) = coordinate.split_once(',').unwrap();
        let mut x_old: i32 = x_old.parse().unwrap();
        let mut y_old: i32 = y_old.parse().unwrap();

        while let Some(coordinate) = nodes.next() {
            let (x_new, y_new) = coordinate.split_once(',').unwrap();
            let x_new = x_new.parse().unwrap();
            let y_new: i32 = y_new.parse().unwrap();
            
            for x in x_old.min(x_new)..=x_old.max(x_new) {
                map.insert((x, y_old), '#');
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
            for y in y_old.min(y_new)..=y_old.max(y_new) {
                map.insert((x_old, y), '#');
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
            x_old = x_new;
            y_old = y_new;
        }
        
    }

    for y in y_min..=y_max+2 {
        for x in x_min..=x_max {
            match map.get(&(x,y)) {
                Some(x) => {print!("{}", x)},
                None => {
                    print!(".")
                },
            };
        }
        println!("");
    }
    
    'outer: loop {
        score += 1;
        let mut sand_pos = (500, 0);
        // check down 
        let mut moving = true;
        while moving {
            sand_pos.1 += 1;
            match map.get(&(sand_pos)) {
                Some(_) => {
                    sand_pos.0 -=1;
                    match map.get(&sand_pos) {
                        Some(_) => {
                            sand_pos.0 +=2;
                            match map.get(&sand_pos) {
                                Some(_) => {
                                    // reset position
                                    sand_pos.0 -= 1;
                                    sand_pos.1 -= 1;
                                    moving = false;
                                }
                                None => {},
                            }
                        },
                        None => {},
                    }
                },
                None => {},
            }
            if sand_pos.1 == y_max+1 {
                moving = false;
            }
            if sand_pos == (500, 0) {
                break 'outer;
            }
        }
        map.insert(sand_pos, 'o');
        
    }
    for y in y_min..=y_max+2 {
        for x in x_min..=x_max {
            match map.get(&(x,y)) {
                Some(x) => {print!("{}", x)},
                None => {
                    print!(".")
                },
            };
        }
        println!("");
    }
    // high 32394
    return score
}



fn main(){
    let input = include_str!("example.txt");
    let score_a = part_a(input);
    println!();
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: i32 = 24;
    const SOLVE_A: i32 = 964;

    const EXAMPLE_B: i32 = 93;
    const SOLVE_B: i32 = 32041;

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