

use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn read_input() -> HashSet<isize> {
    let mut result : HashSet<isize> = HashSet::new();

    let f = File::open("day1_input.txt").expect("Failed to open input file");
    let reader = BufReader::new(f);
    for line_result in reader.lines() {
        let line = line_result.expect("Failed to read line");
        let unique = result.insert(line.parse().expect("Failed to parse input"));
        if !unique {
            panic!("Values in input are not unique. Think of something else");
        }
    }
    result
}

fn part_one(input: &HashSet<isize>) {

    for entry in input {

        let other_entry = 2020 - entry;
        if let Some(_) = input.get(&other_entry) {
            // found it!
            let mult = entry*other_entry;
            println!("Found {} * {} = {}", entry, other_entry, mult);
            // keep running in case it is not the only result. if the solution is unique, we should get only two identical pairs
        }
    }

}

fn part_two(input: &HashSet<isize>) {

    // this can probably be done faster that O(n^2), but eh... just use the same strategy as in part 1

    for entry1 in input {

        let remainder = 2020 - entry1;

        for entry2 in input {

            let entry3 = remainder - entry2;

            if let Some(_) = input.get(&entry3) {
                let mult = entry1*entry2*entry3;
                println!("Found {} * {} * {} = {}", entry1, entry2, entry3, mult);
            }

        }

    }

}

pub fn run() {

    let input = read_input();

    part_one(&input);
    part_two(&input);
}
