#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;
use std::cmp;

#[allow(dead_code)]
fn day1() {

    println!("--- Day 1: Report Repair ---");
    println!("--- Part 1               ---\n");

    let input_path = "input_data/day1_input.txt";

    let file = File::open(input_path);
    let reader = io::BufReader::new(file.unwrap());
    let day1_input_vec: Vec<i32> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    // Borrow the day1_input_vec (with &) so as to not consume it and allow us to iterate on it on
    // the inner loop as well
    'outer: for expense1 in &day1_input_vec {
        for expense2 in &day1_input_vec {
            if expense1 + expense2 == 2020 {
                println!("Expense report values: {} | {}", expense1, expense2);
                println!("Product: {}", expense1 * expense2);
                break 'outer;
            }
        }
    }

    println!("\n--- Part 2               ---\n");

    // Borrow the day1_input_vec (with &) so as to not consume it and allow us to iterate on it on
    // the inner loop as well
    'part2_outer: for expense1 in &day1_input_vec {
        for expense2 in &day1_input_vec {
            let expense3_expected = 2020 - (expense1 + expense2);

            if day1_input_vec.contains(&expense3_expected) {
                println!("Expense report values: {} | {} | {}", expense1, expense2, expense3_expected);
                println!("Product: {}", expense1 * expense2 * expense3_expected);
                break 'part2_outer;
            }
        }
    }

    println!("\n----------------------------");
    println!("----------------------------");
}

#[allow(dead_code)]
fn day2_part1() {
    println!("--- Day 2: Password Philosophy ---");
    println!("--- Part 1                     ---\n");

    let input_path = "input_data/day2_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let day2_input_vec: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    struct PasswordRule {
        minimum: i32,
        maximum: i32,
        value: char,
    }

    let mut valid_count = 0;
    for line in day2_input_vec {
        // Assumed input values! No error checking
        // Input example: 1-3 a: abcdef
        let password_tokens: Vec<&str> = line.split(":").collect();
        let rule_tokens: Vec<&str> = password_tokens[0].split(" ").collect();
        let min_max_tokens: Vec<&str> = rule_tokens[0].split("-").collect();

        let rule: PasswordRule = PasswordRule {
            minimum: min_max_tokens[0].parse().unwrap(),
            maximum: min_max_tokens[1].parse().unwrap(),

            // Assume rule_token[1] contains the character to check and convert it to a char
            value: rule_tokens[1].chars().next().unwrap(),
        };

        // Use filter to count the frequency of the given character
        let frequency: i32 = password_tokens[1].chars()
            .filter(|letter| *letter == rule.value).count() as i32;

        if frequency >= rule.minimum && frequency <= rule.maximum {
            valid_count += 1;
        }
    }

    println!("Number of valid passwords: {}", valid_count);
}

#[allow(dead_code)]
fn day2_part2() {
    println!("--- Part 2                     ---\n");

    let input_path = "input_data/day2_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let day2_input_vec: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    struct PasswordRule {
        position1: usize,
        position2: usize,
        value: char,
    }

    let mut valid_count = 0;
    for line in day2_input_vec {
        // Assumed input values! No error checking
        // Input example: 1-3 a: abcdef
        let password_tokens: Vec<&str> = line.split(":").collect();
        let rule_tokens: Vec<&str> = password_tokens[0].split(" ").collect();
        let position_tokens: Vec<&str> = rule_tokens[0].split("-").collect();

        let rule: PasswordRule = PasswordRule {
            position1: position_tokens[0].parse::<usize>().unwrap(),
            position2: position_tokens[1].parse::<usize>().unwrap(),

            // Assume rule_token[1] contains the character to check and convert it to a char
            value: rule_tokens[1].chars().next().unwrap(),
        };

        // Do an XOR check to count how many passwords are valid
        let password = password_tokens[1];
        if (password.chars().nth(rule.position1).unwrap() == rule.value) !=
            (password.chars().nth(rule.position2).unwrap() == rule.value) {
            valid_count += 1;
        }
    }

    println!("Number of valid passwords: {}", valid_count);
}

