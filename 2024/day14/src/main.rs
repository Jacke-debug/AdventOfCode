use std::time::Instant;

#[derive(Debug, Default)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Default)]
struct Robot {
    pos: Pos,
    vel: Pos,
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let (pos, vel) = line.split_once(" ").unwrap();
        let (pos_x, pos_y) = pos.split_once("=").unwrap().1.split_once(",").unwrap();
        let (vel_x, vel_y) = vel.split_once("=").unwrap().1.split_once(",").unwrap();
        robots.push(Robot {
            pos: Pos {
                x: pos_x.parse::<isize>().unwrap(),
                y: pos_y.parse::<isize>().unwrap(),
            },
            vel: Pos {
                x: vel_x.parse::<isize>().unwrap(),
                y: vel_y.parse::<isize>().unwrap(),
            },
        });
    }
    robots
}

fn modulus(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}

fn part_a(input: &str, size: (isize, isize)) -> usize {
    let steps = 100;
    let (width, height) = size;
    let mut robots = parse_input(input);
    for robot in robots.iter_mut() {
        robot.pos.x = modulus(robot.pos.x + robot.vel.x * steps, width);
        robot.pos.y = modulus(robot.pos.y + robot.vel.y * steps, height);
    }
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;
    let mut count4 = 0;
    for y in 0..height {
        for x in 0..width {
            let c = robots
                .iter()
                .filter(|robot| robot.pos.x == x && robot.pos.y == y)
                .count();
            match y {
                _ if y < height / 2 => match x {
                    _ if x < width / 2 => count1 += c,
                    _ if x > width / 2 => count2 += c,
                    _ => {}
                },
                _ if y > height / 2 => match x {
                    _ if x < width / 2 => count3 += c,
                    _ if x > width / 2 => count4 += c,
                    _ => {}
                },
                _ => {}
            }
        }
    }
    count1 * count2 * count3 * count4
}

fn part_b(input: &str, size: (isize, isize)) -> usize {
    let (width, height) = size;
    let mut robots = parse_input(input);
    let mut steps = 0;
    let cluster_limit = 500;
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    loop {
        steps += 1;
        for robot in robots.iter_mut() {
            robot.pos.x = modulus(robot.pos.x + robot.vel.x, width);
            robot.pos.y = modulus(robot.pos.y + robot.vel.y, height);
        }
        let mut cluster = 0;
        for r1 in robots.iter() {
            for dir in dirs.iter() {
                cluster += robots
                    .iter()
                    .filter(|r| r.pos.x == r1.pos.x + dir.0 && r.pos.y == r1.pos.y + dir.1)
                    .count()
            }
        }
        if cluster > cluster_limit {
            break;
        }
    }
    for y in 0..height {
        for x in 0..width {
            let c = robots
                .iter()
                .filter(|robot| robot.pos.x == x && robot.pos.y == y)
                .count();
            if c == 0 {
                print!(".");
            } else {
                print!("{}", c);
            };
        }
        println!();
    }
    steps
}

fn main() {
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = part_a(input, (WIDTH, HEIGHT));
    assert_eq!(ans, 225943500);

    let ans = part_b(input, (WIDTH, HEIGHT));
    assert_eq!(ans, 6377);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        const WIDTH: isize = 11;
        const HEIGHT: isize = 7;
        let input = include_str!("example.txt");
        let ans = part_a(input, (WIDTH, HEIGHT));
        assert_eq!(ans, 12);
    }
}
