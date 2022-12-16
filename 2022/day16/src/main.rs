use std::{collections::HashMap, borrow::Borrow, hash::Hash};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Connection<'a> {
    dest: &'a str,
    cost: i32,
}

#[derive(Debug)]
#[derive(Clone)]
struct Node<'a> {
    name: &'a str,
    rate: i32,
    state: bool,
    connections: Vec<Connection<'a>>,
}

fn part_a(input: &str) -> i32 {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut map: Vec<Node> = Vec::new();
    let mut zero_valves: HashMap<&str, Node> = HashMap::new();

    while let Some(line) = data.next() {
        let (name, rest) = line.split_once(" has").unwrap();
        let (_, name) = name.split_once(' ').unwrap();
        let (rate, rest) = rest.split_once(";").unwrap();
        let (_, rate) = rate.split_once("=").unwrap();
        let rate: i32 = rate.parse().unwrap();
        match rest.split_once("valves ") {
            Some((_, con_str)) => {
                let mut connections = Vec::new();
                for connection in con_str.split(", ").into_iter() {
                    connections.push(Connection{
                        dest: connection, 
                        cost: 1
                    })
                }
                let mut node = Node{
                    name,
                    rate,
                    state: false,
                    connections: connections,
                };
                if rate == 0 && name != "AA" {
                    zero_valves.insert(name, node);
                } else {
                    if name == "AA" {
                        node.state = true;
                    }
                    map.push(node);
                }
            },
            None => {
                let (_, connection) = rest.split_once("valve ").unwrap();
                let mut connections = Vec::new();
                connections.push(Connection{
                    dest: connection, 
                    cost: 1
                });
                let mut node = Node{
                    name,
                    rate,
                    state: false,
                    connections: connections,
                };
                if rate == 0 && name != "AA"{
                    zero_valves.insert(name, node);
                } else {
                    if name == "AA" {
                        node.state = true;
                    }
                    map.push(node);
                }
            },
        };
    }
    println!("{:?}", map);

    // optimize map

    for valve in map.iter_mut() {
        for _ in 0..30 {
            let mut new_tunnels = Vec::new();
            let mut existing_targets = Vec::new();
            for aa in &valve.connections {
                existing_targets.push(aa.dest);
            }
            for tunnel in &valve.connections {
                // if tunnel leads to a zero valve
                if let Some(zero_valve) = zero_valves.get(tunnel.dest) {
                    for zer_tunnel in &zero_valve.connections {
                        if !existing_targets.contains(&zer_tunnel.dest) {
                            new_tunnels.push(Connection{
                                dest: zer_tunnel.dest,
                                cost: zer_tunnel.cost + tunnel.cost,
                            })
                        }
                    }
                } else {
                    new_tunnels.push(*tunnel)
                }
            }
            println!("\n {:?} \n\n", valve.connections);
            valve.connections = new_tunnels;
            println!("{:?}", valve.connections);
        }
    }

    let mut comp_map = HashMap::new();
    for valve in map.iter_mut() {
        let mut new_tunnels = Vec::new();
        for tunnel in &valve.connections {
            if let Some(zero_valve) = zero_valves.get(tunnel.dest) {
            } else {
                if tunnel.dest == valve.name {
                    // dont add conections to self
                } else {
                    new_tunnels.push(*tunnel)
                }
            }
        }
        valve.connections = new_tunnels;
        comp_map.insert(valve.name, valve.clone());
    }

    println!();
    println!();
    for (_, item) in comp_map.iter() {
        println!("{:?}", item);
    }
    
    let time = 0;
    let flow_rate = 0;
    let total_flow = 0;
    let position = "AA";
    let mut max_flow = Vec::new();
    make_move(comp_map, position, total_flow, flow_rate, time, &mut max_flow);

    score = *max_flow.iter().max().unwrap();
    return score
}

fn make_move(map: HashMap<&str, Node>, position: &str, total_flow: i32, flow_rate: i32, time: i32, max_flow: &mut Vec<i32>) {
    let node = map.get(position).unwrap();
    let mut possible_moves = node.connections.clone();

    if !node.state {
        possible_moves.push(Connection{dest: position, cost: 1}); // add posibility to open vault   
    }

    for this_move in possible_moves {
        let new_total_flow = total_flow + flow_rate*this_move.cost;
        
        let mut new_flow_rate = flow_rate;
        let mut new_map = map.clone();
        if this_move.dest == position {
            new_flow_rate += node.rate;
            new_map.get_mut(position).unwrap().state = true;
            // remove visitied states?
        }

        let new_time = time + this_move.cost;
        if new_time > 20 {
            max_flow.push(new_total_flow);
            return;
        } else {
            make_move(new_map, this_move.dest, new_total_flow, new_flow_rate, new_time, max_flow);
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