use std::{collections::HashMap};


fn get_next_pos(dir: &char, pos: &(usize, usize)) -> Option<(usize, usize)> {
    let mut next_pos = *pos;
    match dir {
        '>' => next_pos.0 += 1,
        '<' => next_pos.0 = next_pos.0.checked_sub(1)?,
        'v' => next_pos.1 += 1,
        '^' => next_pos.1 = next_pos.1.checked_sub(1)?,
        _ => panic!(),
    }
    Some(next_pos)
}

fn part_a(input: &str) -> usize {
    let mut active_beams = Vec::new();
    let mut beam_map:  HashMap<(usize, usize), Vec<char>> = HashMap::new();
    let mut map = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    for (y, line) in input.split('\n').enumerate() {
        for (x, ch) in line.chars().enumerate() {
            beam_map.insert((x, y), vec![]);
            map.insert((x, y), ch);
            x_max = x;
        }
        y_max = y;
    }
    active_beams.push(((0,0), '>'));
    beam_map.insert((0,0), vec!['>']);
    solve(& beam_map, &active_beams, &map, x_max, y_max)
}

fn solve(beam_map: & HashMap<(usize, usize), Vec<char>>, active_beams: & Vec<((usize, usize), char)>, map: & HashMap<(usize, usize), char>, x_max: usize, y_max: usize) -> usize{
    let mut beam_map = beam_map.clone();
    let mut active_beams = active_beams.clone();
    let mut any_new_beams =  true;
    while any_new_beams {
        let mut new_beams: Vec<((usize, usize), char)> = Vec::new();
        for (pos, dir) in active_beams {
            let next_pos = match get_next_pos(&dir, &pos) {
                Some(x) => {x},
                None => continue // beam moves outside map
            };
            match map.get(&next_pos) {
                Some('-') => {
                    match dir {
                        '>' | '<' => {
                            new_beams.push((next_pos, dir));
                        } // pass thorugh
                        _ => {
                            new_beams.push((next_pos, '<'));
                            new_beams.push((next_pos, '>'));
                        } 
                    }
                },
                Some('|') => {
                    match dir {
                        '^' | 'v' => {new_beams.push((next_pos, dir));} // pass thorugh
                        _ => {
                            new_beams.push((next_pos, '^'));
                            new_beams.push((next_pos, 'v'));
                        }
                    }
                },
                Some('/') => {
                    match dir {
                        '^' => {new_beams.push((next_pos, '>'));} 
                        '<' => {new_beams.push((next_pos, 'v'));}
                        '>' => {new_beams.push((next_pos, '^'));}
                        _ => {new_beams.push((next_pos, '<'));}
                    }
                },
                Some('\\') => {
                    match dir {
                        '^' => {new_beams.push((next_pos, '<'));} 
                        '<' => {new_beams.push((next_pos, '^'));}
                        '>' => {new_beams.push((next_pos, 'v'));}
                        _ => {new_beams.push((next_pos, '>'));}
                    }
                },
                Some('.') => {new_beams.push((next_pos, dir));},
                _ => {},
            }
        }
        // check if beam already has been traced
        active_beams = Vec::new();
        any_new_beams = false;
        for (pos, dir) in new_beams {
            match beam_map.get_mut(&pos) {
                Some(v) => {
                    if !v.contains(&dir) {
                        v.push(dir); // new beam that we need to follow
                        active_beams.push((pos, dir));
                        any_new_beams =true;
                    } 
                }
                None => {panic!()}
            }
        }
    }
    let mut score = 0;
    for y in 0..=y_max {
        //println!();
        for x in 0..=x_max {
            let beams = beam_map.get(&(x, y)).unwrap();
            if beams.is_empty() {
                //print!(".");
            } else {
                //print!("#");
                score +=1;
            }
        }
    }
    return score;
}


fn part_b(input: &str) -> usize {
    let mut beam_map:  HashMap<(usize, usize), Vec<char>> = HashMap::new();
    let mut map = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    for (y, line) in input.split('\n').enumerate() {
        for (x, ch) in line.chars().enumerate() {
            beam_map.insert((x, y), vec![]);
            map.insert((x, y), ch);
            x_max = x;
        }
        y_max = y;
    }
    let mut score = 0;
    for x in [0, x_max] {
        for y in 0..=y_max { 
            let mut active_beams = Vec::new();
            for dir in ['v', '^'] {
                active_beams.push(((x,y), dir));
                beam_map.insert((x,y), vec![dir]);
                score = score.max(solve(&beam_map, &active_beams, &map, x_max, y_max));
            }
        }
    }
    for x in 0..=x_max {
        for y in [0, y_max] { 
            let mut active_beams = Vec::new();
            for dir in ['<', '>'] {
                active_beams.push(((x,y), dir));
                beam_map.insert((x,y), vec![dir]);
                score = score.max(solve(&beam_map, &active_beams, &map, x_max, y_max));
            }
        }
    }

    return score;
}

fn main() {
    let input = include_str!("example.txt");
    let ans_a = part_a(input);
    println!("Part A: {}", ans_a);
    let ans_b = part_b(input);
    println!("Part B: {}", ans_b); // 87683 low
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), 46);
    }
    #[test]
    fn test_b_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), 51);
    }
}