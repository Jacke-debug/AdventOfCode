use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn _print_map(map: &HashMap<(isize, isize), char>, len_x: isize, len_y: isize) {
    for y in 0..len_y + 1 {
        for x in 0..len_x + 1 {
            print!("{}", map.get(&(x, y)).unwrap());
        }
        println!();
    }
    println!();
}

struct Region {
    cords: HashSet<(isize, isize)>,
    plant: char,
    area: usize,
    perimeter: usize,
    corners: usize,
}

fn explore_region(
    map: &HashMap<(isize, isize), char>,
    regions: &Vec<Region>,
    region: &mut Region,
    pos: (isize, isize),
) {
    for reg in regions.iter() {
        if reg.cords.contains(&pos) {
            return;
        }
    }
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    region.cords.insert(pos);
    for dir in dirs.iter() {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if region.cords.contains(&new_pos) {
            continue;
        }
        match map.get(&new_pos) {
            Some(n) => {
                if *n == region.plant {
                    region.area += 1;
                    explore_region(map, regions, region, new_pos);
                } else {
                    region.perimeter += 1;
                }
            }
            None => region.perimeter += 1,
        }
    }
}

fn count_corners(map: &HashMap<(isize, isize), char>, region: &mut Region) {
    let potential_corners = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    for pos in region.cords.iter() {
        for dir in potential_corners.iter() {
            let dx = pos.0 + dir.0;
            let dy = pos.1 + dir.1;
            let p_vert = map.get(&(dx, pos.1));
            let p_side = map.get(&(pos.0, dy));
            let p_diag = map.get(&(dx, dy));
            if p_diag == Some(&region.plant) {
                if (p_vert == Some(&region.plant) && p_side == Some(&region.plant))
                    || (p_vert == Some(&region.plant) || p_side == Some(&region.plant))
                {
                    continue;
                } else {
                    region.corners += 1;
                }
                continue;
            }
            if (p_vert != Some(&region.plant) && p_side != Some(&region.plant))
                || (p_vert == Some(&region.plant) && p_side == Some(&region.plant))
            {
                region.corners += 1;
            }
        }
    }
}

fn parse_map(input: &str) -> (HashMap<(isize, isize), char>, isize, isize) {
    let mut map = HashMap::new();
    let mut len_x = 0;
    let mut len_y = 0;
    for (y, line) in input.lines().enumerate() {
        len_y = y as isize;
        for (x, c) in line.chars().enumerate() {
            len_x = x as isize;
            map.insert((len_x, len_y), c);
        }
    }
    // _print_map(&map, len_x, len_y);
    (map, len_x, len_y)
}

fn identify_regions(
    map: &HashMap<(isize, isize), char>,
    len_x: isize,
    len_y: isize,
) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    for y in 0..=len_y {
        'outer: for x in 0..=len_x {
            let pos = (x, y);
            for reg in regions.iter() {
                if reg.cords.contains(&pos) {
                    continue 'outer;
                }
            }
            let c = map.get(&pos).unwrap();
            let mut region = Region {
                cords: HashSet::new(),
                area: 1,
                perimeter: 0,
                plant: *c,
                corners: 0,
            };
            explore_region(map, &regions, &mut region, pos);
            regions.push(region);
        }
    }
    regions
}
fn solve(input: &str) -> (usize, usize) {
    let (map, len_x, len_y) = parse_map(input);
    let mut regions = identify_regions(&map, len_x, len_y);

    let ans_a = regions.iter().map(|a| a.area * a.perimeter).sum();
    // Number of sides is equal to number of corners
    let ans_b: usize = regions
        .iter_mut()
        .map(|region| {
            count_corners(&map, region);
            region.corners * region.area
        })
        .sum();
    (ans_a, ans_b)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = solve(input);
    assert_eq!(ans.0, 1494342);

    let ans = solve(input);
    assert_eq!(ans.1, 893676);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a1() {
        let input = include_str!("example1.txt");
        let ans = solve(input);
        assert_eq!(ans.0, 140);
    }

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert_eq!(ans.0, 1930);
    }

    #[test]
    fn example_b1() {
        let input = include_str!("example1.txt");
        let ans = solve(input);
        assert_eq!(ans.1, 80);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert_eq!(ans.1, 1206);
    }
}
