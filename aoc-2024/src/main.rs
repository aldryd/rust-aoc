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

    // Keep track of the final answer for my input in case a refactor creates a bug
    assert_eq!(total, 173529487);
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

#[derive(PartialEq)]
enum CommandDay3 {
    NoCommand,
    Multiply,
    Do,
    Dont,
}

impl std::fmt::Display for CommandDay3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command: &str;
        match self {
            CommandDay3::Multiply => command = "Multiply",
            CommandDay3::Do => command = "Do",
            CommandDay3::Dont => command = "Don\'t",
            CommandDay3::NoCommand => command = "No Command",
        }
        write!(f, "{}", command)
    }
}

#[derive(PartialEq)]
enum FinderState {
    Scanning,
    ScanningForMul1,
    ScanningForMul2,
    ScanningForMul3,
    ScanningForDoOrDont,
    ScanningForDont,
}

fn day3_part2() {
    println!("--- Day 3: Mull It Over ---");
    println!("--- Part 2              ---\n");

    let day3_input = read_lines("resources/day3_input.txt");

    let _valid_characters: Vec<char> = ['m', 'u', 'l', 'd', 'o', 'n', '\'', 't', '(', ')', ',', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].to_vec();

    let mut total: i32 = 0;
    let mut should_execute_command: bool = true;

    for line in day3_input {
        // Get rid of all the known junk characters
        // @todo The command pattern matching currently has a bug where filtering out the invalid characters yields the wrong result
        // let filtered_input: String = line.chars().filter(|x| valid_characters.contains(x)).collect();
        let filtered_input: String = line;
        let mut index: usize = 0;

        while index < filtered_input.len() {
            let result = get_next_command(&filtered_input, index);
            index = result.1;
            let current_cmd = result.0;

            if current_cmd == CommandDay3::Dont {
                should_execute_command = false;
            } else if current_cmd == CommandDay3::Do {
                should_execute_command = true;
            }

            if should_execute_command && current_cmd == CommandDay3::Multiply {
                if let Some((multiplier1, multiplier2)) = get_mul_inputs(&filtered_input, index) {
                    total += multiplier1 * multiplier2;
                }
            }
        }
    }

    println!(">>>> Uncorrupted total: {}\n", total);

    // Keep track of the final answer for my input in case a refactor creates a bug
    assert_eq!(total, 99532691);
}

fn get_next_command(input: &str, start_index: usize) -> (CommandDay3, usize) {
    let mut state = FinderState::Scanning;

    for (ii, character) in input.chars().skip(start_index).enumerate() {
        match character {
            'm' => {
                state = FinderState::ScanningForMul1;
            },
            'u' => {
                if state != FinderState::ScanningForMul1 {
                    state = FinderState::Scanning;
                } else {
                    state = FinderState::ScanningForMul2
                }
            },
            'l' => {
                if state != FinderState::ScanningForMul2 {
                    state = FinderState::Scanning;
                } else {
                    state = FinderState::ScanningForMul3
                }
            },
            'd' => {
                state = FinderState::ScanningForDoOrDont;
            },
            'o' => {
                if state != FinderState::ScanningForDoOrDont {
                    state = FinderState::Scanning;
                }
            },
            'n' => {
                if state != FinderState::ScanningForDoOrDont {
                    state = FinderState::Scanning;
                } else {
                    state = FinderState::ScanningForDont;
                }
            },
            '\'' => {
                if state != FinderState::ScanningForDont {
                    state = FinderState::Scanning;
                }
            },
            't' => {
                if state != FinderState::ScanningForDont {
                    state = FinderState::Scanning;
                }
            },
            '(' => {
                match state {
                    FinderState::ScanningForMul3 => return (CommandDay3::Multiply, ii + start_index),
                    FinderState::ScanningForDoOrDont => return (CommandDay3::Do, ii + start_index),
                    FinderState::ScanningForDont => return (CommandDay3::Dont, ii + start_index),
                    _ => {
                        state = FinderState::Scanning;
                    },
                }
            }
            _ => {
                state = FinderState::Scanning;
            },
        }
    }

    return (CommandDay3::NoCommand, input.len())
}

