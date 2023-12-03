use std::alloc::GlobalAlloc;

fn part_a(input: &str) -> i32 {
    let mut score = 0;
    let red: i32 = 12;
    let green = 13; 
    let blue = 14;
    'outer: for line in input.split("\r\n") {
        let (game_id, picks) = line.split_once(": ").unwrap();
        let (a, game_id) = game_id.split_once(' ').unwrap();
        let game_id: i32 = game_id.parse().unwrap();
        let picks: Vec<&str> = picks.split("; ").collect();
        for pick in picks {
            let content: Vec<&str> = pick.split(", ").collect();
            for grab in content {
                let (nr, color) = grab.split_once(' ').unwrap();
                let nr: i32 = nr.parse().unwrap();
                match color {
                    "red" => {
                        if nr > red {
                            continue 'outer;
                        }
                    },
                    "blue" => {
                        if nr > blue {
                            continue 'outer;
                        }
                    },
                    "green" => {
                        if nr > green {
                            continue 'outer;
                        }
                    },
                    _ => {panic!()}
                }
            }
        }
        println!("nr {:?}", game_id);
        score += game_id
    }
    return  score;
}

fn part_b(input: &str) -> i32 {
    let mut score = 0;
    
    'outer: for line in input.split("\r\n") {
        let mut red: i32 = 0;
        let mut green = 0; 
        let mut blue = 0;

        let (game_id, picks) = line.split_once(": ").unwrap();
        let (a, game_id) = game_id.split_once(' ').unwrap();
        let game_id: i32 = game_id.parse().unwrap();
        let picks: Vec<&str> = picks.split("; ").collect();
        for pick in picks {
            let content: Vec<&str> = pick.split(", ").collect();
            for grab in content {
                let (nr, color) = grab.split_once(' ').unwrap();
                let nr: i32 = nr.parse().unwrap();
                match color {
                    "red" => {
                        red = red.max(nr);
                    },
                    "blue" => {
                        blue = blue.max(nr);
                    },
                    "green" => {
                        green = green.max(nr);
                    },
                    _ => {panic!()}
                }
            }
        }
        score += red*green*blue;
    }
    return  score;
}


fn main() {
    
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {:?}", ans_a);
    
    let ans_b = part_b(input);
    println!("Part B: {:?}", ans_b);
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
