use std::{collections::{HashMap, HashSet, VecDeque}, arch::x86_64, vec};


fn print_map(map: &[Vec<u32>]) {
    for row in map {
        for &cell in row {
            print!("{} ", cell);
        }
        println!(); // Move to the next line for a new row
    }
}

#[derive(Clone)]
#[derive(Debug)]
struct State {
    pos: (usize, usize),
    dir: (i32, i32),
    consequitive: usize,
    cost: usize,
}

fn part_a(input: &str, min: usize, max: usize) -> usize {
    let mut map = Vec::new();
    for line in input.lines() {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        map.push(row);
    }
    // print_map(&map);

    let start_state = State{
        pos: (0, 0),
        dir: (0, 0),
        consequitive: 0,
        cost: 0,
    };

    let mut solutions: HashMap<(usize, usize), Vec<State>> = HashMap::new();

    walk_map(&map, start_state, &mut solutions, min, max);
    let y_max = map.len()-1;
    let x_max = map.get(0).unwrap().len()-1;
    println!("val {:?}", map[y_max][x_max]);

    if let Some(vec_state) = solutions.get(&(y_max, x_max)){
        println!("{:?}", vec_state);
        if let Some(best_sol) = vec_state.iter().min_by_key(|state| state.cost){
            return best_sol.cost;
        }
    }
    return 0;
}

fn walk_map(map: & Vec<Vec<u32>>, 
    start_state: State, 
    solutions: &mut HashMap<(usize, usize), Vec<State>>,
    min_consqutive: usize, 
    max_consqutive: usize, 
) {
    let mut stack: VecDeque<State> = vec![start_state].into();
    let y_max = map.len()-1;
    let x_max: usize = map.get(0).unwrap().len()-1;

    let directions: HashSet<_> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)].into_iter().collect();
    'outer: while let Some(this_state) = stack.pop_front() {
        match solutions.get_mut(&this_state.pos) {
            Some(vec_states) => {
                let mut found_better_path = false;
                // we have been here before, was this path better?
                for st in vec_states.iter_mut() {
                    // lower cost, same dir lower consequitive?
                    if this_state.dir == st.dir {
                        if this_state.consequitive <= st.consequitive && this_state.cost <= st.cost{
                            *st = this_state.clone(); // best cost from that direciton
                            found_better_path = true;
                            break;
                        } else if this_state.consequitive >= st.consequitive && this_state.cost >= st.cost {
                            continue 'outer; // better way to get to equivalaent state
                        } else if this_state.cost >st.cost+2*9*min_consqutive as usize {
                            continue 'outer;
                        }
                    }
                }
                if !found_better_path {
                    vec_states.push(this_state.clone());
                }
            }
            None => {
                // firs time visiting, this position
                solutions.insert(this_state.pos, vec![this_state.clone()]);
            }
        }
        if this_state.pos == (y_max, x_max) {
            continue; // target reached
        }
        let mut forbidden_moves: HashSet<(i32, i32)> = HashSet::new();
        forbidden_moves.insert((this_state.dir.0*-1, this_state.dir.1*-1));  // can't go back
        if this_state.consequitive == max_consqutive {
            forbidden_moves.insert(this_state.dir);
        }
        'new_dir: for new_dir in directions.difference(&forbidden_moves) {
            let mut new_x = this_state.pos.1; 
            let mut new_y: usize = this_state.pos.0;
            let mut consequitve = 0;
            let mut added_cost = 0; 

            if *new_dir == this_state.dir {
                consequitve = this_state.consequitive + 1;
                new_x = (new_x as i32 + new_dir.1) as usize;
                new_y = (new_y as i32 + new_dir.0) as usize;
                match map.get(new_y ) {
                    Some(row) => {
                        match row.get(new_x) {
                            Some(c) => added_cost += *c as usize,
                            None => continue 'new_dir,
                        }
                    }
                    None => {
                        continue 'new_dir// walking outisde map not possible
                    }
                }
            } else {
                for i in 0..min_consqutive {
                    consequitve += 1;
                    new_x = (new_x as i32 + new_dir.1) as usize;
                    new_y = (new_y as i32 + new_dir.0) as usize;
                    match map.get(new_y ) {
                        Some(row) => {
                            match row.get(new_x) {
                                Some(c) => added_cost += *c as usize,
                                None => continue 'new_dir,
                            }
                        }
                        None => {
                            continue 'new_dir// walking outisde map not possible
                        }
                    }
                }
            }
            
            let new_state = State { 
                pos: (new_y , new_x), 
                dir: (*new_dir), 
                consequitive: (consequitve), 
                cost: (this_state.cost + added_cost) 
            };
            stack.push_back(new_state);
        }
    }
}


fn part_b(input: &str) -> usize {
    
    return 0;
}

fn main() {
    let input = include_str!("input.txt");
    //let ans_a = part_a(input, 1, 3);
    //println!("Part A: {}", ans_a);
    let ans_b = part_a(input, 4, 10);
    println!("Part B: {}", ans_b); 
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input, 1, 3), 102);
    }
    #[test]
    fn test_b_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input, 4, 10), 94);
    }
    #[test]
    fn test_b_example2() {
        let input = include_str!("example2.txt");
        assert_eq!(part_a(input, 4, 10), 71);
    }
}