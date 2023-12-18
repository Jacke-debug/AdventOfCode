use std::{collections::{HashMap, HashSet, VecDeque}, vec};

fn parse_dir(c: char) -> (isize, isize) {
    match c {
        'R' => (1,0),
        'L' => (-1, 0),
        'D' => (0, 1),
        'U' => (0, -1),
        _ => panic!()
    }
}

fn parse_dig(c: &str) -> (isize, isize){
    let a = match c {
        "0" => 'R',
        "1" => 'D',
        "2" => 'L',
        "3" => 'U',
        _ => panic!()
    };

    parse_dir(a)
}

fn part_a(input: &str) -> (isize, isize) {
    let mut pos = (0, 0);
    let mut pos_b = (0, 0);
    let mut vertices = vec![pos];
    let mut circumferance = 1;
    let mut vertices_b = vec![pos_b];
    let mut circumferance_b = 1;
    for line in input.lines() {
        let mut parts = line.split_whitespace();

        let dir = parse_dir(parts.next().unwrap().chars().next().unwrap());
        let dist: isize = parts.next().unwrap().parse().unwrap();
        let color = parts.next().unwrap();
        let dist_b = &color[2..7];
        let dist_b = isize::from_str_radix(dist_b, 16).unwrap();
        let dir_b = parse_dig(&color[7..8]);
        
        for _ in 0..dist {
            pos.0 = pos.0 +dir.0;
            pos.1 = pos.1 +dir.1;
        }
        for _ in 0..dist_b {
            pos_b.0 = pos_b.0 +dir_b.0;
            pos_b.1 = pos_b.1 +dir_b.1;
        }
        vertices.push(pos);
        circumferance += dist;
        vertices_b.push(pos_b);
        circumferance_b += dist_b;
    }
    let ans_a = shoelace_area(vertices, circumferance);
    let ans_b = shoelace_area(vertices_b, circumferance_b);
    return (ans_a, ans_b)
}

fn shoelace_area(vertices: Vec<(isize, isize)>, boundary: isize) -> isize {
    let area: isize = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum();
    area.abs() / 2 + boundary / 2 + 1
}

fn main() {
    let input = include_str!("input.txt");
    let (ans_a, ans_b) = part_a(input);
    println!("Part A: {}", ans_a);
    println!("Part B: {}", ans_b); 
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), (62, 952408144115));
    }
}