use std::{collections::{HashMap, HashSet}, time::Instant};

fn apply_gravity(map: &mut HashMap<usize, (Vec<isize>, Vec<isize>, Vec<isize>)>) -> usize{
    let mut fallen = HashSet::new();
    let mut updated = true;
    'next: while updated {
        updated = false;
        let old_map = map.clone();
        
        let mut updated_idx = None;
        'next_object: for id1 in old_map.keys() {
            let p1 = map.get(id1).unwrap();
            let z = *p1.2.iter().clone().min().unwrap();
            if z < 2 {
                continue 'next_object
            }
            'inner: for (id2, p2) in &old_map{
                if id2 == id1 {
                    continue 'inner
                }
                if *p2.2.iter().max().unwrap() == z-1 {
                    let x_overlap = p2.0.iter().any(|x| p1.0.contains(x));
                    let y_overlap = p2.1.iter().any(|y| p1.1.contains(y));
                    if x_overlap && y_overlap {
                        continue 'next_object;
                    }
                }
            }
            updated_idx = Some(*id1);
            break 'next_object
        }
        match updated_idx {
            Some(id) => {
                fallen.insert(id);
                map.entry(id).and_modify(|p| {
                    for num in &mut p.2 {
                        *num -= 1;
                    }
                });
                updated = true;
            }
            None => {}
        }
    }
    return fallen.iter().count()
}


fn part_a(input: &str) -> (usize, usize) {
    let mut map = HashMap::new();
    for (object_id, line) in input.lines().enumerate() {
        let (start, stop) = line.split_once("~").unwrap();
        let start: Vec<isize> = start.split(',').map(|x| x.parse::<isize>().unwrap()).collect();
        let stop: Vec<isize> = stop.split(',').map(|x| x.parse::<isize>().unwrap()).collect();

        let x = (start[0]..=stop[0]).collect();
        let y = (start[1]..=stop[1]).collect();
        let z = (start[2]..=stop[2]).collect();
        map.insert(object_id, (x, y, z));
    }
    apply_gravity(&mut map);
    println!("Apply gravity done");

    let keys: Vec<usize> = map.keys().cloned().collect();
    let mut ans_b = 0;
    let mut ans_a = 0;

    for item in keys {
        let mut new_map: HashMap<usize, (Vec<isize>, Vec<isize>, Vec<isize>)> = map.clone();
        new_map.remove(&item);
        
        let nr_moved = apply_gravity(&mut new_map);
        ans_b += nr_moved;
        if nr_moved == 0 {
            ans_a += 1;
        }
    }
    return (ans_a, ans_b);
}

fn main() {
    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let (ans_a, ans_b) = part_a(input);
    let elapsed_time = Instant::now() - start_time;
    println!("Elapsed time: {:?}", elapsed_time);
    println!("Part A: {}", ans_a);
    println!("Part B: {}", ans_b);
}
