use std::collections::HashMap;


fn main() {
    let mut state = HashMap::new();
    let input = include_str!("input.txt");

    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.trim().split("\r\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            state.insert((x, y), c);
            max_x = x;
            max_y = y;
        }
    }

    let mut steps = 0;
    loop {
        let mut has_moved = false;
        // move east
        let old_state = state.clone();
        for ((x, y), cucumber) in old_state.iter() {
            if *cucumber == '>' {
                if let Some(next) = old_state.get(&(*x+1, *y)) {
                    if *next == '.' {
                        state.insert((*x, *y), '.');
                        state.insert((*x+1, *y), '>');
                        has_moved = true;
                    }
                } else {
                    let next = old_state.get(&(0, *y)).unwrap();
                    if *next == '.' {
                        state.insert((*x, *y), '.');
                        state.insert((0, *y), '>');
                        has_moved = true;
                    }
                }

            }
        }
        
        let old_state = state.clone();
        // move south
        for ((x, y), cucumber) in old_state.iter() {
            if *cucumber == 'v' {
                if let Some(next) = old_state.get(&(*x, *y+1)) {
                    if *next == '.' {
                        state.insert((*x, *y), '.');
                        state.insert((*x, *y+1), 'v');
                        has_moved = true;
                    }
                } else {
                    let next = old_state.get(&(*x, 0)).unwrap();
                    if *next == '.' {
                        state.insert((*x, *y), '.');
                        state.insert((*x, 0), 'v');
                        has_moved = true;
                    }
                }

            }
        }
        steps += 1;
        if !has_moved {
            println!("{}", steps);
            break
        }
    }

}

