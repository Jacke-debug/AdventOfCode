
fn part_a(input: &str) -> i32 {
    let mut score = 0;
    let mut crystals: Vec<(i32, i32, i32)> = Vec::new();

    for line in input.split("\n") {
        let mut iter = line.split(",");
        let position: (i32, i32, i32) = (
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap()
        );
        crystals.push(position);
    }

    score = get_surfaces(&crystals);

    return score;
}

fn get_surfaces(crystal: &Vec<(i32, i32, i32)>) -> i32 {
    let mut score = 0;
    for (x, y, z) in crystal.iter() {
        for direction in [1, -1] {
            for (dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                let neighbour = (x+dx*direction, y+dy*direction, z+dz*direction);
                if !crystal.contains(&neighbour) {
                    score += 1;
                }
            }
        }
    }
    return score;
}

#[inline(always)]
fn add_outer_volume(map: &mut Vec<(i32, i32, i32)>, pos: (i32, i32, i32)) {
    map.push(pos);
    for direction in [1, -1] {
        for (dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            let (x, y, z) = (pos.0+dx*direction, pos.1+dy*direction, pos.2+dz*direction);
            if x < 1 || y < 1 || z < 1 || x >= 5 || y >= 5 || z >= 8 {
                continue;
            }
            if !map.contains(&(x, y, z)) {
                add_outer_volume(map, (x, y, z)); 
            }
        }
    }

}

fn part_b(input: &str) -> i32 {
    let mut score = 0;
    let mut crystals: Vec<(i32, i32, i32)> = Vec::new();

    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    let mut z_min = i32::MAX;
    let mut z_max = i32::MIN;
    for line in input.split("\n") {
        let mut iter = line.split(",");
        let position: (i32, i32, i32) = (
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
        crystals.push(position);
    }
    x_max = x_max+2;
    y_max = y_max+2;
    z_max = z_max+2;
    let x_size = x_max-x_min;
    let y_size = y_max-y_min;
    let z_size = z_max-z_min;
    println!("{:?}", (x_min, x_max, y_min, y_max, z_min, z_max));
    let outer = x_size*y_size*2 + x_size*z_size*2 + y_size*z_size*2;
    println!("{}", outer);

    crystals = crystals.iter().map(|(x, y, z)| (x+1, y+1, z+1)).collect();
    
    let original_score = get_surfaces(&crystals);

    let pos = (x_min, y_min, z_min);
    add_outer_volume(&mut crystals, pos);
   
    score = original_score - (get_surfaces(&crystals) - outer); 
    // 2463 low
    // high 2992
    return score;
}


fn main() {
    let input = include_str!("example.txt");
    let score_a = part_a(input);
    let score_b = part_b(input);
    println!("Score A: {}", score_a);
    println!("Score B: {}", score_b);
}
