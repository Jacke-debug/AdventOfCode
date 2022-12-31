use std::collections::HashMap;

fn print_map(map: &HashMap<(isize, isize), char>, row: isize, col: isize) {
    for x in 0..row {
        for y in 0..=col {
            match map.get(&(x, y)) {
                Some(x) => {print!("{}", x)},
                None => {print!(" ")},
            }
        }
        println!();
    }
}

fn dir_from_int(num: usize) -> Direction {
    match num {
        0 => {return Direction::Right},
        1 => {return Direction::Down},
        2 => {return Direction::Left},
        3 => {return Direction::Up},
        _ => {panic!()}
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right = 0, 
    Down = 1,
    Left = 2, 
    Up = 3,
}
#[derive(Debug)]
struct Position {
    row: isize,
    col: isize,
    dir: Direction,
}

fn try_move(map: &HashMap<(isize, isize), char>,  pos: &mut Position, steps: usize) {
    let mut dir: (isize, isize) = (0, 0);
    match &pos.dir {
        Direction::Up => {dir = (-1, 0)},
        Direction::Down => {dir = (1, 0)},
        Direction::Left => {dir = (0, -1)},
        Direction::Right => {dir = (0, 1)},
        _ => {},
    }
    for _ in 0..steps {
        let mut new_row = pos.row as isize + dir.0;
        let mut new_col = pos.col as isize + dir.1;
        match map.get(&(new_row, new_col)) {
            Some(_) => {},
            None => {
                // wrap around to other side
                match &pos.dir {
                    Direction::Right => {
                        new_col = map.iter().filter(|((row, col), _)| *row == new_row).map(|((_, col), _)| *col).min().unwrap();
                    }, 
                    Direction::Left => {
                        new_col = map.iter().filter(|((row, col), _)| *row == new_row).map(|((_, col), _)| *col).max().unwrap();
                    },
                    Direction::Up => {
                        new_row = map.iter().filter(|((row, col), _)| *col == new_col).map(|((row, _), _)| *row).max().unwrap();
                    },
                    Direction::Down => {
                        new_row = map.iter().filter(|((row, col), _)| *col == new_col).map(|((row, _), _)| *row).min().unwrap();
                    },
                }
            }
        }
        
        match map.get(&(new_row, new_col)) {
            Some('#') => {
                //println!("blocked");
                return;
            },
            Some('.') => {
                //println!("Move to {} {}", new_row, new_col);
                pos.row = new_row;
                pos.col = new_col;
            },
            _ => {panic!()}
        }
    }
}

fn try_move_b(map: &HashMap<(isize, isize), char>,  pos: &mut Position, steps: usize) {
    let mut dir: (isize, isize) = (0, 0);
    
    for _ in 0..steps {
        match &pos.dir {
            Direction::Up => {dir = (-1, 0)},
            Direction::Down => {dir = (1, 0)},
            Direction::Left => {dir = (0, -1)},
            Direction::Right => {dir = (0, 1)},
            _ => {},
        }

        let mut new_row = pos.row as isize + dir.0;
        let mut new_col = pos.col as isize + dir.1;
        let mut new_dir = pos.dir;
        match map.get(&(new_row, new_col)) {
            Some(_) => {},
            None => {
                // TODO: fix wrapping
                //    2 3
                //    5
                //  7 8
                //  10

                match pos.col {
                    0..=49 => {
                        match pos.row {
                            100..=149 => {
                                // 7
                                match pos.dir {
                                    Direction::Up => {
                                        // 7 -> 5
                                        new_col = 50;
                                        new_row = pos.col+50;
                                        new_dir = Direction::Right;
                                    }
                                    Direction::Left => {
                                        // 7 -> 2
                                        new_col = 50;
                                        new_row = 149-pos.row;
                                        new_dir = Direction::Right;
                                    }
                                    _ => {panic!()},
                                }
                            },
                            
                            150..=199 => {
                                // 10
                                match pos.dir {
                                    Direction::Left => {
                                        // 10 -> 2
                                        new_col = pos.row-100;
                                        new_row = 0;
                                        new_dir = Direction::Down;
                                    }
                                    Direction::Right => {
                                        // 10 -> 8
                                        new_col = pos.row-100;
                                        new_row = 149;
                                        new_dir = Direction::Up;
                                    }
                                    Direction::Down => {
                                        // 10 -> 3
                                        new_col = pos.col+100;
                                        new_row = 0;
                                        new_dir = Direction::Down;
                                    }
                                    _ => {panic!()},
                                }
                            },
                            _ => {
                                panic!()
                            }
                        }
                    },
                    
                    50..=99 => {
                        match pos.row {
                            0..=49 => {
                                // 2
                                match pos.dir {
                                    // 2 -> 10
                                    Direction::Up => {
                                        new_row = pos.col + 100; 
                                        new_col = 0;
                                        new_dir = Direction::Right
                                    }
                                    // 2 -> 7
                                    Direction::Left => {
                                        new_row = 149 - pos.row;
                                        new_col = 0;
                                        new_dir = Direction::Right
                                    }
                                    _ => {panic!()},
                                }
                            },
    
                            50..=99 => {
                                // 5
                                match pos.dir {
                                    // 5 -> 7
                                    Direction::Left => {
                                        new_row = 100;   // top of seven
                                        new_col = pos.row-50;
                                        new_dir = Direction::Down
                                    }
                                    // 5 -> 3
                                    Direction::Right => {
                                        new_row = 49;
                                        new_col = pos.row + 50;
                                        new_dir = Direction::Up
                                    }
                                    _ => {panic!()},
                                }
                            }
                            
                            100..=149 => {
                                // 8
                                match pos.dir {
                                    // 8 -> 3
                                    Direction::Right => {
                                        new_row = 149-pos.row;
                                        new_col = 149;
                                        new_dir = Direction::Left
                                    }
                                    // 8 -> 10
                                    Direction::Down => {
                                        new_row = pos.col + 100;
                                        new_col = 49;
                                        new_dir = Direction::Left
                                    }
                                    _ => {
                                        panic!()},
                                }
                            }
                            _ => {panic!()},
                        }
                    },
                    
                    100..=149 => {
                        // 3 
                        match pos.dir {
                            // 3 -> 8
                            Direction::Right => {
                                new_row = 149-pos.row;
                                new_col = 99;
                                new_dir = Direction::Left
                            }
                            // 3 -> 5
                            Direction::Down => {
                                new_row = pos.col - 50;
                                new_col = 99;
                                new_dir = Direction::Left
                            }
                            // 3 -> 10
                            Direction::Up => {
                                new_row = 199;
                                new_col = pos.col - 100;
                                new_dir = Direction::Up
                            }
                            _ => {panic!()},
                        }
                    },
                    _ => {panic!()},
                }
            }
        }
        
        match map.get(&(new_row, new_col)) {
            Some('#') => {
                //println!("blocked");
                return;
            },
            Some('.') => {
                //println!("Move to {} {}", new_row, new_col);
                pos.dir = new_dir;
                pos.row = new_row;
                pos.col = new_col;
            },
            _ => {
                println!("Trying to reach outside map {} {}", new_row, new_col);
                println!("Coming from {:?}", pos);
                panic!()}
        }
    }
}


fn part_a(input: &str) -> isize {
    let mut lines = input.split("\r\n");
    let mut map: HashMap<(isize, isize), char> = HashMap::new();

    let mut row = 0;
    let mut max_col = 0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            
            break;
        }
        for (col, char) in line.chars().enumerate() {
            match char {
                ' ' => {
                    continue;
                },
                _ => {
                    map.insert((row, col as isize), char);
                    max_col = max_col.max(col);
                }
            }
        }
        row += 1;
    }
    
    let min_col = map.iter().filter(|((row, col), _)| *row == 0).map(|((_, col), _)| *col).min().unwrap();

    let mut pos = Position{
        row: 0,
        col: min_col,
        dir: Direction::Right
    };
    
    let instructions = lines.next().unwrap();

    for instruction in instructions.split_ascii_whitespace() {
        match instruction.parse::<usize>() {
            Ok(steps) => {             
                try_move(&map, &mut pos, steps);
            },
            Err(_) => {
                match instruction {
                    "R" => {
                        pos.dir = dir_from_int((pos.dir as usize + 1) % 4);
                    },
                    "L" => {
                        pos.dir = dir_from_int((pos.dir as usize + 3) % 4);
                    },
                    _ => {panic!()}
                }
            },
        }
    }
    let score = ((pos.row+1)*1000 + (pos.col+1)*4 + pos.dir as isize);
    // 106094
    return score;
}


