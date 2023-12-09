use std::collections::{HashMap, HashSet};



fn part_a(input: &str) -> usize {
    let mut score = 0;

    let mut lines = input.trim().split("\r\n");
    let instruction: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next(); // blank line

    let mut map = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(['=', '(', ')', ',', ' '].as_ref()).filter(|s| !s.is_empty()).collect();
        if parts.len() == 3 {
            map.insert(parts[0], (parts[1], parts[2]));
        } else {
            panic!();
        }

        continue;
    }
    let mut position: &str = "AAA";
    let mut steps = 0;
    while position != "ZZZ" {
        let (left, right) = map.get(position).unwrap();
        if instruction[steps%instruction.len()] == 'L' {
            position = left;
        } else {
            position = right;
        }
        steps +=1;
    }
    return steps;
}


fn check_if_not_done(positions: & Vec<&str>) -> bool {
    for pos in positions {
        if !pos.ends_with('Z') {
            return true;
        }
    }
    return false;
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn smallest_common_divisor(numbers: &[usize]) -> usize {
    if numbers.is_empty() {
        return 0; // or handle the case appropriately
    }
    let result = numbers.iter().fold(numbers[0], |acc, &num| lcm(acc, num));
    result
}


fn part_b(input: &str) -> usize {
    let mut lines = input.trim().split("\r\n");
    let instruction: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next(); // blank line

    let mut map = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(['=', '(', ')', ',', ' '].as_ref()).filter(|s| !s.is_empty()).collect();
        if parts.len() == 3 {
            map.insert(parts[0], (parts[1], parts[2]));
        } else {
            panic!();
        }

        continue;
    }
    let mut positions: Vec<&str> = Vec::new();
    for key in map.keys() {
        if key.ends_with('A') {
            positions.push(&key)
        }
    }

    println!("{:?}", positions);
    let mut steps = 0;
    let mut step_dividers = HashSet::new();
    while positions.len()>0 {
        for position in positions.iter_mut() {
            let (left, right) = map.get(*position).unwrap();
            if instruction[steps % instruction.len()] == 'L' {
                *position = *left;
            } else {
                *position = *right;
            }
            
        }
        steps +=1;
        
        positions.retain(|position| {
            if position.ends_with('Z') {
                step_dividers.insert(steps);
                println!("aa {:?}", steps);
                false // Remove the element
            } else {
                true // Keep the element
            }
        });
    }
    step_dividers.insert(steps);
    
    let a: Vec<usize> = step_dividers.into_iter().collect();
    let score = smallest_common_divisor(&a);
    return score;
}


fn main() {
    
    let input = include_str!("input.txt");
    //let ans_a = part_a(input);
    //println!("Part A: {:?}", ans_a);
    // 248131423 low, 248179786
    
    let ans_b = part_b(input);
    println!("Part B: {:?}", ans_b);
    // 248800039 high, 
}
