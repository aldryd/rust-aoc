use std::fs::File;
use std::io::{self, BufRead};

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

    day1();
    day2_part1();
    day2_part2();
}