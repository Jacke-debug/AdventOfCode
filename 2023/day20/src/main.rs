use std::collections::{HashMap, VecDeque};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn smallest_common_divisor(numbers: &Vec<u64>) -> u64 {
    if numbers.is_empty() {
        return 0; // or handle the case appropriately
    }
    let result = numbers.iter().fold(numbers[0], |acc, &num| lcm(acc, num));
    result
}


#[derive(Debug, Clone)]
struct node<'a> {
    component: &'a str, 
    state: bool, 
    inputs: HashMap<&'a str, bool>,
    receivers: Vec<&'a str>,
}



fn part_a(input: &str) -> (isize, isize) {
    let mut nodes: HashMap<&str, node<'_>> = HashMap::new();
    let mut output_nodes = HashMap::new();

    for line in input.lines() {
        let (sender, receivers) = line.split_once(" -> ").unwrap();
        let name: &str = sender.get(1..).unwrap();

        let receivers: Vec<&str> =  receivers.split(", ").collect();
        if receivers.contains(&"hf") {
            output_nodes.insert(name, 0);
        }

        match sender.chars().nth(0) {
            Some('%') => {
                nodes.insert(name, node {
                    component: "flip_flop",
                    state: false,
                    inputs: HashMap::new(),
                    receivers: receivers,
                });
            }
            Some('&') => {
                nodes.insert(name, node {
                    component: "conjunction",
                    state: false,
                    inputs: HashMap::new(),
                    receivers: receivers,
                });
            }
            Some('b') => {
                let mut b_node = node {
                    component: "roadcaster",
                    state: false,
                    inputs: HashMap::new(),
                    receivers: receivers,
                };
                let mut button_node = node {
                    component: "roadcaster",
                    state: false,
                    inputs: HashMap::new(),
                    receivers: vec!["roadcaster"],
                };
                b_node.inputs.insert("button", false);
                nodes.insert("roadcaster", b_node);
                nodes.insert("button", button_node);
            }
            Some(_) => unreachable!(),
            None => unreachable!(),
        }
    }

    
    let binding = nodes.clone();
    for (sender_id, sender) in binding.iter() {
        for rec_id in &sender.receivers {
            let node = match nodes.get_mut(rec_id) {
                Some(x) => {x},
                None => {
                    // a node that does not send further, pulse dies here
                    continue;
                },
            };
            node.inputs.insert(sender_id, false);
        }
        
    }
    let mut high_pulses = 0;
    let mut low_pulses = 0;

    //output_node.inputs

    'outer: for idx in 0..100000 {
        let mut stack: VecDeque<(&str, bool)> = VecDeque::from(vec![("button", false)]);
        // low -> broadcaster
        while let Some((sender_id, pulse)) = stack.pop_front() {
            let sender = nodes.get(sender_id).unwrap().clone();
            
            for rec_id in sender.receivers.iter() {
                if pulse {
                    high_pulses += 1;
                    match output_nodes.get_mut(sender_id) {
                        Some(iters) => {
                            if *iters == 0 {
                                *iters = idx+1; // number of button presses
                            }
                        }
                        None =>{}
                    }
                } else {
                    low_pulses += 1;
                }
                let receiver= match nodes.get_mut(rec_id) {
                    Some(x) => {x},
                    None => {
                        if !pulse {
                            break 'outer;
                        }
                        continue;
                    },
                };
                let output = match receiver.component {
                    "roadcaster" => {
                        // broadcast, send any received
                        Some(pulse)
                    }
                    "flip_flop" => {
                        // % flip-flop, init off, high ignore, low -> switch between on and off
                        if pulse {
                            None
                        } else {
                            receiver.state = !receiver.state;
                            Some(receiver.state)
                        }
                    }
                    "conjunction" => {
                        // & conjunction, remember last pulse, init low, high for all -> send low, else send_low ->
                        match receiver.inputs.get_mut(sender_id) {
                            Some(val) => {
                                *val = pulse;
                            },
                            None => {
                                println!("{:?}", rec_id);
                                panic!()},
                        }
                        if receiver.inputs.values().all(|v| *v) {
                            Some(false)
                        } else {
                            Some(true)
                        }
                    }
                    _ => panic!()
                };
                match output {
                    Some(b) => {
                        stack.push_back((rec_id, b))
                    }
                    None => {continue}
                }
            }
        }
    }

    println!("{:?}", output_nodes);
    let vec: Vec<u64> = output_nodes.values().map(|x| *x as u64).collect();
    let scd = smallest_common_divisor(&vec);
    let mut ans_b = scd as isize; // 987608569081 low
    return (low_pulses*high_pulses, ans_b)
}

fn main() {
    let input = include_str!("input.txt");
    let (ans_a, ans_b)= part_a(input);
    println!("Part A: {}", ans_a);
    println!("Part B: {}", ans_b);
    // 807069600
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), (32000000, 0));
    }
}