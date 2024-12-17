use std::time::Instant;

#[derive(Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl OpCode {
    fn try_from(value: u64) -> Self {
        match value {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => unreachable!(),
        }
    }

    fn call(&self, program_pointer: &mut usize, rhs: u64, reg: &mut [u64]) -> Option<u64> {
        *program_pointer += 2;
        let combo_op = get_combo_op(rhs, reg);
        match self {
            OpCode::Adv => reg[0] /= 2_u64.pow(combo_op.unwrap() as u32),
            OpCode::Bxl => reg[1] ^= rhs,
            OpCode::Bst => reg[1] = combo_op.unwrap() % 8,
            OpCode::Jnz => {
                if reg[0] != 0 {
                    *program_pointer = rhs as usize;
                }
            }
            OpCode::Bxc => reg[1] ^= reg[2],
            OpCode::Out => return Some(combo_op.unwrap() % 8),
            OpCode::Bdv => reg[1] = reg[0] / 2_u64.pow(combo_op.unwrap() as u32),
            OpCode::Cdv => reg[2] = reg[0] / 2_u64.pow(combo_op.unwrap() as u32),
        }
        None
    }
}

fn get_combo_op(value: u64, registers: &[u64]) -> Option<u64> {
    match value {
        0 => Some(0),
        1 => Some(1),
        2 => Some(2),
        3 => Some(3),
        4 => Some(registers[0]),
        5 => Some(registers[1]),
        6 => Some(registers[2]),
        7 => None,
        _ => unreachable!(),
    }
}

fn run_program(program: Vec<u64>, registers: &mut [u64]) -> Vec<u64> {
    let mut program_pointer = 0;
    let mut output = Vec::new();

    while program_pointer < program.len() - 1 {
        let opcode: OpCode = OpCode::try_from(program[program_pointer]);
        let combo_op = program[program_pointer + 1];
        let opcode = opcode.call(&mut program_pointer, combo_op, registers);
        if let Some(out) = opcode {
            output.push(out);
        }
    }
    output
}

fn part_a(input: &str) -> Vec<u64> {
    let (registers, program) = input.split_once("\r\n\r\n").unwrap();
    let mut registers: Vec<u64> = registers
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse::<u64>().unwrap())
        .collect();
    let program = program
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    run_program(program, &mut registers)
}

fn solve_program(program: &Vec<u64>, pos: usize, a: u64) -> bool {
    let mut reg: Vec<u64> = vec![a, 0, 0];
    for p in 0..(program.len() / 2) {
        let opcode: OpCode = OpCode::try_from(program[p * 2]);
        let rhs = program[p * 2 + 1];
        let combo_op = get_combo_op(rhs, &reg);
        match opcode {
            OpCode::Adv => reg[0] /= 2_u64.pow(combo_op.unwrap() as u32),
            OpCode::Bxl => reg[1] ^= rhs,
            OpCode::Bst => reg[1] = combo_op.unwrap() % 8,
            OpCode::Jnz => return solve_program(program, pos + 1, reg[0]),
            OpCode::Bxc => reg[1] ^= reg[2],
            OpCode::Out if (combo_op.unwrap() % 8) != program[pos] => return false,
            OpCode::Out if pos == (program.len() - 1) => return true,
            OpCode::Out => {}
            OpCode::Bdv => reg[1] = reg[0] / 2_u64.pow(combo_op.unwrap() as u32),
            OpCode::Cdv => reg[2] = reg[0] / 2_u64.pow(combo_op.unwrap() as u32),
        }
    }
    false
}

fn part_b(input: &str) -> Option<u64> {
    let (_registers, program) = input.split_once("\r\n\r\n").unwrap();
    let program = program
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut search: Vec<u64> = vec![0];

    for target in (0..program.len()).rev() {
        let mut next: Vec<u64> = Vec::new();
        for reg_a in search.iter().flat_map(|a| (0..8).map(move |i| a + i)) {
            if solve_program(&program, target, reg_a) {
                if target == 0 {
                    return Some(reg_a);
                }
                next.push(reg_a << 3);
            }
        }
        search = next;
    }
    None
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    assert_eq!(ans_a, [7, 3, 0, 5, 7, 1, 4, 0, 5]);
    let ans_b = part_b(input);
    assert_eq!(ans_b, Some(202972175280682));

    println!("Time: {} us", start.elapsed().as_micros());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(input);
        assert_eq!(ans, [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn example_a1() {
        let mut registers = vec![0, 0, 9];
        let program = vec![2, 6];
        let _output = run_program(program, &mut registers);
        assert_eq!(registers[1], 1);
    }

    #[test]
    fn example_a2() {
        let mut registers = vec![10, 0, 0];
        let program = vec![5, 0, 5, 1, 5, 4];
        let output = run_program(program, &mut registers);
        assert_eq!(output, [0, 1, 2]);
    }

    #[test]
    fn example_a3() {
        let mut registers = vec![2024, 0, 0];
        let program = vec![0, 1, 5, 4, 3, 0];
        let output = run_program(program, &mut registers);
        assert_eq!(output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(registers[0], 0);
    }

    #[test]
    fn example_a4() {
        let mut registers = vec![0, 29, 0];
        let program = vec![1, 7];
        let _output = run_program(program, &mut registers);
        assert_eq!(registers[1], 26);
    }

    #[test]
    fn example_a5() {
        let mut registers = vec![0, 29, 0];
        let program = vec![1, 7];
        let _output = run_program(program, &mut registers);
        assert_eq!(registers[1], 26);
    }

    #[test]
    fn example_a6() {
        let mut registers = vec![0, 2024, 43690];
        let program = vec![4, 0];
        let _output = run_program(program, &mut registers);
        assert_eq!(registers[1], 44354);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example_b.txt");
        let ans = part_b(input);
        assert_eq!(ans, Some(117440));
    }
}
