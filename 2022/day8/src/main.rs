use std::collections::HashMap;



fn part_a(input: &str) -> u32 {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut map = HashMap::new();

    let mut len_row = 0;
    let mut len_col = 0;
    for line in data {
        len_col = 0;
        for point in line.chars() {
            let tree: u32 = point.to_digit(10).unwrap();
            //println!("col, row: {:?} {} tree {}", col, row, tree);
            map.insert((len_col as i32, len_row as i32), tree);
            len_col += 1;
        }
        len_row += 1;
    }
    println!("{:?}", map);

    'my: for ((col, row), tree) in map.iter() {
        //println!("col, row: {:?} {} tree {}", col, row, tree);
        if *col == 0 || *row == 0 || *col == len_col-1 || *row == len_row-1 {
            score +=1;
            continue;
        }
        
        //let mut visible = true;
        'horiz: for range in [0..*col, *col+1..len_col] {
            let mut visible = true;
            for posx in range {
                
                //et posx = col + dx;
                let neighbour = map.get(&(posx, *row)).unwrap();
                if neighbour >= tree {
                    visible = false;
                    continue 'horiz;
                }
            }
            if visible {
                score += 1;
                continue 'my;
            }
        }
        'vert: for range in [0..*row, *row+1..len_row] {
            let mut visible = true;
            for posy in range {
                //let posy = row + dy;
                let neighbour = map.get(&(*col, posy)).unwrap();
                if neighbour >= tree {
                    visible = false;
                    continue 'vert;
                }
            }
            if visible {
                score +=1;
                continue 'my;
            }
        }
    }
    return score
}


fn part_b(input: &str) -> u32{
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut map = HashMap::new();

    let mut len_row = 0;
    let mut len_col = 0;
    for line in data {
        len_col = 0;
        for point in line.chars() {
            let tree: u32 = point.to_digit(10).unwrap();
            //println!("col, row: {:?} {} tree {}", col, row, tree);
            map.insert((len_row as i32, len_col as i32), tree);
            len_col += 1;
        }
        len_row += 1;
    }

    let mut viewing_map: HashMap<(i32, i32), i32> = HashMap::new();

    'my: for ((row, col), tree) in map.iter() {
        //println!("col, row: {:?} {} tree {}", col, row, tree);
        
        let mut viewing_scores: Vec<i32> = Vec::new();
        'horiz: for sign in [-1, 1] {
            let mut talles_neighbour = 0;
            let mut horz_score = 0;
            let mut idx = 1;
            while let Some(neighbour) = map.get(&(*row, col+idx*sign, )){
                idx += 1;
                
                if neighbour >= &talles_neighbour {
                    talles_neighbour = *neighbour;
                }
                horz_score +=1;
                
                if neighbour >= tree {
                    viewing_scores.push(horz_score);
                    continue 'horiz;
                }
            }
            //println!("{}", scenic_score);
            viewing_scores.push(horz_score);
        }

        'vert: for sign in [-1, 1] {
            println!("Vert with sign {}", sign);
            let mut talles_neighbour = 0;
            let mut vert_score = 0;
            let mut idx = 1;
            while let Some(neighbour) = map.get(&(row+idx*sign, *col)){
                println!("{}:{}={}, score {}", col, row+idx*sign, neighbour, vert_score);
                idx += 1;
                if neighbour >= &talles_neighbour {
                    talles_neighbour = *neighbour;
                }
                vert_score +=1;
                
                if neighbour >= tree {
                    viewing_scores.push(vert_score);
                    continue 'vert;
                }
            }
            viewing_scores.push(vert_score);
            
        }
        println!("{}:{} = {:?}", row, col, viewing_scores);
        assert!(viewing_scores.len() == 4);
        let this_score = viewing_scores.iter().product();
        viewing_map.insert((*row, *col), this_score);
    }
    
    for row in 0..len_row {
        for col in 0..len_col {
            if let Some(val) = map.get(&(row, col)) {
                print!("{}", val)
            } else {
                print!(".")
            }
        }
        println!("")
    }
    println!("");

    for row in 0..len_row {
        for col in 0..len_col {
            if let Some(val) = viewing_map.get(&(row, col)) {
                print!("{}", val)
            } else {
                print!(".")
            }
        }
        println!("")
    }
    score = *viewing_map.values().max().unwrap();

    // 6336
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
    const EXAMPLE_A: u32 = 21;
    const EXAMPLE_B: u32 = 0;
    const SOLVE_A: u32 = 0;
    const SOLVE_B: u32 = 0;
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