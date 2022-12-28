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

fn remove_by_name<'a>(map: &mut HashMap<&str, Node<'a>>, name: &str){
    let node_to_remove = &map.get(name).unwrap().to_owned();

    let mut shortened_paths = HashMap::new();
    let mut new_paths = HashMap::new();

    for (this_name, valve) in map.iter_mut() {
        let mut new_tunnels = Vec::new();

        for tunnel in &valve.connections {
            // if tunnel leads to removed_node, dont add it
            if tunnel.dest == node_to_remove.name {
                for zer_tunnel in &node_to_remove.connections {
                    if zer_tunnel.dest == valve.name {
                        // dont add conections to self
                    } else {
                        let new_con = Connection{
                            dest: zer_tunnel.dest,
                            cost: zer_tunnel.cost + tunnel.cost,
                        };
                        new_paths.insert(*this_name, new_con);
                        new_tunnels.push(new_con);
                    }
                }
            } else {
                let mut new_old_tunnel = *tunnel;
                // update cost of existing items 
                for alternative_tunnel in &node_to_remove.connections {
                    // only add existing if its shorted
                    // how to update the same connection in the other direciton?
                    let new_cost = alternative_tunnel.cost + tunnel.cost;
                    if  new_cost< new_old_tunnel.cost {
                        new_old_tunnel.cost = new_cost;
                        shortened_paths.insert(*this_name, new_old_tunnel);
                    }  
                }
                new_tunnels.push(new_old_tunnel)
            }
        }
        valve.connections = new_tunnels;
    }

    for (from, path) in shortened_paths {
        for (node_name, node) in map.iter_mut() {
            if *node_name == from {
                for tunnel in node.connections.iter_mut() {
                    if tunnel.dest == path.dest {
                        tunnel.cost = path.cost;
                    }
                }
            }
        }
    }
    for (from, path) in new_paths {
        for (node_name, node) in map.iter_mut() {
            if *node_name == from {
                for tunnel in node.connections.iter_mut() {
                    if tunnel.dest == path.dest {
                        tunnel.cost = path.cost;
                    }
                }
            }
        }
    }
    map.remove(name);
}


fn part_a(input: &str) -> i32 {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut map: HashMap<&str, Node> = HashMap::new();
    let mut zero_nodes: Vec<&str> = Vec::new();

    while let Some(line) = data.next() {
        let (name, rest) = line.split_once(" has").unwrap();
        let (_, name) = name.split_once(' ').unwrap();
        let (rate, rest) = rest.split_once(";").unwrap();
        let (_, rate) = rate.split_once("=").unwrap();
        let rate: i32 = rate.parse().unwrap();
        let (_, con_str) = rest.split_once("valves ").unwrap();

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
            zero_nodes.push(name);
        }
        if name == "AA" {
            node.state = true;
        }
        map.insert(name, node);
    }

    println!("Starting map:");
    for line in map.iter() {
        println!("{:?}", line);
    }
    println!();
    for node_to_remove in zero_nodes {
        println!("Removing {}", node_to_remove);
        remove_by_name(&mut map, node_to_remove);
        println!("Result");
        for line in map.iter() {
            println!("{:?}", line);
        }
    }

    let time0 = 0;
    let flow_rate = 0;
    let total_flow = 0;
    let position = "AA";
    for this_move in map.get(position).unwrap().to_owned().connections {
        //let time = time0 + this_move.cost;
        make_move(&map, position, &this_move, total_flow, flow_rate, time0, &mut score);
    }

    return score
}


const MAX_TIME: i32 = 30;

fn make_move(map: & HashMap<&str, Node>, position: &str, this_move: &Connection, total_flow: i32, flow_rate: i32, time: i32, score: &mut i32) {
    let new_time= time + this_move.cost;
    if new_time > MAX_TIME {
        //println!("{}", time);
        let total_flow = total_flow + (MAX_TIME-time)*flow_rate;
        //println!("over time {}, {}, {}", new_time, total_flow, flow_rate);
        if total_flow > *score {
            *score = total_flow;
        }
        return;
    }

    //println!("total_flow {}, flow_rate {}, this_move.cost {}", total_flow, flow_rate, this_move.cost.to_owned());
    let total_flow = total_flow + flow_rate*this_move.cost;

    let mut new_map = map.clone();

    //println!("position {} - {}, time {}", position, this_move.dest, new_time);

    let mut new_flow_rate = flow_rate;
    let mut new_pos = position;
    if this_move.dest == position {
        let node = new_map.get_mut(this_move.dest).unwrap();
        node.state = true;
        new_flow_rate += node.rate;
    } else {
        new_pos = this_move.dest;
    }
    
    //for line in new_map.iter() {
    //    println!("{:?}", line);
    //}
    
    //println!("new pos {}", new_pos);
    let mut node = new_map.get(new_pos).unwrap();
    let mut possible_moves = node.connections.clone();
    //println!("possible_moves {:?}", possible_moves);

    if !node.state {
        possible_moves.push(Connection{dest: new_pos, cost: 1}); // add posibility to open vault  
    } else {
        // valve is already open, and we are moving awawy => dont come back
        remove_by_name(&mut new_map, new_pos);
    }

    if possible_moves.is_empty() {
        let mut end_flow = total_flow;
        if new_time < MAX_TIME {
            end_flow += (MAX_TIME - new_time)*new_flow_rate;
        }
        *score = *score.max(&mut end_flow);
        return;
    }
    for move_to_make in possible_moves {
        make_move(&new_map, new_pos, &move_to_make, total_flow, new_flow_rate, new_time, score);
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