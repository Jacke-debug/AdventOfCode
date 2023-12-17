use std::{collections::{HashMap, HashSet}, arch::x86_64, vec};


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
struct state {
    pos: (usize, usize),
    dir: (i32, i32),
    consequitive: usize,
    cost: usize,
}

fn part_a(input: &str) -> usize {
    let mut map = Vec::new();
    for line in input.lines() {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        map.push(row);
    }
    print_map(&map);

    let state = state{
        pos: (0, 0),
        dir: (0, 0),
        consequitive: 0,
        cost: 0,
    };

    let mut solutions: HashMap<(usize, usize), Vec<state>> = HashMap::new();
    walk_map(&map, & state, &mut solutions);
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

fn walk_map(map: & Vec<Vec<u32>>, start_state: & state, solutions: &mut HashMap<(usize, usize), Vec<state>>) {
    let mut stack = vec![start_state];

    while let Some(this_state) = stack.pop() {
        let y_max = map.len()-1;
        let x_max = map.get(0).unwrap().len()-1;

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
                            return; // better way to get to equivalaent state
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
            return; // target reached
        }
        let directions: HashSet<_> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)].into_iter().collect();

        let mut forbidden_moves = HashSet::new();
        forbidden_moves.insert((this_state.dir.0*-1, this_state.dir.1*-1));  // can't go back
        if this_state.consequitive == 2 {
            forbidden_moves.insert(this_state.dir);
        }
        for new_dir in directions.difference(&forbidden_moves) {
            let mut new_state = this_state.clone();

            if *new_dir == this_state.dir {
                new_state.consequitive += 1;
            } else {
                new_state.consequitive = 0;
            }

            let new_x = (this_state.pos.1 as i32 + new_dir.1) as usize;
            let new_y = (this_state.pos.0 as i32 + new_dir.0) as usize;

            match map.get(new_y ) {
                Some(row) => {
                    match row.get(new_x) {
                        Some(c) => new_state.cost += *c as usize,
                        None => continue,
                    }                
                }
                None => {
                    continue // walking outisde map not possible
                }  
            }
            new_state.pos = (new_y , new_x);
            new_state.dir = *new_dir;
            //walk_map(map, &mut new_state, solutions)
            stack.push(new_state);
        }
    }
}


fn part_b(input: &str) -> usize {
    
    return 0;
}

fn main() {

    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {}", ans_a);
    let ans_b = part_b(input);
    println!("Part B: {}", ans_b); 
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), 102);
    }
    //#[test]
    fn test_b_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), 51);
    }
}