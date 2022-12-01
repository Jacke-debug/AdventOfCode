use core::panic;
use std::{collections::{HashSet}, time::Instant};

#[derive(Debug, Clone, Copy)]
struct Pod {
    id: i32,
    art: i32,
    done: bool,
    depth: i32,
    x_pos: i32,
}

fn pos_to_state(x: i32) -> i32 {
    3 + x*2 // 0 -> 3, 1 -> 5
}

fn art_to_num(c: char) -> i32 {
    if c == 'A' {
        0
    } else if c == 'B' {
        1
    } else if c == 'C' {
        2
    } else {
        3
    }
}

fn pos_is_final(pod: & Pod, state: & Vec<Pod>) -> bool {
    let pod_in_right_column = pod.x_pos == pos_to_state(pod.art); 
    if pod_in_right_column {
        let mut is_final = true;
        let pods_below: Vec<&Pod> = state.into_iter().filter(|p| p.depth < pod.depth && p.x_pos == pod.x_pos).collect();
        for p2 in pods_below {
            if p2.x_pos != pos_to_state(p2.art) {
                is_final = false;
            }
        }
        return is_final
    }
    return false;
}

fn free_to_move(pod: Pod, desired_x_pos: i32, state: & Vec<Pod>) -> bool{
    let empty_corridor: HashSet<i32> = [1, 2, 4, 6, 8, 10, 11].iter().cloned().collect();
    let free_spots_in_corridor = get_free_spots_in_corridor(state);
    let mut spots_that_must_be_free: HashSet<i32> = HashSet::new();
    if pod.x_pos-desired_x_pos > 0 {
        for idx in desired_x_pos+1..=pod.x_pos-1 {
            if empty_corridor.contains(&idx) {
                spots_that_must_be_free.insert(idx);
            }
        }
    } else {
        for idx in pod.x_pos+1..=desired_x_pos-1 {
            if empty_corridor.contains(&idx) {
                spots_that_must_be_free.insert(idx);
            }
        }
    }
    spots_that_must_be_free.is_subset(&free_spots_in_corridor)
}

#[derive(Debug, Clone, Copy)]
struct Motion {
    pod_id: i32,
    finishing: bool, 
    x_pos: i32, 
    depth: i32,
}

fn get_free_spots_in_corridor(state: &Vec<Pod>) -> HashSet<i32> {
    let corridor: HashSet<i32> = [1, 2, 4, 6, 8, 10, 11].iter().cloned().collect();
    let pos_in_corridor = state.iter().filter(|p| p.depth == 0).map(|p| p.x_pos);
        let mut takes_spots: HashSet<i32> = HashSet::new();
        for pos_in_corridor in pos_in_corridor {
            takes_spots.insert(pos_in_corridor);
        }
    &corridor - &takes_spots
}

fn can_pod_move(pod: & Pod, state: & Vec<Pod>) -> Vec<Motion> {
    // assume pod.done = false 
    let mut possible_motions = Vec::new();
    
    if pod.depth == 0 {
        // pod is in corridor
        let desired_x_pos = pos_to_state(pod.art);
        let pods_in_room_that_should_move = state.into_iter().filter(|p| p.x_pos == desired_x_pos && !p.done).count();
        if pods_in_room_that_should_move != 0 {
            return possible_motions // can't move to room
        }
        
        let pods_in_room = state.iter().filter(|p| p.x_pos == desired_x_pos && p.done).count();
        if free_to_move(*pod, desired_x_pos, state) {
            let motion = Motion { pod_id: pod.id, finishing: true, x_pos: desired_x_pos, depth: (pods_in_room as i32) -4 };
            possible_motions.push(motion); 
            return possible_motions
        }
    } else {
        let mut pod_above = state.iter().filter(|p| p.x_pos == pod.x_pos && p.depth > pod.depth);
        
        if let Some(pod_above) = pod_above.next() {
            return possible_motions;    // pod can't move
        }
        let free_spots_in_corridor = get_free_spots_in_corridor(state);
        for spot in free_spots_in_corridor.iter() {
            if free_to_move(*pod, *spot, state) {
                let motion = Motion{  pod_id: pod.id, finishing: false, x_pos: *spot, depth: 0};
                possible_motions.push(motion); 
            }
        }
    }
    return possible_motions;
}

fn is_final_state(state: & Vec<Pod>) -> bool {
    for pod in state {
        if !pod.done {
            return false;
        } 
    }
    true
}

fn make_move(mv: Motion, state: & Vec<Pod>, cost: i32, lowest_cost: &mut i32) {
    let mut new_state = state.clone();
    let mut pod = new_state.iter_mut()
        .filter(|p| p.id == mv.pod_id).next().unwrap();
    
    let mut this_cost = cost;
    let dy = (pod.depth - mv.depth).abs();
    let dx = (pod.x_pos - mv.x_pos).abs();
    
    this_cost += 10_i32.pow(pod.art as u32) * (dx + dy);
    if this_cost > *lowest_cost {
        return
    }
    // println!("moving: {}, x_pos: {}, depth: {}", id, mv.x_pos, mv.depth);
    pod.x_pos = mv.x_pos;
    pod.depth = mv.depth;
    pod.done = pos_is_final(pod, &state);
    //println!("Checking if fone \n {:#?}", new_state);
    if is_final_state(&new_state) {
        *lowest_cost = *lowest_cost.min(&mut this_cost);
    } else {
        check_possible_moves(& new_state, this_cost, lowest_cost);
    }
}

fn check_possible_moves(state: & Vec<Pod>, cost: i32, lowest_cost: &mut i32){
    let mut moves = Vec::new();
    
    for pod in state.iter().filter(|p| !p.done) {
        let moves_ = can_pod_move(pod, &state);
        for movv in moves_ {
            moves.push(movv);
        }
    }

    if let Some(finishing_move) = moves.iter().filter(|p| p.finishing).next() {
        make_move(*finishing_move, state, cost, lowest_cost);
    } else {
        // println!("id: {}, {:?}", id, moves);
        for mv in moves.iter() {
            make_move(*mv, state, cost, lowest_cost);
        }
    }
}
fn main() {
    let start = Instant::now();
    // BCBD 
    // ADCA
    let mut state = Vec::new();
    let input = include_str!("partb.txt");
    let mut id = 0;
    for (y, line) in input.trim().split("\r\n").enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let depth = 0-(y as i32 +1);
            let pod = Pod {id: id, art: art_to_num(c), done: false, depth: depth, x_pos: pos_to_state(x as i32)};
            state.push(pod);
            id += 1;
        }
    }

    // update state

    let prev_state = state.clone();

    for pod in state.iter_mut() {
        pod.done = pos_is_final(pod, & prev_state);
    }

    let mut lowest_cost = i32::MAX;
    let cost = 0;
    check_possible_moves(&mut state, cost, &mut lowest_cost); 

    // println!("{:?}", cost_vec);
    println!("{:?}", lowest_cost);
    // 14350 too low

    // with depth 4: 16763

    let end = Instant::now();
    println!("{:?}", end-start);


    // define state
    // search all next state
        // loop through all pods and see where they can move
        // Extra: if one can move to Final densitnation. Prioritze that. 
        // check if amphipod is satisfied (can it still move)
        // check if there is a blocker
        // compute amout of steps needed
    // determine if end-state
}
