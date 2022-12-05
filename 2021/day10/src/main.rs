use std::collections::HashMap;

fn part_a() {
    let input = include_str!("input.txt");

    let mut pairs = HashMap::new();
    pairs.insert('(', ')');
    pairs.insert('[', ']');
    pairs.insert('{', '}');
    pairs.insert('<', '>');

    let mut points = HashMap::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);

    let mut score = 0;

    for line in input.trim().split('\n') {
        let mut stack = Vec::new();
        for c in line.chars() {
            if pairs.get(&c).is_some() {
                stack.push(c);
            } else {
                if let Some(c2) = stack.pop() {
                    let expected = pairs[&c2];
                    if c != expected {
                        println!("Line {}", line);
                        println!("expected {}", expected);
                        println!("found {}", c);
                        if c.is_whitespace() {
                            continue;
                        }
                        score += points[&c];
                        break;
                        
                    }
                } else {
                    break;
                }
            }
        }
    }

    println!("score: {}", score);

}


fn main() {
    let input = include_str!("input.txt");

    let mut pairs = HashMap::new();
    pairs.insert('(', ')');
    pairs.insert('[', ']');
    pairs.insert('{', '}');
    pairs.insert('<', '>');

    let mut points = HashMap::new();
    points.insert(')', 1);
    points.insert(']', 2);
    points.insert('}', 3);
    points.insert('>', 4);

    let mut scores = Vec::new();

    for line in input.trim().split('\n') {
        let mut stack = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            if c.is_whitespace() {
                continue;
            }
            if pairs.get(&c).is_some() {
                stack.push(c);
            } else {
                if let Some(c2) = stack.pop() {
                    let expected = pairs[&c2];
                    if c != expected {
                        corrupt = true;
                        break;
                    }
                } else {
                    println!("Line ending {}", line);
                    break;
                }
            }
            
        }
        if !corrupt {
            let mut completion = String::new();
            let mut this_score: i64 = 0;
            stack.reverse();
            for s in stack {
                this_score *= 5;
                this_score += points[&pairs[&s]];
                println!("{}", this_score);
                completion.push(pairs[&s]);
            }
            println!("Completing {}", line);
            println!("With: {}", completion);
            scores.push(this_score);
        }
    }

    assert!(scores.len() % 2 == 1);
    scores.sort_unstable();
    let score = scores[scores.len()/2];
    println!("score: {:?}", scores);
    println!("score: {}", score);

}