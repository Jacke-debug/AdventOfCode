use std::collections::HashSet;

fn update_tail_pos(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let diff_x = head_pos.0 - tail_pos.0;
    let diff_y = head_pos.1 - tail_pos.1;
    let mut posx = tail_pos.0;
    let mut posy = tail_pos.1;
    if diff_y.abs() == 2 {
        posx += diff_x.signum();
        posy += diff_y.signum();
    } else if diff_x.abs() == 2 {
        posx += diff_x.signum();
        posy += diff_y.signum();
    } 
    return (posx, posy);
}

fn part_a(input: &str) -> u32 {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    for line in data {
        let (dir, steps) = line.split_once(" ").unwrap();
        let steps: i32 = steps.parse().unwrap();
        let dxy = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!(),
        };

        for _ in 0..steps {
            head_pos = (head_pos.0 + dxy.0, head_pos.1 + dxy.1);
            tail_pos = update_tail_pos(head_pos, tail_pos);
            visited.insert(tail_pos);
        }
    }
    score = visited.len();

    return score.try_into().unwrap();
}


fn part_b(input: &str) -> u32{
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let rope_length = 10;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut knot_pos: Vec<(i32, i32)> = Vec::new();
    for _ in 0..rope_length {
        knot_pos.push((0, 0));
    }
    println!("{}", knot_pos.len());
    for line in data {
        let (dir, steps) = line.split_once(" ").unwrap();
        let steps: i32 = steps.parse().unwrap();

        println!("{}", line);
        let dxy = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!(),
        };

        for _ in 0..steps {
            knot_pos[0] = (knot_pos[0].0 + dxy.0, knot_pos[0].1 + dxy.1);
            for i in 1..rope_length {
                knot_pos[i] = update_tail_pos(knot_pos[i-1], knot_pos[i])
            }
            //tail_pos = update_tail_pos(head_pos, tail_pos);
            visited.insert(knot_pos[rope_length-1]);
        }
        println!("Head pos {:?}", knot_pos[0]);
        println!("End pos {:?}", knot_pos[rope_length-1]);
    }
    // 182, 310
    for row in -20..20 {
        for col in -20..20 {
            if visited.contains(&(col, -row)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("")
    }
    score = visited.len();

    return score.try_into().unwrap();
}



fn main(){
    let input = include_str!("input.txt");
    let score_a = part_a(input);
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: u32 = 13;
    const SOLVE_A: u32 = 5930;
    const EXAMPLE_B: u32 = 1;
    const SOLVE_B: u32 = 2443;
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