
const WIN: i32 = 6;
const DRAW: i32 = 3;

fn parse_line_part_b(line: &str) ->i32 {
    let mut score = 0;
    let mut chars = line.chars();
    let opponent = chars.next().unwrap();
    chars.next();
    let mine = chars.next().unwrap();
    //println!("Opponent {}, Mine {}", opponent, mine);
    if mine == 'X' {
        // lose
        if opponent == 'C' {
            score += 2;
        } else if opponent == 'A' {
            score += 3;
        } else {
            score += 1;
        }
    } else if mine == 'Y' {
        // draw
        score += 3;
        if opponent == 'A' {
            score += 1
        } else if opponent == 'B' {
            score += 2;
        } else {
            score += 3;
        }
    } else if mine == 'Z' {
        // win
        score +=6;
        if opponent == 'B' {
            score += 3
        } else if opponent == 'C' {
            score += 1;
        } else {
            score += 2;
        }
    } else {
        panic!()
    }
    return  score;
}

fn parse_line_part_a(line: &str) -> i32 {
    let mut score = 0;

    //println!("{}", line);
    let mut chars = line.chars();
    let opponent = chars.next().unwrap();
    chars.next();
    let mine = chars.next().unwrap();
    //println!("Opponent {}, Mine {}", opponent, mine);
    if mine == 'X' {
        score += 1;
        if opponent == 'C' {
            score += WIN;
        } else if opponent == 'A' {
            score += DRAW;
        }
    } else if mine == 'Y' {
        score += 2;
        if opponent == 'A' {
            score += WIN
        } else if opponent == 'B' {
            score += DRAW;
        }
    } else if mine == 'Z' {
        score +=3;
        if opponent == 'B' {
            score += WIN
        } else if opponent == 'C' {
            score += DRAW;
        }
    } else {
        panic!()
    }
    return score
}


fn main(){
    // A - rock    X - Lose
    // B - paper   Y - draw 
    // C - siccors Z - win

    
    let input = include_str!("input.txt");
    let data = input.trim().split("\r\n");
    
    let mut score_a  = 0;
    let mut score_b  = 0;

    for line in data {
        score_a += parse_line_part_a(line);
        score_b += parse_line_part_b(line);
    }
    println!("Score Part A {}", score_a);
    println!("Score Part B {}", score_b);
}