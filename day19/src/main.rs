use std::{collections::HashMap, hash::Hash, alloc::System, ops::RangeBounds};


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
    let input = include_str!("example.txt");
    let mut lines = input.trim().split("\r\n");
    let mut scanners = Vec::new();

    loop {
        let mut beacons = Vec::new();
        for _ in 0..24 {
            beacons.push(Vec::new());
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
                beacons[i].push(pt);
            }
        }
        scanners.push(beacons);
    }
    

    let mut systems: Vec<HashSet<usize>> = Vec::new();
    
    loop {
        let mut aligned= Vec::new();
        let mut next_realignment = None;
        'outer: for (i, scanner) in scanners.iter().enumerate() {
            for (j, other_scanner) in scanners.iter().enumerate().skip(i+1) {
                let other_beacons = &other_scanner[0];

                for (rotation_idx, beacons) in scanner.iter().enumerate() {
                    let mut potential_offsets: HashMap<(i64, i64, i64), usize> = HashMap::new();
                    for beacon in beacons {
                        for other_beacon in other_beacons {
                            let offset = (beacon.0 -other_beacon.0, beacon.1 -other_beacon.1, beacon.2 -other_beacon.2);
                        
                        *potential_offsets.entry(offset).or_default() += 1;
                        }
                    }
                    let mut potential_offset: Vec<_> = potential_offsets
                        .into_iter()
                        .filter(|(_, count)| *count >= 12)
                        .map(|(offset, _)| offset)
                        .collect();

                    if let Some(offset) = potential_offset.pop() {
                        aligned.push((i, j, rotation_idx, offset));
                        if rotation_idx != 0 {
                            next_realignment = Some((i, j, rotation_idx));
                            break 'outer;
                        }
                    }
                    eprintln!("potential offset: {:?}", potential_offset)
                }
            }
        }
        if let Some((i, j, rot_idx)) = next_realignment {
            eprintln!("next");
            let system = systems.iter()
                .filter(|items| items.contains(i) || items.contains(j));
        } else {
            eprint!("{:#?}", aligned);
            break;
        }
    }

    //eprintln!("{}", systems.len());
}


fn append_scanner(map: &mut HashMap<(i64, i64, i64), i64>, scanner: &Vec<(i64, i64, i64)>) {
    for pos in scanner.iter() {
        // avoid duplicate inputs
        if map.contains_key(pos) {
            continue;
        } 
        map.insert(*pos, 1);
    }
}