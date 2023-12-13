use std::{collections::HashMap, io::empty};

fn print_map(map: &Vec<Vec<char>>) {
    println!();
    for row in map {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn transpose(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_rows = map.len();
    let num_columns = map[0].len();
    (0..num_columns)
        .map(|col_index| {
            (0..num_rows)
                .map(|row_index| map[row_index][col_index])
                .collect()
        })
        .collect()
}

fn find_mirroring_line(map: &Vec<Vec<char>>) -> Option<usize> {
    let height = map.len();
    'outer: for idx in 1..height {
        let mut pre_mirror: Vec<Vec<char>> = map[0..idx].to_vec();
        let mut post_mirror: Vec<Vec<char>> = map[idx..].to_vec();
        
        pre_mirror.reverse();
        if pre_mirror.len() <= post_mirror.len() {
            post_mirror = post_mirror[0..pre_mirror.len()].to_vec();
        } else {
            pre_mirror = pre_mirror[0..post_mirror.len()].to_vec();
        }
        if pre_mirror == post_mirror {
            return Some(idx);
        }
    }
    None
}

fn part_a(input: &str) -> usize {
    let mut answer = 0;
    let maps: Vec<&str> = input.trim().split_terminator("\r\n\r\n").collect();
    
    for map_str in maps {
        let map: Vec<Vec<char>> = map_str
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        
        match find_mirroring_line(&map) {
            Some(x) => answer+=x*100,
            None => {
                match find_mirroring_line(&transpose(&map)) {
                    Some(x) => answer+=x,
                    None => panic!(),
                }
            }
            
        }
    }
    return answer;
}

fn part_b(input: &str) -> usize {
    let mut answer = 0;
    let maps: Vec<&str> = input.trim().split_terminator("\r\n\r\n").collect();
    
    'outer: for map_str in maps {
        let map: Vec<Vec<char>> = map_str
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        for (i, row) in map.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let mut new_map = map.clone();
                match new_map[i][j] {
                    '.' => {new_map[i][j]='#'},
                    '#' => {new_map[i][j]='.'},
                    _ => panic!()
                }

                match find_mirroring_line(&map) {
                    Some(x) => answer+=x*100,
                    None => {
                        match find_mirroring_line(&transpose(&map)) {
                            Some(x) => {
                                answer+=x;
                                continue 'outer;
                            },
                            None => continue,
                        }
                    }
                    
                }

            }
        }
        
    }
    return answer;
}



fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {:?}", ans_a); // 29213
    let ans_b = part_b(input);
    println!("Part B: {:?}", ans_b);   
}
