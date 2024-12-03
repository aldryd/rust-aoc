use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

/// Reads the input filename and returns a Vector of Strings for each line in the file
fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename);
    let reader = io::BufReader::new(file.unwrap());

    reader.lines().flatten().flat_map(|l| l.parse()).collect()
}

fn day1_part1() {
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

fn day1_part2() {
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

fn day2_part1() {
    println!("--- Day 2: Red-Nosed Reports ---");
    println!("--- Part 1                   ---\n");

    let day2_input = read_lines("resources/day2_input.txt");

    const MAX_LEVEL_CHANGE: i32 = 3;
    let mut total_safe_reports = 0;

    for report_raw in day2_input {
        let report: Vec<i32> = report_raw.split(" ").map(|x| x.trim().parse::<i32>().unwrap()).collect();

        if is_report_safe(report.borrow(), MAX_LEVEL_CHANGE, None) {
            total_safe_reports += 1;
        }
    }

    println!(">>>> Number of safe reports: {}", total_safe_reports);

    // Keep track of the final answer for my input in case a refactor creates a bug
    assert_eq!(total_safe_reports, 432);
}

fn day2_part2() {
    println!("--- Day 2: Red-Nosed Reports ---");
    println!("--- Part 2                   ---\n");

    let day2_input = read_lines("resources/day2_input.txt");

    const MAX_LEVEL_CHANGE: i32 = 3;
    let mut total_safe_reports = 0;

    for report_raw in day2_input {
        let report: Vec<i32> = report_raw.split(" ").map(|x| x.trim().parse::<i32>().unwrap()).collect();

        if is_report_safe(report.borrow(), MAX_LEVEL_CHANGE, None) {
            total_safe_reports += 1;
        } else {
            // If the report is unsafe, iterate through the report while skipping 1 element at a time
            for index in 0..report.len() {
                if is_report_safe(report.borrow(), MAX_LEVEL_CHANGE, Some(index)) {
                    // Any report that can be considered safe by skipping a single step should be counted
                    total_safe_reports += 1;
                    break;
                }
            }
        }
    }

    println!(">>>> Number of safe reports: {}", total_safe_reports);

    // Keep track of the final answer for my input in case a refactor creates a bug
    assert_eq!(total_safe_reports, 488);
}

fn is_report_safe(report: &Vec<i32>, max_level_change: i32, skip_index: Option<usize>) -> bool {
    let mut is_increasing: Option<bool> = None;
    let mut is_safe: bool = true;

    // Create a local copy that can be modified. This allows us to use the windows iterator more easily.
    // It would probably be more efficient to write the for loop such that it skips the index while
    // checking each item instead of creating a clone.
    let mut local_report: Vec<i32> = report.clone();

    if let Some(index) = skip_index {
        local_report.remove(index);
    }

    for level_window in local_report.windows(2) {
        let difference = level_window.get(1).unwrap() - level_window.get(0).unwrap();

        if difference == 0 {
            // If there is no change in the levels, we already know this is an unsafe report since they
            // must all increase or decrease
            is_safe = false;
            break;
        }

        // If is_increasing has not yet been set, use the first time through as the value it should be for
        // the entire set of numbers
        if let Some(local_is_increasing) = is_increasing {
            if (difference > 0) != local_is_increasing {
                // If the current increase/decrease is different than the saved value, this report is unsafe
                is_safe = false;
                break;
            }
        }

        is_increasing = Some(difference > 0);

        if difference.abs() > max_level_change {
            is_safe = false;
            break;
        }
    }
    is_safe
}

fn day3_part1() {
    println!("--- Day 3: Mull It Over ---");
    println!("--- Part 1              ---\n");

    let day3_input = read_lines("resources/day3_input.txt");

    let valid_characters: Vec<char> = ['m', 'u', 'l', '(', ')', ',', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].to_vec();

    let mut total = 0;
    for line in day3_input {
        // Get rid of all the known junk characters
        let filtered_input: String = line.chars().filter(|x| valid_characters.contains(x)).collect();

        // println!(">>>> {:?}", filtered_input);

        if let Some(value) = get_multiple(&filtered_input) {
            total += value;
        }
    }

    println!(">>>> Uncorrupted total: {}\n", total);
}

fn get_multiple(input: &str) -> Option<i32> {
    if let Some(mul_tokens) = input.split_once("mul(") {
        let mut multipler_stash: String = "".to_owned();

        let mut multiplier1: i32 = 0;
        let mut multiplier2: i32 = 0;

        let mut next: bool = false;
        let mut total: i32 = 0;

        for (index, character) in mul_tokens.1.chars().enumerate() {
            if character.is_numeric() {
                multipler_stash.push(character);
            } else if character == ',' {
                if let Ok(value) = multipler_stash.parse::<i32>() {
                    multiplier1 = value;
                }

                multipler_stash.clear();
            } else if character == ')' {
                if let Ok(value) = multipler_stash.parse::<i32>() {
                    multiplier2 = value;
                }

                multipler_stash.clear();

                // println!(">>>> {} * {}", multiplier1, multiplier2);
                total = multiplier1 * multiplier2;

                next = true;
            } else {
                multiplier1 = 0;
                multiplier2 = 0;
                multipler_stash.clear();

                next = true;
            }

            if next {
                let substring: String = mul_tokens.1.chars().skip(index).collect();
                if let Some(value) = get_multiple(substring.borrow()) {
                    total += value;
                }

                break;
            }
        }

        Some(total)
    } else {
        None
    }
}

fn day3_part2() {
    println!("--- Day 3: Mull It Over ---");
    println!("--- Part 2              ---\n");

    let day3_input = read_lines("resources/day3_input.txt");
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

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: aoc-2024 DAY");
    }

    let day: u8 = args.get(1).unwrap().parse::<u8>().unwrap();

    let time = Instant::now();

    match day {
        1 => {
            day1_part1();
            day1_part2();
        },
        2 => {
            day2_part1();
            day2_part2();
        },
        3 => {
            day3_part1();
            day3_part2();
        },
        _ => {}
    }

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("\nElapsed Time: {:.4} ms", elapsed_ms);
}
