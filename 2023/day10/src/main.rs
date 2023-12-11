use std::{collections::{HashMap, HashSet}, f32::consts::E};



fn part_a(input: &str) -> i64 {
    let mut score = 0;
    let lines = input.trim().split("\r\n");
    let mut map = HashMap::new();

    let moves: HashMap<char, Vec<(i32, i32)>> = [
        ('F', vec![(1, 0), (0, 1)]),
        ('J', vec![(-1, 0), (0, -1)]),
        ('|', vec![(0, 1), (0, -1)]),
        ('7', vec![(-1, 0), (0, 1)]),
        ('L', vec![(1, 0), (0, -1)]),
        ('-', vec![(-1, 0), (1, 0)]),
        ('.', vec![(0, 0)]),
        ('S', vec![(1, 0), (-1, 0), (0, 1), (0, -1)]),
    ].iter().cloned().collect();
    
    let mut rows = 0;
    let mut cols = 0;
    let mut pos: (i32, i32) = (0, 0);
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), char);
            if char == 'S' {
                pos = (x as i32, y as i32);
            }
            rows = x;
        }
        cols = y;
    }

    let mut visited = HashSet::new();
    visited.insert(pos.clone());
    let mut translation = (1, 0); // TODO: Fix
    loop {
        score += 1;
        pos = (pos.0 + translation.0, pos.1 + translation.1);
        let next_move = match map.get(&pos) {
            Some(ch) => {
                if *ch == 'S' {
                    // loop complete
                    break
                }
                ch
            },
            None => {panic!()},
        };
        let next_pos = moves.get(&next_move).unwrap();

        if translation == (-next_pos[0].0, -next_pos[0].1) {
            translation = next_pos[1]
        } else if translation == (-next_pos[1].0, -next_pos[1].1) {
            translation = next_pos[0]
        } else {
            // move here not possible
            panic!()
        }

        if visited.contains(&pos) {
            // loop found
            panic!();
        }
        visited.insert(pos.clone());
    }

    
    // insta flip '|'
    // S= L
    let up = vec!['J', 'L', 'S'];
    let down = vec!['F', '7'];
    let mut score_b = 0;
    for y in 0..=cols {
        let mut is_inside = false; // each row we start outside
        let mut state = 0;
        for x in 0..=rows {
            //print!("{}", map.get(&(x as i32, y as i32)).unwrap());
            if visited.contains(&(x as i32, y as i32)) {
                let char = map.get(&(x as i32, y as i32)).unwrap();
                if up.contains(char) {
                    if state == -1 {
                        is_inside = !is_inside;
                    } else {
                        state = 1;
                    }
                    
                } else if down.contains(char) {
                    if state == 1 {
                        is_inside = !is_inside;
                    } else {
                        state = -1;
                    }
                } else if *char == '|'{
                    is_inside = !is_inside;
                }
            } else if is_inside {
                score_b += 1;
            }
        }
    }

    println!("score_b {}", score_b);
    
    return score/2
}


fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {:?}", ans_a);
    // 6886

}
