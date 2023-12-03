use std::collections::HashMap;


fn part_a(input: &str) -> i32 {
    let mut score = 0;

    let mut map: HashMap<(i32, i32), char> = HashMap::new();

    let mut len_row = 0;
    let mut len_col = 0;
    for line in input.split("\r\n") {
        len_col = 0;
        for ch in line.chars() {
            map.insert((len_row as i32, len_col as i32), ch);
            len_col += 1;
        }
        len_row += 1;
    }
    for row in 0..len_row {
        let mut number_candiate = String::from("");
        for col in 0..len_col {
            let entry = map.get(&(row, col)).unwrap();
            if entry.is_numeric() {
                number_candiate.push(*entry);
            } else if number_candiate.len() > 0 {
                
                // number has ended, check neighbors and add the score
                let part_number: i32 = number_candiate.parse().unwrap();
                if check_neighbours(&map, row, col, part_number) {
                    //println!("{}", part_number);
                    score += part_number; 
                }
                number_candiate = String::from("");
            }
        }
        // if we reach the end of the row and have an ongoing number we should try it
        if number_candiate.len() > 0 {
            let part_number: i32 = number_candiate.parse().unwrap();
            if check_neighbours(&map, row, len_col, part_number) {
                score += part_number; 
            }
        }
    }
    return score;
}

fn check_neighbours(map: &HashMap<(i32, i32), char>, row: i32, col: i32, number: i32) -> bool {
    //println!("checking {}", number);
    let number_of_digits = number.to_string().len();
    for this_row in row-1..=row+1 {
        for this_col in col-1-number_of_digits as i32..=col {
            //println!("row {}, col{}", this_row, this_col);
            match map.get(&(this_row, this_col)) {
                Some(ch) => {
                    if ch.is_numeric() || *ch == '.' {
                        continue;
                    } else {
                        return true;
                    }
                },
                None => {continue},
            }
        }
    }
    return false;
}

fn part_b(input: &str) -> i32 {
    let mut score = 0;

    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut gear_map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    let mut len_row = 0;
    let mut len_col = 0;
    for line in input.split("\r\n") {
        len_col = 0;
        for ch in line.chars() {
            map.insert((len_row as i32, len_col as i32), ch);
            gear_map.insert((len_row, len_col), Vec::new());
            len_col += 1;
        }
        len_row += 1;
    }
    for row in 0..len_row {
        let mut number_candiate = String::from("");
        for col in 0..len_col {
            let entry = map.get(&(row, col)).unwrap();
            if entry.is_numeric() {
                number_candiate.push(*entry);
            } else if number_candiate.len() > 0 {
                
                // number has ended, check neighbors and add the score
                let part_number: i32 = number_candiate.parse().unwrap();
                find_adjacent_numbers(&map, &mut gear_map, row, col, part_number);
                number_candiate = String::from("");
            }
        }
        // if we reach the end of the row and have an ongoing number we should try it
        if number_candiate.len() > 0 {
            let part_number: i32 = number_candiate.parse().unwrap();
            find_adjacent_numbers(&map, &mut gear_map, row, len_col, part_number);
        }
    }

    for vec in gear_map.values(){
        if vec.len() == 2 {
            score += vec[0] *vec[1];
        }
    }
    return score;
}

fn find_adjacent_numbers(map: &HashMap<(i32, i32), char>, gear_map: &mut HashMap<(i32, i32), Vec<i32>>, row: i32, col: i32, number: i32)-> bool{
    //println!("checking {}", number);
    let number_of_digits = number.to_string().len();
    for this_row in row-1..=row+1 {
        for this_col in col-1-number_of_digits as i32..=col {
            match map.get(&(this_row, this_col)) {
                Some(ch) => {
                    if *ch == '*' {
                        println!("number {}", number);
                        let mut v = gear_map.get_mut(&(this_row, this_col)).unwrap();
                        v.push(number)
                    }
                },
                None => {continue},
            }
        }
    }
    return false;
}

fn main() {
    
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {:?}", ans_a);
    // 511086 too low, 513648 high
    
    let ans_b = part_b(input);
    println!("Part B: {:?}", ans_b);
}
