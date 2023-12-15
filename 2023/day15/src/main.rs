use std::collections::HashMap;


fn hash(str: &str) -> usize {
    let mut answer = 0;
    for ch in str.chars() {
        let val = ch as u8;
        answer = ((answer + val as usize) * 17) % 256;
    }
    return answer;
}
fn part_a(input: &str) -> usize {
    let mut score = 0;
    for line in input.split(',') {
        score += hash(line);
    }
    return score;
}

fn part_b(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = Vec::new();
    for _ in 0..256 {
        let empty_map: Vec<(&str, usize)> = Vec::new();
        boxes.push(empty_map);
    }

    let mut score = 0;
    'next: for line in input.split(',') {
        let mut answer = 0;
        if line.contains('=') {
            let (label, lens) = line.split_once('=').unwrap();
            let box_nr = hash(label);
            let lens = lens.parse::<usize>().unwrap();
            let mut box_content = boxes.get_mut(box_nr).unwrap();
            for (label_in_box, lens_in_box) in box_content.iter_mut() {
                if *label_in_box == label {
                    *lens_in_box = lens;
                    continue 'next;
                }
            }
            box_content.push((label, lens))
        } else if line.contains('-') {
            let (label, _) = line.split_once('-').unwrap();
            let box_nr = hash(label);
            let mut box_content = boxes.get_mut(box_nr).unwrap();
            let mut idx_to_remove = None;
            for (idx, (label_in_box, _)) in box_content.iter().enumerate() {
                if *label_in_box == label {
                    idx_to_remove = Some(idx);
                }
            }
            match idx_to_remove {
                Some(idx) => {
                    box_content.remove(idx);
                }
                _ => {}
            }
        } else {
            panic!()
        }
    }

    for (box_id, content) in boxes.iter().enumerate() {
        for (lens_spot, (_, lens)) in content.iter().enumerate() {
            score += (box_id+1)* (lens_spot+1)*lens;
        }
    }
    println!("{:?}", boxes);
    return score;
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
        let input = "HASH";
        assert_eq!(part_a(input), 52);
    }
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), 1320);
    }
    #[test] 
    fn test_b_simple() {
        let input = "rn=1";
        assert_eq!(part_b(input), 1);
    }
    #[test]
    fn test_b_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), 145);
    }
}