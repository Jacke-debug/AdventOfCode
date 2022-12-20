use std::{collections::{HashMap, HashSet}, ops::RangeInclusive};

fn manhattan_distance(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
    return (x1-x2).abs() + (y1-y2).abs();
}

fn part_a(input: &str) -> usize {
    let mut data = input.trim().split("\n");
    let mut score = 0;

    let mut map = HashMap::new();
    let mut beacon_map = HashSet::new();

    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut range_max = 0;
    while let Some(line) = data.next() {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let (first, second) = line.split_once(',').unwrap();
        let x_pos: i64 = first.split_once('=').unwrap().1.parse().unwrap();
        
        let (y_str, third) = second.split_once(':').unwrap();
        let y_pos: i64 = y_str.split_once('=').unwrap().1.parse().unwrap();
        println!("pos: {} {}", x_pos, y_pos);

        let (beacon_x, last) = third.split_once(',').unwrap();
        let beacon_x: i64 = beacon_x.split_once('=').unwrap().1.parse().unwrap();

        let beacon_y: i64 = last.split_once('=').unwrap().1.parse().unwrap();
        println!("beacon: {} {}", beacon_x, beacon_y);

        beacon_map.insert((beacon_x, beacon_y));

        let range = manhattan_distance(x_pos, beacon_x, y_pos, beacon_y);
        map.insert((x_pos, y_pos), range);
        x_min = x_min.min(x_pos).min(beacon_x);
        x_max = x_max.max(x_pos).max(beacon_x);
        y_min = y_min.min(y_pos).min(beacon_y);
        y_max = y_max.max(y_pos).max(beacon_y);
        range_max = range_max.max(range)
    }

    //print_map(&map, x_min, x_max, y_min, y_max);

    let mut covered_point = HashSet::new();
    // distance to y=10
    // start from 0 go positive, then go negative?
    let y_pos = 2000000;
    let mut pos_plus = -1;
    'outer: while pos_plus <  x_max.max(x_min.abs()) + range_max {
        pos_plus += 1;
        let pos_minus = 0-pos_plus;
        'pos: for (sensor, range) in map.iter() {
            if beacon_map.contains(&(pos_plus, y_pos)) {
                break 'pos;
            }
            if manhattan_distance(pos_plus, sensor.0, y_pos, sensor.1) <= *range {
                covered_point.insert((pos_plus, y_pos));
                break 'pos;
            }
        }

        'neg: for (sensor, range) in map.iter() {
            if beacon_map.contains(&(pos_minus, y_pos)) {
                break 'neg;
            }
            if manhattan_distance(pos_minus, sensor.0, y_pos, sensor.1) <= *range {
                covered_point.insert((pos_minus, y_pos));
                break 'neg;
            }
        }
    }

    score = covered_point.len();
    //println!("{:?}", covered_point);
    return score
}

fn print_map(map: &HashMap<(i64, i64), i64>, x_min: i64, x_max: i64, y_min: i64, y_max: i64) {
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if let Some(val) = map.get(&(x,y)) {
                print!("{:#02}", val);
            } else {
                print!(". ");
            }
        }
        println!("")
    }
}



fn part_b(input: &str) -> i64{
    let mut data = input.trim().split("\n");
    let mut score = 0;

    let mut map = HashMap::new();
    let mut beacon_map = HashSet::new();

    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut range_max = 0;
    while let Some(line) = data.next() {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let (first, second) = line.split_once(',').unwrap();
        let x_pos: i64 = first.split_once('=').unwrap().1.parse().unwrap();
        
        let (y_str, third) = second.split_once(':').unwrap();
        let y_pos: i64 = y_str.split_once('=').unwrap().1.parse().unwrap();

        let (beacon_x, last) = third.split_once(',').unwrap();
        let beacon_x: i64 = beacon_x.split_once('=').unwrap().1.parse().unwrap();

        let beacon_y: i64 = last.split_once('=').unwrap().1.parse().unwrap();

        beacon_map.insert((beacon_x, beacon_y));

        let range = manhattan_distance(x_pos, beacon_x, y_pos, beacon_y);
        map.insert((x_pos, y_pos), range);
        x_min = x_min.min(x_pos).min(beacon_x);
        x_max = x_max.max(x_pos).max(beacon_x);
        y_min = y_min.min(y_pos).min(beacon_y);
        y_max = y_max.max(y_pos).max(beacon_y);
        range_max = range_max.max(range)
    }

    let max = 4000000;

    'edge: for ((pos_x, pos_y), range) in map.iter() {
        let mut x = pos_x + range + 2;
        let mut y = *pos_y-1;
        'ne: while x > *pos_x {
            x -= 1;
            y += 1;
            assert!(manhattan_distance(x, *pos_x, y, *pos_y) == range +1);
            if x < 0 || y < 0 || x > max || y > max {
                continue 'ne;
            }
            
            for (sensor, r) in map.iter() {
                if manhattan_distance(x, sensor.0, y, sensor.1) < *r {
                    continue 'ne;
                }
            }
        }

        'se: while y > *pos_y {
            x -= 1;
            y -= 1;
            assert!(manhattan_distance(x, *pos_x, y, *pos_y) == range +1);
            if x < 0 || y < 0 || x > max || y > max {
                continue 'se;
            }
            
            for (sensor, r) in map.iter() {
                if manhattan_distance(x, sensor.0, y, sensor.1) < *r {
                    continue 'se;
                }
            }
            
        }

        'sw: while x < *pos_x {
            x += 1;
            y -= 1;
            assert!(manhattan_distance(x, *pos_x, y, *pos_y) == range +1);
            if x < 0 || y < 0 || x > max || y > max {
                continue 'sw;
            }
            
            for (sensor, r) in map.iter() {
                if manhattan_distance(x, sensor.0, y, sensor.1) < *r {
                    continue 'sw;
                }
            }
            
        }
        'nw: while y < *pos_y {
            x += 1;
            y += 1;
            assert!(manhattan_distance(x, *pos_x, y, *pos_y) == range +1);
            if x < 0 || y < 0 || x > max || y > max {
                continue 'nw;
            }
            
            for (sensor, r) in map.iter() {
                if manhattan_distance(x, sensor.0, y, sensor.1) < *r {
                    continue 'nw;
                }
            }
            score = x*4000000 + y;
            println!("Done {}, {}", x, y);
            break 'edge;
        }
    }
    
    
    return score
}



fn main(){
    let input = include_str!("input.txt");
    let score_a = part_a(input);
    println!();
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: i64 = 31;
    const SOLVE_A: i64 = 472;

    const EXAMPLE_B: i64 = 29;
    const SOLVE_B: i64 = 465;

    use super::*;
    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), EXAMPLE_A);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), EXAMPLE_B);
    }

    #[test]
    fn solve_a() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), SOLVE_A);
    }

    #[test]
    fn solve_b() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), SOLVE_B);
    }
}