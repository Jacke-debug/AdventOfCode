use std::collections::HashMap;

// broadcast, send any received 

// & conjunction, remember last pulse, init low, high for all -> send low, else send_low ->
struct conjunction<'a> {
    name: &'a str,
    last_pulse: bool, 
    inputs: Vec<bool>,
}

// % flip-flop, init off, high ignore, low -> switch between on and off
#[derive(Debug, Clone)]
struct node<'a> {
    name: &'a str,
    component: &'a str, 
    state: bool, 
    inputs: HashMap<&'a str, bool>,
}


fn part_a(input: &str) -> isize {
    let mut nodes: HashMap<&str, node<'_>> = HashMap::new();
    let mut receiver_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        println!("{}", line);
        let (sender, receivers) = line.split_once(" -> ").unwrap();
        let name: &str = sender.get(1..).unwrap();

        for receiver in  receivers.split(", ") {
            match receiver_map.get_mut(&receiver) {
                Some(vec) => vec.push(receiver),
                None => {
                    receiver_map.insert(name, vec![receiver]);
                }
            }
        }

        match sender.chars().nth(0) {
            Some('%') => {
                nodes.insert(name, node {
                    name,
                    component: "flip_flop",
                    state: false,
                    inputs: HashMap::new(),
                });
            }
            Some('&') => {
                nodes.insert(name, node {
                    name,
                    component: "conjunction",
                    state: false,
                    inputs: HashMap::new(),
                });
            }
            Some('b') => {
                nodes.insert("roadcaster", node {
                    name: "roadcaster",
                    component: "roadcaster",
                    state: false,
                    inputs: HashMap::new(),
                });
            }
            Some(_) => unreachable!(),
            None => unreachable!(),
        }
    }

    for (node_id, senders) in receiver_map.iter() {
        let node = match nodes.get_mut(node_id) {
            Some(x) => {x},
            None => {
                println!("Unkown rec {}", node_id);
                continue;
            },
        };
        for sender in senders {
            node.inputs.insert(sender, false);
        }
    }

    println!();
    for _ in 0..1000 {
        // low -> broadcaster
        let active_node = receiver_map.get("roadcaster").unwrap();
        for node_id in active_node {

        }
    }
    println!("{:?}", nodes);

    return 0
}


fn main() {
    let input = include_str!("example.txt");
    let ans_a= part_a(input);
    println!("Part A: {}", ans_a);
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), 32000000);
    }
}