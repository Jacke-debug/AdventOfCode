use std::{collections::{HashMap, HashSet}, ops::RangeInclusive};

fn manhattan_distance(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    return (x1-x2).abs() + (y1-y2).abs();
}

fn part_a(input: &str) -> usize {
    let mut data = input.trim().split("\r\n");
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
        let x_pos: i32 = first.split_once('=').unwrap().1.parse().unwrap();
        
        let (y_str, third) = second.split_once(':').unwrap();
        let y_pos: i32 = y_str.split_once('=').unwrap().1.parse().unwrap();
        println!("pos: {} {}", x_pos, y_pos);

        let (beacon_x, last) = third.split_once(',').unwrap();
        let beacon_x: i32 = beacon_x.split_once('=').unwrap().1.parse().unwrap();

        let beacon_y: i32 = last.split_once('=').unwrap().1.parse().unwrap();
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

fn print_map(map: &HashMap<(i32, i32), i32>, x_min: i32, x_max: i32, y_min: i32, y_max: i32) {
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



fn part_b(input: &str) -> i32{
    let mut data = input.trim().split("\r\n");
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
        let x_pos: i32 = first.split_once('=').unwrap().1.parse().unwrap();
        
        let (y_str, third) = second.split_once(':').unwrap();
        let y_pos: i32 = y_str.split_once('=').unwrap().1.parse().unwrap();
        println!("pos: {} {}", x_pos, y_pos);

        let (beacon_x, last) = third.split_once(',').unwrap();
        let beacon_x: i32 = beacon_x.split_once('=').unwrap().1.parse().unwrap();

        let beacon_y: i32 = last.split_once('=').unwrap().1.parse().unwrap();
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

    'x_loop: for x_pos in  0..=4000000 {
        println!("{}", x_pos);
        'y_loop: for y_pos in 0..=4000000 {
            'pos: for (sensor, range) in map.iter() {
                if beacon_map.contains(&(x_pos, y_pos)) {
                    continue 'y_loop;
                }
                if manhattan_distance(x_pos, sensor.0, y_pos, sensor.1) <= *range {
                    continue 'y_loop;
                }
            }

            println!("{:?} {}", x_pos, y_pos);
            break 'x_loop;
        }
    }
    
    
    return score
}



fn main(){
    let input = include_str!("input.txt");
    //let score_a = part_a(input);
    let score_a = 0;
    println!();
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: i32 = 31;
    const SOLVE_A: i32 = 472;

    const EXAMPLE_B: i32 = 29;
    const SOLVE_B: i32 = 465;

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