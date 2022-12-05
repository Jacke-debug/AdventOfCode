use std::{collections::HashMap, hash::Hash};

fn main() {
    let input = include_str!("input.txt");

    let mut map = HashMap::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            max_y = y;
            max_x = x;
            let var: u32 = match c.to_digit(10) {
                Some(v) => v, 
                None => continue,
            };
            map.insert((x as i64, y as i64), var);
        }
    }

    let mut flashes = 0;
    println!("{:?}", map);
    for i in 0..99999 {
        // increase all values by 1
        for (_, val) in map.iter_mut() {
            *val +=1;
        }

        // flash all 9s
        let mut new_flash = true;
        while new_flash {
            new_flash = false;
            for y in 0..=max_y {
                for x in 0..=max_x {
                    let y = y as i64;
                    let x = x as i64;
                    let val  = map.get(&(x, y)).unwrap();
                    if *val == 10 {
                        map.insert((x, y), 0);
                        for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                            if let Some(neighbour) = map.get_mut(&(x+dx, y+dy)) {
                                if *neighbour < 10 && *neighbour != 0{
                                    *neighbour += 1;
                                    if *neighbour == 10 {
                                        new_flash = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // set all 10s to 0 and count flashes
        let mut flashes_this_round = 0;
        for (_, val) in map.iter() {
            if *val == 0 {
                flashes +=1; 
                flashes_this_round += 1;
            }
        }
        if flashes_this_round == 100 {
            println!("100 flashes at step: {}", i+1);
            break;
        }
        
        
    }
    println!("falshes: {}", flashes)
}
