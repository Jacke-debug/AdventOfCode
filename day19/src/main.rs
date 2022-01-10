use std::{collections::{HashMap, HashSet}};


fn roll(mut pt: (i64, i64, i64)) -> (i64, i64, i64) {
    (pt.0, pt.2, -pt.1)
}

fn turn(mut pt: (i64, i64, i64)) -> (i64, i64, i64) {
    (-pt.1, pt.0, pt.2)
}

fn rotations(mut pt: (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let mut pts = Vec::new();

    for _cycle in 0..2 {
        for _step in 0..3 {
            pt = roll(pt);
            pts.push(pt);
            for _i in 0..3 {
                pt = turn(pt);
                pts.push(pt);
            }
        }
        pt = roll(turn(pt));
    }
    
    pts
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.trim().split("\r\n");
    let mut scanners = Vec::new();

    loop {
        let mut beacons = Vec::new();
        for _ in 0..24 {
            beacons.push(HashSet::new());
        }
        if lines.next().is_none() {
            break;
        }
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut xyz = line.split(',');
            let x = xyz.next().unwrap().parse::<i64>().unwrap();
            let y = xyz.next().unwrap().parse::<i64>().unwrap();
            let z = xyz.next().unwrap().parse::<i64>().unwrap();
            for (i, pt) in rotations((x, y, z)).into_iter().enumerate() {
                beacons[i].insert(pt);
            }
        }
        scanners.push(beacons);
    }
    
    let mut scanner_offsets = Vec::new();
    let mut map = HashSet::new();
    let pos = &scanners[0];
    for p in pos[0].iter() {
        map.insert(*p);
    }
    println!("{}", map.len());
    scanners.remove(0);
    check_scanners(&mut map, &mut scanners, &mut scanner_offsets);

    while scanners.len() != 0 {
        let mut sub_map = scanners.pop().unwrap().pop().unwrap();
        check_scanners(&mut sub_map, &mut scanners, &mut scanner_offsets);

        let mut beacons = Vec::new();
        for _ in 0..24 {
            beacons.push(HashSet::new());
        }

        for (x, y, z) in sub_map.iter() {
            for (i, pt) in rotations((*x, *y, *z)).into_iter().enumerate() {
                beacons[i].insert(pt);
            }
        }
        scanners.push(beacons);

        check_scanners(&mut map, &mut scanners, &mut scanner_offsets);
    }
    println!("Total numbers of beacons: {}", map.len()); // 372

    let mut longest = 0;
    // part B, largest distance between 
    for key in scanner_offsets.iter() {
        for other_key in scanner_offsets.iter() {
            let dist = distance(*key, *other_key);
            if dist > longest {
                longest = dist;
            }
        }
    }
    println!("Longest Manhattan distance: {}", longest); // 12241
}

fn distance(beacon_a: (i64, i64, i64), beacon_b: (i64, i64, i64)) -> i64{
    (beacon_a.0 - beacon_b.0).abs() + (beacon_a.1 - beacon_b.1).abs() + (beacon_a.2 - beacon_b.2).abs() 
}

fn check_scanners(map: &mut HashSet<(i64, i64, i64)>, scanners: &mut Vec<Vec<HashSet<(i64, i64, i64)>>>, scanner_offsets: &mut Vec<(i64, i64 ,i64)>) {
    loop{
        let mut prev_length = scanners.len();
        let mut match_found = false;
        let mut index = 0;
        'inner: for (idx, scanner_sets) in scanners.iter_mut().enumerate() {
            
            for other_beacons in scanner_sets.iter() {
                let mut potential_offsets: HashMap<(i64, i64, i64), usize> = HashMap::new();

                for beacon in map.iter() {
                    for other_beacon in other_beacons {
                        let offset = (
                            beacon.0 - other_beacon.0,
                            beacon.1 - other_beacon.1,
                            beacon.2 - other_beacon.2,
                        );
                        *potential_offsets.entry(offset).or_default() += 1;
                    }
                }
                
                let mut potential_offsets: Vec<_> = potential_offsets
                            .into_iter()
                            .filter(|entry| entry.1 >= 12)
                            .map(|(offset, _)| offset)
                            .collect();

                assert!(potential_offsets.is_empty() || potential_offsets.len() == 1);
                if let Some(offset) = potential_offsets.pop() {
                    println!("Matching set found!");
                    scanner_offsets.push(offset);
                    let mut count = 0;
                    for other_beacon in other_beacons.iter() {
                        let new_beacon = (other_beacon.0 + offset.0, other_beacon.1 + offset.1, other_beacon.2 + offset.2);
                        
                        if map.contains(&new_beacon){
                            count += 1;
                        } else {
                            map.insert(new_beacon);
                        }
                    }
                    println!("{} matching beacons", count);
                    match_found = true;
                    index = idx;
                    break 'inner
                }
            }
        }
        if match_found {
            scanners.remove(index);
            println!("Scanners left: {}", scanners.len());
        }
        if scanners.len() == prev_length {
            break;
        }
    }
}
