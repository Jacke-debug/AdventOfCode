use std::{str::Chars, collections::VecDeque};


fn compare(left_chars: &mut VecDeque<char>, right_chars: &mut VecDeque<char>) -> bool {
    while let Some(mut left) = left_chars.pop_front() {
        let mut right = match right_chars.pop_front() {
            Some(right) => {right},
            None => {
                // right run out first => wrong order
                return false;
            },
        };
        // println!("Comparing {} vs {}", left, right);

        if left == right {
            return compare(left_chars, right_chars);
        }

        match left {
            '[' => {
                if right == ']' {
                    return false;
                } else {
                    right_chars.push_front(']');
                    right_chars.push_front(right);
                    return compare(left_chars, right_chars)
                }
            },
            ',' => {
                // left longer
                return false
            },
            ']' => {
                // first vector is shorter than second => wrong order
                return true
            },
            _ => {
                // numeric
                let mut left_num: u32;
                if left == 'x' {
                    left_num = 10;
                } else {
                    left_num = left.to_digit(10).unwrap();
                }
                
                match right.to_digit(10) {
                    Some(mut right_num) => {
                        if left_num < right_num {
                            return true;
                        } else {
                            return false
                        }
                    },
                    None => {
                        if right == 'x' {
                            let right_num = 10;
                            if left_num < right_num {
                                return true;
                            } else {
                                return false
                            }
                        } else {
                            if right == ']' {
                                return false;
                            } else {
                                left_chars.push_front(']');
                                left_chars.push_front(left);
                                return compare(left_chars, right_chars)
                            }
                        }
                    },
                };
            }
        }
    }
    // left ran out first 
    return true;
}

fn part_a(input: &str) -> i32 {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut correct_indices: Vec<i32> = Vec::new();
    let mut idx = 0;
    while let Some(first) = data.next() {
        idx +=1;

        let second = data.next().unwrap();
        let mut left_chars: VecDeque<char> = first.chars().collect();
        let mut right_chars: VecDeque<char>  = second.chars().collect();
        println!("{:?}", first);
        println!("{:?}", second);
         
        let is_correct = compare(&mut left_chars, &mut right_chars);
            
        data.next();
        if is_correct {
            println!("correct");
            correct_indices.push(idx);
        } else {
            println!("wrong");
        }
    }

    println!("{:?}", correct_indices);
    score = correct_indices.iter().sum();
    return score
    // 5818 high
}




fn part_b(input: &str) -> usize{
    let mut data = input.trim().split("\r\n");
    

    let mut lines: VecDeque<VecDeque<char>> = VecDeque::new();
    
    //lines.push_front(VecDeque::from(['[','[','6',']',']']));
    
    lines.push_front(VecDeque::new());
    
    while let Some(line) = data.next() {
        let line: VecDeque<char> = line.chars().collect();
        let mut is_correct = false;
        let mut idx = 0;
        while !is_correct && idx < lines.len(){
            let mut line_after = lines.get(idx).unwrap().clone();
            is_correct = compare(&mut line.clone(),  &mut line_after);
            idx+=1;
        }
        lines.insert(idx-1, line)

    }
    lines.retain(|i| !i.is_empty());
    
    let line2 = VecDeque::from(['[','[','2',']',']']);
    let mut is_correct = false;
    let mut idx2 = 0;
    while !is_correct && idx2 < lines.len(){
        let mut line_after = lines.get(idx2).unwrap().clone();
        is_correct = compare(&mut line2.clone(),  &mut line_after);
        idx2+=1;
    }

    lines.insert(idx2-1, line2);

    let line6 = VecDeque::from(['[','[','6',']',']']);
    let mut is_correct = false;
    let mut idx6 = 0;
    while !is_correct && idx6 < lines.len(){
        let mut line_after = lines.get(idx6).unwrap().clone();
        is_correct = compare(&mut line6.clone(),  &mut line_after);
        idx6+=1;
    }
    lines.insert(idx6-1, line6);
    
    let len = lines.len();
    for line in lines {
        for char in line {
            print!("{}", char);
        }
        println!("");
    }
    println!("{}, {}, {}", len, idx2, idx6);
    let mut score = (idx2)*(idx6);
    return score
    // 22848 low
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
    const EXAMPLE_A: i32 = 13;
    const SOLVE_A: i32 = 5366;

    const EXAMPLE_B: usize = 140;
    const SOLVE_B: usize = 23391;

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