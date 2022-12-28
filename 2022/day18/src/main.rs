use std::collections::{btree_map::Values, VecDeque};


fn part_a(input: &str) -> usize {
    let mut score = 0;

    let mut x_min = usize::MAX;
    let mut x_max = usize::MIN;
    let mut y_min = usize::MAX;
    let mut y_max = usize::MIN;
    let mut z_min = usize::MAX;
    let mut z_max = usize::MIN;

    let mut crystal: Vec<(usize, usize, usize)> = Vec::new();
    for line in input.split("\r\n") {
        let mut iter = line.split(",");
        let position: (usize, usize, usize) = (
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap()
        );
        x_max = x_max.max(position.0);
        x_min = x_min.min(position.0);
        y_max = y_max.max(position.1);
        y_min = y_min.min(position.1);
        z_max = z_max.max(position.2);
        z_min = z_min.min(position.2);
        crystal.push(position);
    }
    x_max = x_max+1;
    y_max = y_max+1;
    z_max = z_max+1;

    let mut map: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; z_max]; y_max]; x_max];
    for (x, y, z) in crystal.into_iter() {
        map[x][y][z] = true;
    }

    score = get_surfaces(&map, (x_max, y_max, z_max));

    return score;
}

fn get_surfaces(crystal: &Vec<Vec<Vec<bool>>>, size: (usize, usize, usize)) -> usize {
    let(x_max, y_max, z_max) = size;
    let mut score = 0;
    for (x, subyz) in crystal.into_iter().enumerate() {
        for (y, subz) in subyz.into_iter().enumerate() {
            for (z, val) in subz.into_iter().enumerate() {
                if *val {
                    for direction in [1, -1] {
                        let (x0, y0 ,z0) = (x as i32, y as i32, z as i32);
                        for (dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                            let (pos_x, pos_y, pos_z) = (x0+dx*direction, y0+dy*direction, z0+dz*direction);
                            if pos_x < 0 || pos_y < 0 || pos_z < 0 || pos_x >= x_max as i32|| pos_y >= y_max as i32 || pos_z >= z_max as i32 {
                                score += 1;
                                continue;
                            }
                            
                            let xu = pos_x as usize;
                            let yu = pos_y as usize;
                            let zu = pos_z as usize;

                            if !crystal[xu][yu][zu] {
                                score += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    return score;
}

fn add_outer_volume(map: &mut Vec<Vec<Vec<bool>>>, pos: &(usize, usize, usize), size: &(usize, usize, usize)) {
    let(x_max, y_max, z_max) = size;
    let (x0, y0, z0) = pos;
    map[*x0][*y0][*z0] = true;
    let mut queue = VecDeque::new();
    queue.push_back((*x0, *y0, *z0));

    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();
        for direction in [1, -1] {
            for (dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                let (x_new, y_new, z_new) = (x as i32 +dx*direction, y as i32 +dy*direction, z as i32 +dz*direction);
                
                if x_new < 0 || y_new < 0 || z_new < 0 || x_new >= *x_max as i32 || y_new >= *y_max as i32 || z_new >= *z_max as i32 {
                    continue;
                }
                let (x_new, y_new, z_new) = (x_new as usize, y_new as usize, z_new as usize);
                if !map[x_new][y_new][z_new] {
                    map[x_new][y_new][z_new] = true;
                    queue.push_back((x_new, y_new, z_new));
                }
            }
        }
    }

}

fn part_b(input: &str) -> i32 {
    let mut score = 0;

    let mut x_min = usize::MAX;
    let mut x_max = usize::MIN;
    let mut y_min = usize::MAX;
    let mut y_max = usize::MIN;
    let mut z_min = usize::MAX;
    let mut z_max = usize::MIN;

    let mut crystal: Vec<(usize, usize, usize)> = Vec::new();
    
    for line in input.split("\r\n") {
        let mut iter = line.split(",");
        let position: (usize, usize, usize) = (
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap()
        );
        x_max = x_max.max(position.0);
        x_min = x_min.min(position.0);
        y_max = y_max.max(position.1);
        y_min = y_min.min(position.1);
        z_max = z_max.max(position.2);
        z_min = z_min.min(position.2);
        crystal.push(position);
    }
    // shift
    crystal = crystal.iter().map(|(x, y, z)| (x+1, y+1, z+1)).collect();

    x_max = x_max+2;
    y_max = y_max+2;
    z_max = z_max+2;

    let mut map: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; z_max]; y_max]; x_max];
    for (x, y, z) in crystal.into_iter() {
        map[x][y][z] = true;
    }

    let outer = x_max*y_max*2 + x_max*z_max*2 + y_max*z_max*2;
    let outer = outer as i32;
    
    let original_score = get_surfaces(&map, (x_max, y_max, z_max));

    let pos = (0, 0, 0);
    add_outer_volume(&mut map, &pos, &(x_max, y_max, z_max));

    let complemented_score = get_surfaces(&map, (x_max, y_max, z_max)) as i32;
    score = original_score as i32 - (complemented_score - outer); 
    // 2463 low
    // high 2992
    return score;
}


fn main() {
    let input = include_str!("input.txt");
    
    let score_a = part_a(input);
    println!("Score A: {}", score_a);

    let score_b = part_b(input);
    println!("Score B: {}", score_b);
}
