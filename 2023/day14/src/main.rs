use std::collections::{HashMap, HashSet};

fn print_map(map: &Vec<Vec<char>>) {
    println!();
    for row in map {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn score_map(map: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for (y, row) in map.iter().enumerate() {
        let factor = map.len()-y;
        score += row.iter().filter(|&&c| c =='O').count()*factor;
    }
    score
}

fn order_vector(row: &Vec<char>) -> Vec<char> {
    let mut ordered_row = row.clone();
    while ordered_row.windows(2).any(|window| window == ['.', 'O']) {
        for idx in 1..row.len() {
            if ordered_row[idx] == 'O' && ordered_row[idx - 1] == '.' {
                ordered_row.swap(idx, idx - 1);
            }
        }
    }
    return ordered_row;
}
fn tilt_north(grid: &mut Vec<Vec<char>>){
    let mut new_map = grid.clone();
    for y in 0..grid[0].len() {
        let mut row = Vec::new();
        for x in 0..grid.len() {
            row.push(grid[x][y]);
        }
        let ordered_row = order_vector(&row);
        
        for x in 0..grid.len() {
            new_map[x][y] = ordered_row[x]
        }
    }
    *grid = new_map;
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let mut new_map = grid.clone();
    for y in 0..grid[0].len() {
        let mut row = Vec::new();
        for x in 0..grid.len() {
            row.push(grid[x][y]);
        }
        row.reverse();
        let mut ordered_row = order_vector(&row);
        ordered_row.reverse();
        for x in 0..grid.len() {
            new_map[x][y] = ordered_row[x]
        }
    }
    *grid = new_map;
}

fn tilt_west(grid: & mut Vec<Vec<char>>){
    for row in grid {
        let mut ordered_row = order_vector(row);
        *row = ordered_row
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>){
    for row in grid.iter_mut() {
        row.reverse();
        let mut ordered_row = order_vector(&row);
        ordered_row.reverse();
        *row = ordered_row
    }
}




fn part_a(input: &str) -> usize {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        grid.push(row);
    }
    tilt_north(&mut grid);
    let answer = score_map(&grid);
    return answer;
}

fn part_b(input: &str) -> usize {
    let num_cycles = 1000000000; 
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        grid.push(row);
    }
    let mut seen_states = HashMap::new();

    let mut pattern_start = 0;
    let mut pattern_len = 0;
    for it in 1..num_cycles {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
        if let Some(&first_occurrence) = seen_states.get(&grid) {
            pattern_start = first_occurrence;
            pattern_len = it - first_occurrence;
            println!("first_occurrence: {}", first_occurrence);
            println!("Repeated pattern detected at iteration: {}", it);
            println!("Length of the repeating pattern: {}", pattern_len);
            break;
        }
        seen_states.insert(grid.clone(), it);
    }
    let remainig = (num_cycles-pattern_start + pattern_len)%pattern_len;
    println!("remaingin  {}", remainig);
    for it in 0..remainig {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
    }
    let answer = score_map(&grid);
    return answer;
}

fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {}", ans_a);
    let ans_b = part_b(input);
    println!("Part B: {}", ans_b); // 87683 low
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), 136);
    }
    #[test]
    fn test_b() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), 64);
    }
}