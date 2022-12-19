use std::collections::HashSet;


fn part_a(input: &str) -> i32 {
    let mut score = 0;
    let mut crystals: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in input.split("\r\n") {
        let mut iter = line.split(",");
        let position: (i32, i32, i32) = (
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap()
        );
        crystals.insert(position);
    }

    for (x, y, z) in crystals.iter() {
        for direction in [1, -1] {
            for (dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                let neighbour = (x+dx*direction, y+dy*direction, z+dz*direction);
                match crystals.get(&neighbour) {
                    Some(_) => {},
                    None => {score += 1;},
                }
            }
        }
    }
    return score;
}

fn check_neighbour(cystal: &HashSet<(i32, i32, i32)>, checked: &mut HashSet<(i32, i32, i32)>, pos: (i32, i32, i32)) -> bool {
    let size = cystal.iter().map(|(x, y, z)| x.max(y.max(z))).max().unwrap();

    let (x, y, z) = pos;

    for dir in [1, -1] {
        'dirs: for (dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            let neighbour = (x+dx*dir, y+dy*dir, z+dz*dir);
            if neighbour.0 == *size || neighbour.0 == 0 ||neighbour.1 == *size || neighbour.1 == 0 || neighbour.2 == *size || neighbour.2 == 0 {
                return false;
            }
            if let Some(_) = checked.get(&neighbour) {
                continue 'dirs;
            }
            match cystal.get(&neighbour) {
                Some(_) => {
                    // no empty space on this side. 
                    continue 'dirs;
                },
                None => {
                    // empty space, check if
                    checked.insert(neighbour);
                    return check_neighbour(cystal, checked, neighbour);
                },
            }
        }        
    }
    return true;
}

fn part_b(input: &str) -> i32 {
    let mut score = 0;
    let mut crystals: HashSet<(i32, i32, i32)> = HashSet::new();

    let mut x_min = 0;
    let mut x_max = 0;
    for line in input.split("\r\n") {
        let mut iter = line.split(",");
        let position: (i32, i32, i32) = (
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap(),
            iter.next().and_then(|s| s.parse().ok()).unwrap()
        );
        x_max = x_max.max(position.0);
        x_min = x_min.min(position.0);
        crystals.insert(position);
    }

    
    let mut interla_points: Vec<(i32, i32, i32)> = Vec::new();
    for (x, y, z) in crystals.iter() {
        for direction in [1, -1] {
            for (dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                let neighbour = (x+dx*direction, y+dy*direction, z+dz*direction);
                let mut checked: HashSet<(i32, i32, i32)> = HashSet::new();
                match crystals.get(&neighbour) {
                    Some(_) => {
                        
                    },
                    None => {
                        if check_neighbour(&crystals, &mut checked, neighbour) {
                            interla_points.push((neighbour));
                        }                     
                    },
                }
                
            }
        }
    }
    for point in interla_points {
        println!("Internal {:?}", point);
        crystals.insert(point);
    }

    // fill voids?
    let mut checked: HashSet<(i32, i32, i32)> = HashSet::new();
    for (x, y, z) in crystals.iter() {
        for direction in [1, -1] {
            for (dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                let neighbour = (x+dx*direction, y+dy*direction, z+dz*direction);
                match crystals.get(&neighbour) {
                    Some(_) => {
                        
                    },
                    None => {
                        // check if point is internal?
                        score += 1;                        
                    },
                }
            }
        }
    }
    // 2463 low
    return score;
}


fn main() {
    let input = include_str!("example.txt");
    let score_a = part_a(input);
    let score_b = part_b(input);
    println!("Score A: {}", score_a);
    println!("Score B: {}", score_b);
}
