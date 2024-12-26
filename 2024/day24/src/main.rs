use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Operator {
    And,
    Or,
    Xor,
    Val,
}

#[derive(Clone, Copy, Debug)]
struct Gate<'a> {
    operator: Operator,
    inputs: [&'a str; 2],
    output: Option<bool>,
}

fn eval_gate(id: &str, gates: &mut HashMap<&str, Gate>) -> bool {
    let (op, inputs) = {
        let gate = gates.get(id).unwrap();
        (gate.operator, gate.inputs)
    };
    if let Some(val) = gates.get(id).unwrap().output {
        return val;
    }
    let val_a = eval_gate(inputs[0], gates);
    let val_b = eval_gate(inputs[1], gates);
    let res = match op {
        Operator::And => val_a && val_b,
        Operator::Or => val_a || val_b,
        Operator::Xor => val_a ^ val_b,
        _ => unreachable!(),
    };
    gates.get_mut(id).unwrap().output = Some(res);
    res
}

fn parse_input(input: &str) -> HashMap<&str, Gate> {
    let mut gates = HashMap::new();

    let (init_states, gate_def) = input.split_once("\r\n\r\n").unwrap();
    for l in init_states.lines() {
        let (id, state) = l.split_once(": ").unwrap();
        let state = state != "0";
        gates.insert(
            id,
            Gate {
                operator: Operator::Val,
                inputs: ["---", "---"],
                output: Some(state),
            },
        );
    }

    for line in gate_def.lines() {
        let (input, id) = line.split_once(" -> ").unwrap();
        let logic: Vec<&str> = input.split_whitespace().collect();
        gates.insert(
            id,
            Gate {
                operator: match logic[1] {
                    "AND" => Operator::And,
                    "OR" => Operator::Or,
                    "XOR" => Operator::Xor,
                    _ => unreachable!(),
                },
                inputs: [logic[0], logic[2]],
                output: None,
            },
        );
    }
    gates
}

fn part_a(input: &str) -> usize {
    let mut gates = parse_input(input);
    let mut output_gates: Vec<_> = gates
        .keys()
        .filter(|&id| id.starts_with('z'))
        .copied()
        .collect();
    output_gates.sort();
    output_gates
        .into_iter()
        .rev()
        .fold(0, |acc, id| (acc << 1) | eval_gate(id, &mut gates) as usize)
}

fn part_b(input: &str) -> String {
    let gates = parse_input(input);
    let mut bad = vec![];

    for (id, gate) in gates.iter() {
        let output = id;
        let op = gate.operator;
        let mut ins = gate.inputs;
        ins.sort();
        let [in0, in1] = ins;

        if output.starts_with("z") && !output.ends_with("45") {
            if op != Operator::Xor {
                bad.push(*output);
            }
        } else if !(in0.starts_with("x") || in1.starts_with("y")) {
            if op == Operator::Xor {
                bad.push(*output);
            }
        } else if in0.starts_with("x") && in1.starts_with("y")
            || in0.starts_with("y") && in1.starts_with("x")
        {
            if in0.ends_with("00") || in1.ends_with("00") {
                continue;
            }
            let mut ops = vec![];
            for (_id, g2) in gates.iter() {
                let opb = g2.operator;
                let ins_l2 = g2.inputs.to_vec();
                if ins_l2.contains(output) {
                    ops.push(opb);
                }
            }

            if op == Operator::Xor && !ops.contains(&Operator::Xor)
                || op == Operator::And && !ops.contains(&Operator::Or)
            {
                bad.push(*output);
            }
        }
    }
    bad.sort();
    bad.join(",")
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = part_a(input);
    assert_eq!(ans, 52728619468518);
    let ans = part_b(input);
    assert_eq!(ans, "dck,fgn,nvh,qdg,vvf,z12,z19,z37");
    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(input);
        assert!(ans == 2024);
    }

    #[test]
    fn example_a1() {
        let input = include_str!("example_a.txt");
        let ans = part_a(input);
        assert!(ans == 4);
    }
}
