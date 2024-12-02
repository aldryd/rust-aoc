use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

/// Reads the input filename and returns a Vector of Strings for each line in the file
fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename);
    let reader = io::BufReader::new(file.unwrap());

    reader.lines().flatten().flat_map(|l| l.parse()).collect()
}

fn _day1_part1() {
    println!("--- Day 1: Historian Hysteria ---");
    println!("--- Part 1                    ---\n");

    let day1_input = read_lines("resources/day1_input.txt");

    let mut col1: Vec<i32> = vec![];
    let mut col2: Vec<i32> = vec![];
    for location_ids in day1_input {
        if let Some(columns) = location_ids.split_once(" ") {
            col1.push(columns.0.trim().parse::<i32>().unwrap());
            col2.push(columns.1.trim().parse::<i32>().unwrap());
        }
    }

    col1.sort();
    col2.sort();

    let mut total_distance = 0;
    for (ii, val) in col1.iter().enumerate() {
        let distance = val - col2[ii];

        total_distance += distance.abs();
    }

    println!(">>>> total_distance: {}\n", total_distance);

    // Keep track of the final answer for my input in case a refactor creates a bug
    assert_eq!(total_distance, 2031679);
}

fn _day1_part2() {
    println!("--- Day 1: Historian Hysteria ---");
    println!("--- Part 2                    ---\n");

    let day1_input = read_lines("resources/day1_input.txt");

    let mut col1: Vec<i32> = vec![];
    let mut col2_count_map: HashMap<i32, i32> = HashMap::new();
    for location_ids in day1_input {
        if let Some(columns) = location_ids.split_once(" ") {
            let col1_value = columns.0.trim().parse::<i32>().unwrap();
            let col2_value = columns.1.trim().parse::<i32>().unwrap();

            col1.push(col1_value);

            // Increment the count for the number of times the value has been seen in the second column
            let count = col2_count_map.entry(col2_value).or_insert(0);
            *count += 1;
        }
    }

    let mut total_similarity = 0;

    // Loop through all the values from column 1 to see how many times they show up in column 2
    for col1_value in col1 {
        if let Some(similarity) = col2_count_map.get(&col1_value) {
            total_similarity += col1_value * similarity;
        }
    }

    println!(">>>> total_similarity:{}\n", total_similarity);

    // Keep track of the final answer for my input in case a refactor creates a bug
    assert_eq!(total_similarity, 19678534);
}

fn main() {
    println!("         .     .  .      +     .      .          .");
    println!("     .       .      .     #       .           .");
    println!("        .      .         ###            .      .      .");
    println!("      .      .   \"#:. .:##\"##:. .:#\"  .      .");
    println!("          .      . \"####\"###\"####\"  .");
    println!("       .     \"#:.    .:#\"###\"#:.    .:#\"  .        .       .");
    println!("  .             \"#########\"#########\"        .        .");
    println!("        .    \"#:.  \"####\"###\"####\"  .:#\"   .       .");
    println!("     .     .  \"#######\"\"##\"##\"\"#######\"                  .");
    println!("                .\"##\"#####\"#####\"##\"           .      .");
    println!("    .   \"#:. ...  .:##\"###\"###\"##:.  ... .:#\"     .");
    println!("      .     \"#######\"##\"#####\"##\"#######\"      .     .");
    println!("    .    .     \"#####\"\"#######\"\"#####\"    .      .");
    println!("            .     \"      000      \"    .     .");
    println!("       .         .   .   000     .        .       .");
    println!(".. .. ..................O000O........................ ...... ...");
    println!("... .. .......... Advent of Code 2024 ................... ... ..\n");

    _day1_part1();
    _day1_part2();
}
