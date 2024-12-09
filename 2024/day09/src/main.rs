use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Memory {
    size: usize,
    id: usize,
}

#[derive(Debug, Clone, Copy)]
enum Partition {
    Free(Memory),
    Used(Memory),
}

fn parse_disc(input: &str) -> (Vec<usize>, Vec<Option<usize>>) {
    let disk_map: Vec<usize> = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .collect();

    let total_size: usize = disk_map.iter().sum();
    let mut disc: Vec<Option<usize>> = vec![None; total_size];

    let mut idx = 0;
    let mut id = 0;
    for (i, &size) in disk_map.iter().enumerate() {
        let fill_value = if i % 2 == 0 { Some(id) } else { None };
        disc[idx..idx + size].fill(fill_value);
        idx += size;
        if i % 2 == 0 {
            id += 1;
        }
    }
    (disk_map, disc)
}

fn part_a(input: &str) -> isize {
    let (disk_map, map) = parse_disc(input);
    let total_size: usize = disk_map.iter().sum();

    let mut checksum = 0;
    let mut back_idx = total_size - 1;
    let length = map.iter().filter(|x| x.is_some()).count();

    for i in 0..length {
        if let Some(id) = map[i] {
            checksum += i * id;
        } else {
            while map[back_idx].is_none() {
                back_idx -= 1;
            }
            checksum += i * map[back_idx].unwrap();
            back_idx -= 1;
        }
    }
    checksum as isize
}

fn _print_disc(disc: &[Partition]) {
    for mem in disc {
        match mem {
            Partition::Free(mem) => {
                for _ in 0..mem.size {
                    print!(".");
                }
            }
            Partition::Used(mem) => {
                for _ in 0..mem.size {
                    print!("{}", mem.id);
                }
            }
        }
    }
    println!();
}

fn do_checksum(disc: &[Partition]) -> isize {
    let mut checksum = 0;
    let mut idx = 0;
    for mem in disc {
        let memory = match mem {
            Partition::Free(memory) => memory,
            Partition::Used(memory) => memory,
        };
        for _ in 0..memory.size {
            checksum += idx * memory.id as isize;
            idx += 1;
        }
    }
    checksum
}

fn part_b(input: &str) -> isize {
    let mut defrag_disc: Vec<Partition> = input
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| {
            c.to_digit(10).map(|n| {
                if idx % 2 == 0 {
                    Partition::Used(Memory {
                        size: n as usize,
                        id: idx / 2,
                    })
                } else {
                    Partition::Free(Memory {
                        size: n as usize,
                        id: 0,
                    })
                }
            })
        })
        .collect();

    // Take backmost package, find its index.
    // Check if there are any free slots before that index with sufficient size
    let mut extra = 0;
    for i in (0..defrag_disc.len()).rev() {
        let ii = i + extra;
        if let Partition::Used(mem) = defrag_disc[ii] {
            for idx in 0..ii {
                match defrag_disc[idx] {
                    Partition::Free(free) => {
                        if free.size >= mem.size {
                            defrag_disc[ii] = Partition::Free(Memory {
                                size: mem.size,
                                id: 0,
                            });
                            defrag_disc[idx] = Partition::Used(mem);
                            if free.size > mem.size {
                                defrag_disc.insert(
                                    idx + 1,
                                    Partition::Free(Memory {
                                        size: free.size - mem.size,
                                        id: 0,
                                    }),
                                );
                                extra += 1;
                            }
                            break;
                        }
                    }
                    Partition::Used(_) => {}
                }
            }
        }
    }
    do_checksum(&defrag_disc)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = part_a(input);
    assert_eq!(ans, 6337921897505);

    let ans = part_b(input);
    assert_eq!(ans, 6362722604045);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(input);
        assert_eq!(ans, 1928);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = part_b(input);
        assert_eq!(ans, 2858);
    }
}
