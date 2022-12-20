use std::collections::{HashSet};

fn get_y_max(map: & HashSet<(i64, i64)>) -> i64 {
    let mut y_max = 0;
    for (_, y) in map.iter() {
        y_max = y_max.max(*y);
    }
    return y_max;
}


fn print_2d_map(map: & HashSet<(i64, i64)>) {
    println!();
    let y_max = get_y_max(&map);
    for y in 0..=y_max{
        let y_pos = y_max-y;
        for x in 0..7 {
            match map.get(&(x, y_pos)) {
                Some(_) => {
                    print!("#");
                },
                None => {print!(".");},
            }
        }
        println!();
    }
}

fn print_rocks(rocks:  & Vec<Vec<(i64, i64)>>) {
    for rock in rocks {
        for y in 0..=3 {
            for x in 0..=3 {
                if rock.contains(&(x,y)) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

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

fn move_rock(rock: &mut Vec<(i64, i64)>, pos: & (i64, i64)) {
    for (x, y) in rock {
        *x += pos.0;
        *y += pos.1;
    }
}

fn part_a(input: &str, pattern: &Vec<char>, iters: usize) -> i64{
    let mut score = 0;

    let mut map = HashSet::new();
    for x in 0..7 {
        map.insert((x, 0));
    }

    let rocks = parse_rocks();
    //print_rocks(&rocks);
    
    //print_2d_map(&map);

    let mut count = 0;
    'new_rock: for nr in 0..iters {
        let y_start = get_y_max(&map) + 4;
        let x_start = 2;
        let mut rock = rocks.get(nr%rocks.len()).unwrap().clone();
        move_rock(&mut rock, &(x_start, y_start));
        loop {
            match pattern.get(count%pattern.len()).unwrap() {
                '>' => {
                    let mut move_possible = true;
                    for (x, y) in &rock {
                        if x+1 > 6 {
                            move_possible = false;
                            break;
                        }
                        match map.get(&(x+1, *y)) {
                            Some(_) => {
                                move_possible = false;
                                break;
                            },
                            None => {},
                        }
                    }
                    if move_possible {
                        move_rock(&mut rock, &(1, 0))
                    }
                },
                '<' => {
                    // check if all parts are still inside, then move
                    let mut move_possible = true;
                    for (x, y) in &rock {
                        if x-1 < 0 {
                            move_possible = false;
                            break;
                        }
                        match map.get(&(x-1, *y)) {
                            Some(_) => {
                                move_possible = false;
                                break;
                            },
                            None => {},
                        }
                    }
                    if move_possible {
                        move_rock(&mut rock, &(-1, 0))
                    }
                }
                _ => {panic!();},
            }
            // go to next index in jet_pattern
            count += 1;

            // check if free under, then move
            let mut free_under = true;
            for (x, y) in &rock {
                match map.get(&(*x, y-1)) {
                    Some(_) => {free_under = false;},
                    None => {},
                }
            }
            if free_under {
                move_rock(&mut rock, &(0, -1));
            } else {
                
                // new_rock
                let mut covered_y = HashSet::new();
                for (x, y) in rock {
                    map.insert((x, y));
                    covered_y.insert(y);
                    
                }
                // if the row is full, remove all under
                'new_y: for y in covered_y {
                    for x in 0..7 {
                        match map.get(&(x, y)) {
                            Some(_) => {},
                            None => {
                                continue 'new_y;
                            },
                        }
                    }
                    //print_2d_map(&map);
                    map.retain(|(_, b)| b>= &y);
                    let mut new_map = HashSet::new();
                    for (a, b) in map {
                        new_map.insert((a, b-y));
                    }
                    score += y;
                    map = new_map;
                    //print_2d_map(&map);
                }
                continue 'new_rock;
            }
        }
    }
    print_2d_map(&map);
    score += get_y_max(&map);
    return score
}

fn run_until_repeated_n_times(input: &str, jet_pattern: & Vec<char>, n_times: usize) -> (i64, usize){
    let mut score = 0;

    let mut map = HashSet::new();
    for x in 0..7 {
        map.insert((x, 0));
    }
    let rocks = parse_rocks();

    let mut n_repeats = 0;
    let mut count = 0;
    'new_rock: for rock_number in 0..10000000 {
        let y_start = get_y_max(&map)+ 4;
        let x_start = 2;
        
        let mut rock = rocks.get(rock_number%rocks.len()).unwrap().clone();
        move_rock(&mut rock, &(x_start, y_start));
        loop {
            match jet_pattern.get(count%jet_pattern.len()).unwrap() {
                '>' => {
                    let mut move_possible = true;
                    for (x, y) in &rock {
                        if x+1 > 6 {
                            move_possible = false;
                            break;
                        }
                        match map.get(&(x+1, *y)) {
                            Some(_) => {
                                move_possible = false;
                                break;
                            },
                            None => {},
                        }
                    }
                    if move_possible {
                        move_rock(&mut rock, &(1, 0))
                    }
                },
                '<' => {
                    // check if all parts are still inside, then move
                    let mut move_possible = true;
                    for (x, y) in &rock {
                        if x-1 < 0 {
                            move_possible = false;
                            break;
                        }
                        match map.get(&(x-1, *y)) {
                            Some(_) => {
                                move_possible = false;
                                break;
                            },
                            None => {},
                        }
                    }
                    if move_possible {
                        move_rock(&mut rock, &(-1, 0))
                    }
                }
                _ => {panic!();},
            }
            // go to next index in jet_pattern
            count += 1;

            // check if free under, then move
            let mut free_under = true;
            for (x, y) in &rock {
                match map.get(&(*x, y-1)) {
                    Some(_) => {free_under = false;},
                    None => {},
                }
            }
            if free_under {
                move_rock(&mut rock, &(0, -1));
            } else {
                let mut covered_y = HashSet::new();
                for (x, y) in rock {
                    map.insert((x, y));
                    covered_y.insert(y);
                    
                }
                if rock_number%(jet_pattern.len()) == 0 && rock_number%(rocks.len()) == 0 {
                    //println!("score: {} iteration: {}", score, rock_number);
                    
                    if n_repeats == n_times {
                        print_2d_map(&map);
                        score = get_y_max(&map);
                        return (score, rock_number);
                    }
                    n_repeats += 1;
                }
                continue 'new_rock;
            }
        }
    }
    panic!()
}



fn main(){
    // Tetris
    let mut score_a = 0;
    let mut score_b = 0;
    let input = include_str!("example.txt");
    let mut data = input.trim().split("\r\n");
    let line = data.next().unwrap();
    let jet_pattern: Vec<char> = line.chars().collect();

    for i in 1..10 {
        let (score_1, nr1) = run_until_repeated_n_times(input, &jet_pattern, 1);
        let (score_2, nr2) = run_until_repeated_n_times(input, &jet_pattern,1+i);
        let (score_3, nr3) = run_until_repeated_n_times(input, &jet_pattern,1+i*10);
        println!("score_1: {} \nnr1: {}", score_1, nr1);
        println!("score_2: {} \nnr2: {}", score_2, nr2);
        println!("score_2: {} \nnr3: {}", score_3, nr3);
        
        if (score_3-score_2)/(9 *i as i64) == score_2 {
            println!("added iters {}", (i));
        }
    }
    
    //assert!((score_2-score_1)==(score_3-score_2));
//
    //let jet_len = jet_pattern.len();
    //// 1 - 67
    //// 2 - 126
    //// 3 - 185
    //println!("added iters {}", (nr2-nr1));
    //let n_times = (1000000000000-nr1)/(nr2-nr1);
    //let remains = (1000000000000-nr1)%(nr2-nr1);
    //println!("n_times: {} \nremains: {}", n_times, remains);
//
    //let score_remain = part_a(input, &jet_pattern, remains);
    //let score_b = score_1 + (score_2-score_1)*n_times as i64 + score_remain;
    //println!();
    ////let score_b = part_b(input);
    //println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: i64 = 3068;
    const SOLVE_A: i64 = 472;

    const EXAMPLE_B: i64 = 29;
    const SOLVE_B: i64 = 465;

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