fn part_b(input: &str) -> isize {
    //
    //     1
    // 2 3 4
    //     5 6
    //      

    let mut lines = input.split("\r\n");
    let mut map: HashMap<(isize, isize), char> = HashMap::new();

    let mut row = 0;
    let mut max_col = 0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            
            break;
        }
        for (col, char) in line.chars().enumerate() {
            match char {
                ' ' => {
                    continue;
                },
                _ => {
                    map.insert((row, col as isize), char);
                    max_col = max_col.max(col);
                }
            }
        }
        row += 1;
    }

    let min_col = map.iter().filter(|((row, col), _)| *row == 0).map(|((_, col), _)| *col).min().unwrap();

    let mut pos = Position{
        row: 0,
        col: min_col,
        dir: Direction::Right
    };
    
    let instructions = lines.next().unwrap();

    for instruction in instructions.split_ascii_whitespace() {
        match instruction.parse::<usize>() {
            Ok(steps) => {             
                try_move_b(&map, &mut pos, steps);
            },
            Err(_) => {
                match instruction {
                    "R" => {
                        pos.dir = dir_from_int((pos.dir as usize + 5) % 4);
                    },
                    "L" => {
                        pos.dir = dir_from_int((pos.dir as usize + 3) % 4);
                    },
                    _ => {panic!()}
                }
            },
        }
    }
    let score = ((pos.row+1)*1000 + (pos.col+1)*4 + pos.dir as isize);
    return score;
}



fn main() {
    let input = include_str!("input.txt");
    let score_a = part_a(input);
    println!("Score A {}", score_a);   
    let score_b = part_b(input); 
    println!("Score B {}", score_b); // 141286 low < x <186067 high
}