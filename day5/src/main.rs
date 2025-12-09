use std::collections::HashSet;
use std::ops::RangeInclusive;

fn read_input() -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut fresh_ids = Vec::new();
    let mut ingredient_ids = Vec::new();
    let mut line = String::new();
    std::fs::read_to_string("src/input.txt").unwrap().lines().for_each(|l| {
        line = l.to_string();
        if line.contains("-") {
            let t_range = line.split("-").collect::<Vec<&str>>();
            let start = t_range[0].parse::<u64>().unwrap();
            let end = t_range[1].parse::<u64>().unwrap();
            let range = start..=end;
            fresh_ids.push(range);
        } else if line.len() > 0 {
            ingredient_ids.push(line.parse::<u64>().unwrap());
        }
    });
    (fresh_ids, ingredient_ids)
}

/**
This doesn't work because the ranges are not necessarily disjoint, so we can't just create a set from the smallest to the largest number in the vector 
**/
fn read_input_broken() -> (HashSet<u64>, Vec<u64>) {
    let mut ingredient_ids = Vec::new();
    let mut line = String::new();
    let mut range = Vec::new();
    std::fs::read_to_string("src/test.txt").unwrap().lines().for_each(|l| {
        line = l.to_string();
        if line.contains("-") {
            // first create the vector of start/end ranges
            let t_range = line.split("-").collect::<Vec<&str>>();
            // start
            range.push(t_range[0].parse::<u64>().unwrap());
            // end
            range.push(t_range[1].parse::<u64>().unwrap());
        } else if line.len() > 0 {
            ingredient_ids.push(line.parse::<u64>().unwrap());
        }
    });
    // sort the vector
    range.sort();
    // make a new set from the smallest to the largest number in the vector
    let final_range = range[0]..range[range.len() -1] + 1;
    println!("Final Range: {:?}", final_range);
    let fresh_ids = final_range.into_iter().collect::<HashSet<u64>>();
    println!("Fresh Ids {:?}", fresh_ids);
    println!("Ingredient Ids: {:?}", ingredient_ids);
    let a = range.contains(&ingredient_ids[0]);
    println!("A in range {}", a);
    (fresh_ids, ingredient_ids)
}

/**
Reads the input and returns a Set of fresh IDs, and a list of available ingredient IDs
Lines with a - are ranges of fresh IDs
Lines after the newline are ingredient IDs

This works, but it's incredibly slow.
**/
fn read_input_slow() -> (HashSet<u64>, Vec<u64>) {
    let mut fresh_ids = HashSet::new();
    let mut ingredient_ids = Vec::new();
    let mut line = String::new();
    std::fs::read_to_string("src/input.txt").unwrap().lines().for_each(|l| {
        line = l.to_string();
        if line.contains("-") {
            let t_range = line.split("-").collect::<Vec<&str>>();
            let start = t_range[0].parse::<u64>().unwrap();
            let end = t_range[1].parse::<u64>().unwrap();
            let range = start..end + 1;
            //println!("Range: {:?}", range);
            let range_set = range.collect::<HashSet<u64>>();
            //println!("Range set: {:?}", range_set);
            fresh_ids.extend(range_set);
        } else if line.len() > 0 {
            ingredient_ids.push(line.parse::<u64>().unwrap());
        }
    });
    (fresh_ids, ingredient_ids)
}

fn is_fresh(id: u64, fresh_ids: &Vec<RangeInclusive<u64>>) -> bool {
    fresh_ids.iter().any(|r| r.contains(&id))
}

fn total_fresh_oom(fresh_ids: &Vec<RangeInclusive<u64>>) -> u64 {
    let mut count = 0;
    // make a set of all the fresh ids
    // this method runs out of memory
    let mut fresh_ids_set = HashSet::new();
    for fresh_id in fresh_ids {
        fresh_ids_set.extend(fresh_id.clone());
    }
    count += fresh_ids_set.len() as u64;
    count
}

fn total_fresh(fresh_ids: &Vec<RangeInclusive<u64>>) -> u64 {
    if fresh_ids.is_empty() {
        return 0;
    }

    // Convert ranges into (start, end) tuples and sort by start
    let mut ranges: Vec<(u64, u64)> = fresh_ids
        .iter()
        .map(|r| (*r.start(), *r.end()))
        .collect();

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = vec![];
    
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            // Overlapping or touching → merge by extending end
            current.1 = current.1.max(end);
        } else {
            // Disjoint → push current and start a new one
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);

    // Sum the lengths of merged ranges
    merged.iter()
        .map(|(s, e)| *e - *s + 1)
        .sum()
}

fn main() {
    let (fresh_ids, ingredient_ids) = read_input();
    //println!("Fresh IDs: {:?}", fresh_ids);
    //println!("Ingredient IDs: {:?}", ingredient_ids);
    let mut count_fresh = 0;
    for ingredient_id in ingredient_ids {
        if is_fresh(ingredient_id, &fresh_ids) {
            //println!("Ingredient {} is fresh", ingredient_id);
            count_fresh += 1;
        } else {
            //println!("Ingredient {} is not fresh", ingredient_id);
        }
    }
    println!("Number of fresh ingredients: {}", count_fresh);
    println!("Total fresh ingredients: {}", total_fresh(&fresh_ids));
}