#[allow(dead_code)]
fn day3() {
    println!("--- Day 3: Toboggan Trajectory ---");
    println!("--- Part 1                     ---\n");

    let input_path = "input_data/day3_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let landscape: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    let tree = '#';
    let _open = '.';

    let right_movement = 3;
    let down_movement = 1;
    let mut current_x = 0;
    let mut tree_total = 0;

    // Skip the starting N lines as they are not used
    for land_line in landscape.iter().skip(down_movement).step_by(down_movement) {
        current_x += right_movement;

        if current_x >= land_line.chars().count() {
            current_x -= land_line.chars().count();
        }

        if land_line.chars().nth(current_x).unwrap() == tree {
            tree_total += 1;
        }
    }

    println!("Total trees hit: {}", tree_total);
}

#[allow(dead_code)]
fn day3_part2() {
    println!("--- Day 3: Toboggan Trajectory ---");
    println!("--- Part 2                     ---\n");

    let input_path = "input_data/day3_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let landscape: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    let tree = '#';
    let _open = '.';

    let movements: Vec<(usize, usize)> = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut tree_product:i64 = 1;
    for move_instruction in movements {
        let right_movement = move_instruction.0;
        let down_movement = move_instruction.1;

        let mut current_x = 0;
        let mut tree_total = 0;

        // Skip the starting N lines as they are not used
        for land_line in landscape.iter().skip(down_movement).step_by(down_movement) {
            current_x += right_movement;

            if current_x >= land_line.chars().count() {
                current_x -= land_line.chars().count();
            }

            if land_line.chars().nth(current_x).unwrap() == tree {
                tree_total += 1;
            }
        }

        tree_product *= tree_total;
    }

    println!("Tree product: {}", tree_product);
}

#[allow(dead_code)]
fn day4_part1_is_valid_passport(passport: Vec<&str>) -> bool {
    let number_of_required_fields: usize = 7;

    let mut tags: HashMap<&str, bool>= HashMap::new();
    for pair in passport {
        let key_value_vec: Vec<&str> = pair.split(":").collect();
        tags.entry(key_value_vec[0]).or_insert(true);
    }

    // Valid passports may or may not have the "cid" field, so we don't care if it exists
    // Since all the other fields are required, all 7 of them must exist
    return number_of_required_fields == tags.iter().filter(|&(key, value)| *key != "cid" && *value == true).count()
}

#[allow(dead_code)]
fn is_valid_height(value: &str) -> bool {
    // unit check
    if value.len() >= 2 {
        if value.ends_with("in") {
            let height: i32 = value.trim_end_matches("in").parse().unwrap();
            height >= 59 && height <= 76
        } else if value.ends_with("cm") {
            let height: i32 = value.trim_end_matches("cm").parse().unwrap();
            height >= 150 && height <= 193
        } else {
            false
        }
    } else {
        false
    }
}

#[allow(dead_code)]
fn is_valid_hair_color(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    }

    RE.is_match(value)
}

#[allow(dead_code)]
fn is_valid_eye_color(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    }

    RE.is_match(value)
}

#[allow(dead_code)]
fn day4_part2_is_field_valid(field_name: &str, value: &str) ->bool {
    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)
    return match field_name {
        "byr" => value.parse::<i32>().unwrap() >= 1920 && value.parse::<i32>().unwrap() <= 2002,
        "iyr" => value.parse::<i32>().unwrap() >= 2010 && value.parse::<i32>().unwrap() <= 2020,
        "eyr" => value.parse::<i32>().unwrap() >= 2020 && value.parse::<i32>().unwrap() <= 2030,
        "hgt" => is_valid_height(value),
        "hcl" => is_valid_hair_color(value),
        "ecl" => is_valid_eye_color(value),
        "pid" => value.chars().filter(|the_char| the_char.is_digit(10)).count() == 9,

        // The cid field is optional so ignore it
        //"cid" => true,

        // No other values are valid
        _ => false,
    }
}