fn get_mul_inputs(input: &str, start_index: usize) -> Option<(i32, i32)> {
    let mut multipler_stash: String = "".to_owned();

    let mut multiplier1: i32 = 0;

    for character in input.chars().skip(start_index) {
        if character == '(' {
            // Nothing to do here. Let the ( go on by
        } else if character.is_numeric() {
            multipler_stash.push(character);
        } else if character == ',' {
            if let Ok(value) = multipler_stash.parse::<i32>() {
                multiplier1 = value;
            } else {
                return None
            }

            multipler_stash.clear();
        } else if character == ')' {
            if let Ok(value) = multipler_stash.parse::<i32>() {
                return Some((multiplier1, value))
            } else {
                return None
            }
        } else {
            return None
        }
    }

    return None
}

struct Point {
    x: usize,
    y: usize,
}

fn day4_part1() {
    println!("--- Day 4: Ceres Search ---");
    println!("--- Part 1              ---\n");

    let xmas = "XMAS";

    let day4_input = read_lines("resources/day4_input.txt");

    let mut total = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    let mut x_points: Vec<Point> = vec![];
    for (row, line) in day4_input.iter().enumerate() {
        let grid_line: Vec<char> = line.chars().collect();

        // @todo Can this be made more efficient with a filter_map?
        for (column, letter) in line.chars().enumerate() {
            if letter == 'X' {
                x_points.push(Point { x: column, y: row });
            }
        }

        grid.push(grid_line);
    }

    for x_point in x_points {
        // For each X in the grid, identify if it forms XMAS in any viable direction

        let mut search_n: bool = true;
        let mut search_ne: bool = true;
        let mut search_e: bool = true;
        let mut search_se: bool = true;
        let mut search_s: bool = true;
        let mut search_sw: bool = true;
        let mut search_w: bool = true;
        let mut search_nw: bool = true;

        // Prevent searching in a direction that cannot possibly have an answer to avoid extra computation
        if x_point.x < xmas.len() - 1 {
            search_nw = false;
            search_w = false;
            search_sw = false;
        } else if x_point.x > grid[0].len() - xmas.len() {
            search_ne = false;
            search_e = false;
            search_se = false;
        }

        if x_point.y < xmas.len() - 1 {
            search_nw = false;
            search_n = false;
            search_ne = false;
        } else if x_point.y > grid.len() - xmas.len() {
            search_sw = false;
            search_s = false;
            search_se = false;
        }

        let mut word_list: Vec<String> = vec![];

        if search_n {
            word_list.push([grid[x_point.y][x_point.x], grid[x_point.y - 1][x_point.x], grid[x_point.y - 2][x_point.x], grid[x_point.y - 3][x_point.x]].iter().collect());
        }

        if search_ne {
            word_list.push([grid[x_point.y][x_point.x], grid[x_point.y - 1][x_point.x + 1], grid[x_point.y - 2][x_point.x + 2], grid[x_point.y - 3][x_point.x + 3]].iter().collect());
        }

        if search_e {
            word_list.push([grid[x_point.y][x_point.x], grid[x_point.y][x_point.x + 1], grid[x_point.y][x_point.x + 2], grid[x_point.y][x_point.x + 3]].iter().collect());
        }

        if search_se {
            word_list.push([grid[x_point.y][x_point.x], grid[x_point.y + 1][x_point.x + 1], grid[x_point.y + 2][x_point.x + 2], grid[x_point.y + 3][x_point.x + 3]].iter().collect());
        }

        if search_s {
            word_list.push([grid[x_point.y][x_point.x], grid[x_point.y + 1][x_point.x], grid[x_point.y + 2][x_point.x], grid[x_point.y + 3][x_point.x]].iter().collect());
        }

        if search_sw {
            word_list.push([grid[x_point.y][x_point.x], grid[x_point.y + 1][x_point.x - 1], grid[x_point.y + 2][x_point.x - 2], grid[x_point.y + 3][x_point.x - 3]].iter().collect());
        }

        if search_w {
            word_list.push([grid[x_point.y][x_point.x], grid[x_point.y][x_point.x - 1], grid[x_point.y][x_point.x - 2], grid[x_point.y][x_point.x - 3]].iter().collect());
        }

        if search_nw {
            word_list.push([grid[x_point.y][x_point.x], grid[x_point.y - 1][x_point.x - 1], grid[x_point.y - 2][x_point.x - 2], grid[x_point.y - 3][x_point.x - 3]].iter().collect());
        }

        for word in word_list {
            if word == xmas {
                total += 1;
            }
        }

        // println!(">>>> ({}, {}) | N:{} | NE:{} | E:{} | SE:{} | S:{} | SW:{} | W:{} | NW:{}",
        //          x_point.x, x_point.y,
        //          search_n, search_ne, search_e, search_se, search_s, search_sw, search_w, search_nw);
    }

    println!(">>>> Xmas total: {}\n", total);

    // Keep track of the final answer for my input in case a refactor creates a bug
    assert_eq!(total, 2507);
}

