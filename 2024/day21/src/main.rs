use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::time::Instant;


const NUMPAD: &[&[u8]] = &[b"789", b"456", b"123", b" 0A"];
const ARROWS: &[&[u8]] = &[b" ^A", b"<v>"];

fn pad_move(r: usize, c: usize, m: u8, pad: &[&[u8]]) -> (usize, usize, Option<u8>) {
    match m {
        b'<' => (r, c - 1, None),
        b'^' => (r - 1, c, None),
        b'>' => (r, c + 1, None),
        b'v' => (r + 1, c, None),
        b'A' => (r, c, Some(pad[r][c])),
        _ => unreachable!(),
    }
}

fn calculate_cost(cache: &mut HashMap<(u8, u8, usize), usize>, goal: u8, prev_m: u8, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    if let Some(&d) = cache.get(&(goal, prev_m, depth)) {
        return d;
    }
    let start = match prev_m {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => unreachable!(),
    };
    let mut q = BinaryHeap::from([(0, start, b'A', 0)]);
    while let Some((d, (r, c), prev, out)) = q.pop() {
        let d = (-d) as usize;
        if out == goal {
            cache.insert((goal, prev_m, depth), d);
            return d;
        }
        for &m in b"A^<v>" {
            let (rr, cc, x) = pad_move(r, c, m, ARROWS);
            if *ARROWS.get(rr).and_then(|row| row.get(cc)).unwrap_or(&b' ') == b' ' {
                continue;
            }
            let x = x.unwrap_or(0);
            if x != 0 && x != goal {
                continue;
            }
            let d = d + calculate_cost(cache, m, prev, depth - 1);
            q.push((-(d as i64), (rr, cc), m, x));
        }
    }
    unreachable!()
}

fn find_moves(cache: &mut HashMap<(u8, u8, usize), usize>, code: &[u8], depth: usize) -> usize {
    let mut q = BinaryHeap::from([(0, (3, 2), b'A', 0)]);
    let mut seen = HashMap::new();
    while let Some((d, (row, col), prev, l)) = q.pop() {
        let d = (-d) as usize;
        if l == code.len() {
            return d;
        }
        let k = ((row, col), prev, l);
        if seen.contains_key(&k) {
            continue;
        }
        seen.insert(k, d);
        for &m in b"A^<v>" {
            let (rr, cc, x) = pad_move(row, col, m, NUMPAD);
            if *NUMPAD.get(rr).and_then(|row| row.get(cc)).unwrap_or(&b' ') == b' ' {
                continue;
            }
            let mut l = l;
            if let Some(x) = x {
                if x != code[1] {
                    continue;
                }
                l += 1;
            }
            let d = d + calculate_cost(cache, m, prev, depth);
            q.push((-(d as i64), (rr, cc), m, l));
        }
    }
    unreachable!();
}
fn solve(input: &str) -> (usize, usize) {
    let (mut ans_a, mut ans_b) = (0, 0);
    let mut cache = HashMap::new();
    for code in input.lines() {
        let n = code.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        ans_a = n * find_moves(&mut cache, code.as_bytes(), 2);
    }
    (ans_a, ans_b)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("example.txt");
    let ans = solve(input);
    assert_eq!(ans.0, 1393);
    assert_eq!(ans.1, 990096);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a1() {
        let input = "029A";
        let ans = solve(input);
        println!("Human answer  : <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
        assert!(ans.0 == 68*291);
    }
    
    #[test]
    fn example_a2() {
        let input = "980A";
        let ans = solve(input);
        assert!(ans.0 == 60*980);
    }

    #[test]
    fn example_a3() {
        let input = "179A";
        let ans = solve(input);
        assert!(ans.0 == 68*179);
    }

    #[test]
    fn example_a4() {
        let input = "456A";
        let ans = solve(input);
        assert!(ans.0 == 64*456);
    }

    #[test]
    fn example_a5() {
        let input = "379A";
        let ans = solve(input);
        println!("Human answer  : <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");
        assert!(ans.0 == 64*379);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert_eq!(ans.1, 0);
    }
}