#[allow(dead_code)]
fn day4_part2_is_valid_passport(passport: Vec<&str>) -> bool {
    let number_of_required_fields: usize = 7;

    let mut tags: HashMap<&str, bool>= HashMap::new();
    for pair in passport {
        let key_value_vec: Vec<&str> = pair.split(":").collect();
        let key = key_value_vec[0];
        let value = key_value_vec[1];

        if day4_part2_is_field_valid(key, value) {
            tags.entry(key).or_insert(true);
        }
    }

    // Valid passports may or may not have the "cid" field, so we don't care if it exists
    // Since all the other fields are required, all 7 of them must exist
    return number_of_required_fields == tags.iter().filter(|&(key, value)| *key != "cid" && *value == true).count()
}

#[allow(dead_code)]
fn day4() {
    println!("--- Day 4: Passport Processing ---");
    println!("--- Part 1                     ---\n");

    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)

    let input_path = "input_data/day4_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let passports_unparsed: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    let mut passport_list: Vec<Vec<&str>> = vec![];
    let mut passport: Vec<&str> = vec![];

    // Loop to read through the input and parse it into a vector of passports
    for line in &passports_unparsed {
        let line_tokens: Vec<&str> = line.split_whitespace().collect();

        if line_tokens.len() == 0 {
            // Once there is a blank line, store the current working passport and then clear it to
            // start on the next one
            passport_list.push(passport.clone());
            passport.clear();
        } else {
            passport.extend(line_tokens);
        }
    }

    println!("Number of passports: {}", passport_list.len());

    let passport_list_part2 = passport_list.clone();

    let mut valid_passport_count_part1 = 0;
    for passport in passport_list {
        if day4_part1_is_valid_passport(passport) {
            valid_passport_count_part1 += 1;
        }
    }

    let mut valid_passport_count_part2 = 0;
    for passport in passport_list_part2 {
        if day4_part2_is_valid_passport(passport) {
            valid_passport_count_part2 += 1;
        }
    }

    println!("Number of valid passports (part 1): {}", valid_passport_count_part1);
    println!("Number of valid passports (part 2): {}", valid_passport_count_part2);

    // Include asserts for the right answers in case I decide to tweak the solutions later
    assert_eq!(valid_passport_count_part1, 226);
    assert_eq!(valid_passport_count_part2, 160);
}

fn day5() {
    println!("--- Day 5: Binary Boarding ---\n");

    let input_path = "input_data/day5_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let all_seats: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    let mut min_seat_id: i32 = i32::MAX;
    let mut max_seat_id: i32 = 0;

    let mut assigned_seat_list: Vec<i32> = vec![];

    // Each seat ID in the list can be represented as a 10 bit value where:
    // F and L are 0
    // B and R are 1
    // For example, the seat FBFBFFBRRR is 0101001111 which is 335

    for seat in all_seats {
        let mut seat_bits: u16 = 0;
        for designation in seat.chars() {
            seat_bits <<= 1;
            if designation == 'B' || designation == 'R' {
                seat_bits |= 1;
            }
        }

        assigned_seat_list.push(seat_bits as i32);

        min_seat_id = cmp::min(seat_bits as i32, min_seat_id);
        max_seat_id = cmp::max(seat_bits as i32, max_seat_id);
    }

    // Include asserts for the right answers in case I decide to tweak the solutions later
    assert_eq!(max_seat_id, 915);
    println!("Highest seat ID: {}", max_seat_id);

    // Binary search through the assigned seats to find the missing one
    assigned_seat_list.sort();
    for seat_id_check in min_seat_id..max_seat_id {
        let result = assigned_seat_list.binary_search(&seat_id_check);
        if result.is_err() {
            // This is my seat!
            println!("My seat ID is: {}", seat_id_check);

            // Include asserts for the right answers in case I decide to tweak the solutions later
            assert_eq!(seat_id_check, 699);
            break;
        }
    }
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
    println!("... .. .......... Advent of Code 2020 ................... ... ..");

    // day1();
    // day2_part1();
    // day2_part2();
    //day3();
    //day3_part2();
    //day4();
    day5();
}
