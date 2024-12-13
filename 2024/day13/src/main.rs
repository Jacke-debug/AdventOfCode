use std::time::Instant;

#[derive(Debug, Default)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Default)]
struct ClawMachine {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
}

fn parse_coordinates(line: &str) -> Coord {
    let (_, coordinates) = line.split_once("X+").unwrap();
    let (x_part, y_part) = coordinates.split_once(", Y+").unwrap();
    let x = x_part.parse::<isize>().unwrap();
    let y = y_part.parse::<isize>().unwrap();
    Coord { x, y }
}

fn parse_prize(line: &str) -> Coord {
    let (_, coordinates) = line.split_once("X=").unwrap();
    let (x_part, y_part) = coordinates.split_once(", Y=").unwrap();
    let x = x_part.parse::<isize>().unwrap();
    let y = y_part.parse::<isize>().unwrap();
    Coord { x, y }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut claw_machines = Vec::new();
    let mut claw_machine = ClawMachine::default();
    let mut lines = input.lines();
    while let Some(mut line) = lines.next() {
        if line.is_empty() {
            claw_machines.push(claw_machine);
            claw_machine = ClawMachine::default();
            line = lines.next().unwrap();
        }
        claw_machine.button_a = parse_coordinates(line);
        claw_machine.button_b = parse_coordinates(lines.next().unwrap());
        claw_machine.prize = parse_prize(lines.next().unwrap());
    }
    claw_machines.push(claw_machine);
    claw_machines
}

fn has_integer_solution(claw_machine: &ClawMachine, diff: f64) -> Option<(usize, usize)> {
    let y = claw_machine.prize.y as f64 + diff;
    let x = claw_machine.prize.x as f64 + diff;
    let xa = claw_machine.button_a.x as f64;
    let ya = claw_machine.button_a.y as f64;
    let xb = claw_machine.button_b.x as f64;
    let yb = claw_machine.button_b.y as f64;

    let n1 = (y - x * yb / xb) / (ya - xa * yb / xb);
    let n2 = (x - n1 * xa) / xb;

    let n1 = n1.round();
    let n2 = n2.round();
    match n1 * xa + n2 * xb == x && n1 * ya + n2 * yb == y {
        true => Some((n1 as usize, n2 as usize)),
        false => None
    }
}


fn solve(input: &str) -> (usize, usize) {
    const COST_A: usize = 3;
    const COST_B: usize = 1;
    let claw_machines = parse_input(input);

    let (ans_a, ans_b) = claw_machines.iter().fold((0, 0), |(mut acc_a, mut acc_b), machine| {
        [0.0, 10_000_000_000_000.0].iter().for_each(|&diff| {
            if let Some((n1, n2)) = has_integer_solution(machine, diff) {
                let cost = n1 * COST_A + n2 * COST_B;
                if diff > 1.0 {
                    acc_b += cost;
                } else if n1 <= 100 && n2 <= 100 {
                    acc_a += cost;
                }
            }
        });
        (acc_a, acc_b)
    });

    (ans_a, ans_b)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = solve(input);
    assert_eq!(ans.0, 31897);

    let ans = solve(input);
    assert_eq!(ans.1, 87596249540359);

    println!("Time: {} us", start.elapsed().as_micros());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert_eq!(ans.0, 480);
    }
}
