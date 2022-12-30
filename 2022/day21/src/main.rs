use std::collections::HashMap;

fn part_a() {
    let input = include_str!("input.txt");
    let mut lines = input.split("\r\n");

    let mut monkeys: HashMap<&str, i64> = HashMap::new();
    let mut op_monkeys: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    while let Some(line) = lines.next() {
        let (name, yell) = line.split_once(": ").unwrap();
        match yell.parse::<i64>() {
            Ok(x) => {
                monkeys.insert(name, x);
            },
            Err(_) => {
                let (var1, part2) = yell.split_once(" ").unwrap();
                let (op, var2) = part2.split_once(" ").unwrap();
                op_monkeys.insert(name, (var1, op, var2));
                // split up in operations
            },
        };
    }

    while !op_monkeys.is_empty() {
        let mut removed = Vec::new();
        for (name, (n1, op, n2)) in op_monkeys.iter() {
            if let Some(num1) = monkeys.get(n1) {
                if let Some(num2) = monkeys.get(n2) {
                    let mut val = 0;
                    match *op {
                        "+" => {
                            val = num1 + num2;
                        },
                        "-" => {
                            val = num1 - num2;
                        },
                        "*" => {
                            val = num1 * num2;
                        },
                        "/" => {
                            val = num1 / num2;
                        },
                        _ => panic!()
                    }
                    removed.push(*name);
                    monkeys.insert(name, val);
                }
            }
        }
        for rem in removed {
            op_monkeys.remove(rem);
        }
    }

    
    println!("Root says: {}", monkeys.get("root").unwrap());
}


fn main() {
    let input = include_str!("inputb.txt");
    let mut lines = input.split("\r\n");

    let mut orig_monkeys: HashMap<&str, i64> = HashMap::new();
    let mut orig_op_monkeys: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    while let Some(line) = lines.next() {
        let (name, yell) = line.split_once(": ").unwrap();
        if name == "humn" {
            continue;   // skip myself
        }
        match yell.parse::<i64>() {
            Ok(x) => {
                orig_monkeys.insert(name, x);
            },
            Err(_) => {
                let (var1, part2) = yell.split_once(" ").unwrap();
                let (op, var2) = part2.split_once(" ").unwrap();
                orig_op_monkeys.insert(name, (var1, op, var2));
                // split up in operations
            },
        };
    }

    // shorten amount of operations. 
    for _  in 0..1000 {
        let mut removed = Vec::new();
        for (name, (n1, op, n2)) in orig_op_monkeys.iter() {
            if let Some(num1) = orig_monkeys.get(n1) {
                if let Some(num2) = orig_monkeys.get(n2) {
                    let mut val = 0;
                    match *op {
                        "+" => {
                            val = num1 + num2;
                        },
                        "-" => {
                            val = num1 - num2;
                        },
                        "*" => {
                            val = num1 * num2;
                        },
                        "/" => {
                            val = num1 / num2;
                        },
                        _ => panic!()
                    }
                    removed.push(*name);
                    orig_monkeys.insert(name, val);
                }
            }
        }
        for rem in removed {
            orig_op_monkeys.remove(rem);
        }
    }

    

    // # brut force
    let mut my_num = 3059361893920; 
    'outer: loop {
        //println!("{}", my_num);
        my_num += 1;
        let mut monkeys = orig_monkeys.clone();
        let mut op_monkeys = orig_op_monkeys.clone();
        monkeys.insert("humn", my_num);
        while !op_monkeys.is_empty() {
            let mut removed = Vec::new();
            for (name, (n1, op, n2)) in op_monkeys.iter() {
                if let Some(num1) = monkeys.get(n1) {
                    if let Some(num2) = monkeys.get(n2) {
                        let mut val = 0;
                        match *op {
                            "+" => {
                                val = num1 + num2;
                            },
                            "-" => {
                                val = num1 - num2;
                            },
                            "*" => {
                                val = num1 * num2;
                            },
                            "/" => {
                                val = num1 / num2;
                            },
                            "=" => {
                                println!("{} == {}", num1, num2);
                                if num1 == num2 { 
                                    break 'outer;
                                } else {
                                    continue 'outer;
                                }
                            }
                            _ => panic!()
                        }
                        removed.push(*name);
                        monkeys.insert(name, val);
                    }
                }
            }
            for rem in removed {
                op_monkeys.remove(rem);
            }
        }
    }

    println!("My number: {}", my_num);
}
