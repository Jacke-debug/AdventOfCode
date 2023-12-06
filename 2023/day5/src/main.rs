use std::{collections::HashSet, time::Instant};





fn part_a(input: &str) -> u64 {
    let mut score = std::u64::MAX;

    let mut lines = input.trim().split("\r\n");

    let (_, seeds) = lines.next().unwrap().split_once(':').unwrap();
    let seeds: Vec<u64> = seeds.split_whitespace().map(|s| s.parse().unwrap()).collect();

    lines.next();
    lines.next();
    
    let mut map_vec: Vec<Vec<Vec<u64>>>= Vec::new();
    let mut inner_map: Vec<Vec<u64>>= Vec::new();
    while let Some(line) = lines.next() {
        if line == "" {
            lines.next(); // skip header
            map_vec.push(inner_map.clone());
            inner_map = Vec::new();
            continue;
        }

        //println!("{}", line);
        let my_numbers: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        // destination_range_start, source_range_start, range_length
        inner_map.push(my_numbers);
    }
    map_vec.push(inner_map.clone());

    println!("");
    let mut nr = 0;
    for seed in seeds {
        nr = seed;
        //println!("{}", nr);
        'outer: for maps in &map_vec {
            'inner: for mapping in maps {
                
                let source = mapping[1];
                let dest = mapping[0];
                let range = mapping[2];
                if nr >= source && nr <= source+range{
                    //println!("{}, {}, {}", dest, source, range);
                    nr = dest + nr - source;
                    break 'inner
                } 
            }
        }
        score = score.min(nr);
    }

    return score
}


fn part_b(input: &str) -> u64 {
    let mut lines = input.trim().split("\r\n");

    let (_, seeds) = lines.next().unwrap().split_once(':').unwrap();
    let seeds: Vec<u64> = seeds.split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    let mut current_ranges = HashSet::new();
    for chunk in seeds.chunks(2) {
        current_ranges.insert((chunk[0], chunk[0] + chunk[1]));
    }

    //println!("{:?}", current_ranges);
    lines.next();
    lines.next();

    let mut map_vec: Vec<Vec<Vec<u64>>>= Vec::new();
    let mut inner_map: Vec<Vec<u64>>= Vec::new();
    while let Some(line) = lines.next() {
        if line == "" {
            lines.next(); // skip header
            map_vec.push(inner_map.clone());
            inner_map = Vec::new();
            continue;
        }
        let my_numbers: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        // destination_range_start, source_range_start, range_length
        inner_map.push(my_numbers);
    }
    map_vec.push(inner_map.clone());

    for map in map_vec {
        let mut table_splits = HashSet::new();

        for range in current_ranges.iter() {
            let mut range_splits = HashSet::new();
            range_splits.insert(*range);

            for mapping in map.iter() {
                let mut line_splits = HashSet::new();
                let start = mapping[1];
                let dest = mapping[0];
                let last_mapped_val = mapping[1] + mapping[2];

                // For every line, check all of the resulting ranges from the previous line
                for range_split in range_splits.iter() {
                    // If there is an overlap, do the split
                    if range_split.1 > start && last_mapped_val >= range_split.1 || last_mapped_val < range_split.1 && range_split.0 < last_mapped_val {
                        // Range before the overlap
                        if range_split.0 < start {
                            line_splits.insert((range_split.0, start - 1));
                        }
                        // Find the overlap and apply the translation
                        let overlap = (range_split.0.max(start), range_split.1.min(last_mapped_val));
                        table_splits.insert((dest + (overlap.0 - start), dest+(overlap.1 - start)));
                        // Range after the overlap
                        if range_split.1 > last_mapped_val {
                            line_splits.insert((last_mapped_val, range_split.1));
                        }
                    } else {
                        // no overlap, keep existing ranges
                        line_splits.insert(*range_split);
                    }
                }
                range_splits = line_splits.clone();
                line_splits.clear();
            }

            table_splits.extend(range_splits.clone());
            range_splits.clear();
        }
        current_ranges = table_splits.clone();
        table_splits.clear();
    }

    return current_ranges.iter().map(|x| x.0).min().unwrap();
}


fn main() {
    
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {:?}", ans_a);

    
    let start_time = Instant::now();
    let ans_b = part_b(input);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Part B: {:?}", ans_b);

    
    // Calculate the elapsed time
    
    println!("Elapsed time: {}.{:03} seconds", elapsed_time.as_secs(), elapsed_time.subsec_millis());

}
