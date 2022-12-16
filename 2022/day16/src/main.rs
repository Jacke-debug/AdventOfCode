use std::{collections::HashMap, borrow::Borrow, hash::Hash};

#[derive(Debug)]
#[derive(Clone)]
struct Node<'a> {
    name: &'a str,
    rate: i32,
    state: bool,
    connections: Vec<&'a str>,
}

fn part_a(input: &str) -> i32 {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut map: HashMap<&str, Node> = HashMap::new();

    while let Some(line) = data.next() {
        let (name, rest) = line.split_once(" has").unwrap();
        let (_, name) = name.split_once(' ').unwrap();
        let (rate, rest) = rest.split_once(";").unwrap();
        let (_, rate) = rate.split_once("=").unwrap();
        let rate: i32 = rate.parse().unwrap();
        match rest.split_once("valves ") {
            Some((_, connections)) => {
                let cons = connections.split(", ");
                let mut connections: Vec<&str> = cons.into_iter().collect();
                if rate != 0 {
                    connections.push(name);
                }
                
                let node = Node{
                    name,
                    rate,
                    state: false,
                    connections: connections,
                };
                map.insert(name, node);
            },
            None => {
                let (_, connection) = rest.split_once("valve ").unwrap();
                let mut connections: Vec<&str> = vec!(connection);
                if rate != 0 {
                    connections.push(name);
                }
                let node = Node{
                    name,
                    rate,
                    state: false,
                    connections: connections,
                };
                map.insert(name, node);
            },
        };
    }
    println!("{:?}", map);

    let mut position = "AA";
    let mut time = 0;
    let mut total_flow = 0;
    let mut flow_rate = 0;
    let mut max_flow: Vec<i32> = Vec::new();
    make_move(&map, position, total_flow, flow_rate, time, &mut max_flow);

    score = *max_flow.iter().max().unwrap();
    return score
}

fn make_move(map: &HashMap<&str, Node>, position: &str, total_flow: i32, flow_rate: i32, time: i32, max_flow: &mut Vec<i32>) {
    let node = map.get_mut(position).unwrap();
    let mut possible_moves = node.connections.clone();
    if node.rate == 0 || !node.state {
        possible_moves.pop(); // remove conneciton to self
    }
    for this_move in possible_moves {
        let new_total_flow = total_flow + flow_rate;
        
        let mut new_flow_rate = flow_rate;
        let new_position = this_move;
        if this_move == position {
            new_flow_rate += node.rate;
            node.state = true;
        }
        let new_time = time + 1;
        if new_time == 30 {
            max_flow.push(new_total_flow);
            return;
        } else if new_time > 15 && new_total_flow < 40 {
            return;
        } else if new_time > 20 && new_total_flow < 60 {
            return;
        } else if new_time > 25 && new_total_flow < 60 {
            return;
        } else {
            make_move(map, new_position, new_total_flow, new_flow_rate, new_time, max_flow);
        }
    }
    
}


fn part_b(input: &str) -> i32{
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    return score
}



fn main(){
    let input = include_str!("example.txt");
    let score_a = part_a(input);
    println!();
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: i32 = 31;
    const SOLVE_A: i32 = 472;

    const EXAMPLE_B: i32 = 29;
    const SOLVE_B: i32 = 465;

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