fn day4_part2() {
    println!("--- Day 4: Ceres Search ---");
    println!("--- Part 2              ---\n");

    let mas: &str = "MAS";
    let sam: &str = "SAM";

    let day4_input = read_lines("resources/day4_input.txt");

    let mut total = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    let mut a_points: Vec<Point> = vec![];
    for (row, line) in day4_input.iter().enumerate() {
        let grid_line: Vec<char> = line.chars().collect();

        // @todo Can this be made more efficient with a filter_map?
        for (column, letter) in line.chars().enumerate() {
            if letter == 'A' {
                a_points.push(Point { x: column, y: row });
            }
        }

        // println!("{}", line);

        grid.push(grid_line);
    }

    for a_point in a_points {
        // For each A in the grid, check if it is the center of an X-MAS

        // Ignore the outermost portion of the grid since any A characters there cannot be valid
        if a_point.x < 1 || a_point.x > grid[0].len() - 2 || a_point.y < 1 || a_point.y > grid.len() - 2 {
            continue;
        }

        // Extract the X of words with the A at the center
        let word1: String = [grid[a_point.y - 1][a_point.x - 1], grid[a_point.y][a_point.x], grid[a_point.y + 1][a_point.x + 1]].iter().collect();
        let word2: String = [grid[a_point.y + 1][a_point.x - 1], grid[a_point.y][a_point.x], grid[a_point.y - 1][a_point.x + 1]].iter().collect();

        if (word1 == mas || word1 == sam) && (word2 == mas || word2 == sam) {
            total += 1;
        }
    }

    println!(">>>> X-MAS total: {}\n", total);

    // Keep track of the final answer for my input in case a refactor creates a bug
    assert_eq!(total, 1969);
}

fn day5_part1() {
    println!("--- Day 5: Print Queue ---");
    println!("--- Part 1             ---\n");

    let day5_input = read_lines("resources/day5_input.txt");

    let mut page_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut page_updates: Vec<Vec<i32>> = vec![];

    let mut total = 0;
    for line in day5_input {
        if line.contains("|") {
            if let Some(raw_rules) = line.split_once("|") {
                // Build up a rule set for the sorting order of each value. The value in the HashMap represents the
                // page numbers that must come after the key.
                let key: i32 = raw_rules.0.parse::<i32>().unwrap();
                let value: i32 = raw_rules.1.parse::<i32>().unwrap();

                page_rules.entry(key).and_modify(|val_list| val_list.push(value)).or_insert(vec![value]);
            }
        } else if line.contains(",") {
            let update: Vec<i32> = line.split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect();
            page_updates.push(update);
        }
    }

    // For each page update in the list, run a sort. If the order changes, then we know the update is invalid.
    for update in page_updates.iter_mut() {
        // Keep the original so we can compare after it is sorted
        let original = update.clone();

        update.sort_by(|a, b| {
            if let Some(order_rules) = page_rules.get(a) {
                for rhs in order_rules {
                    if b == rhs {
                        return std::cmp::Ordering::Less
                    }
                }
            }

            return std::cmp::Ordering::Equal
        });

        if update == &original {
            total += update.get(update.len() / 2).unwrap();
        }
    }

    println!(">>>> Page total: {}\n", total);

    // Keep track of the final answer for my input in case a refactor creates a bug
    assert_eq!(total, 4578);
}

fn day5_part2() {
    println!("--- Day 5: Print Queue ---");
    println!("--- Part 2             ---\n");

    let day5_input = read_lines("resources/day5_input.txt");


    // Keep track of the final answer for my input in case a refactor creates a bug
    // assert_eq!(total, 1969);
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

    day5_part1();

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
        4 => {
            day4_part1();
            day4_part2();
        },
        5 => {
            day5_part1();
            day5_part2();
        },
        _ => {
            println!("Day {} not implemented yet", day);
        }
    }

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("\nElapsed Time: {:.4} ms", elapsed_ms);
}
