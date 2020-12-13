#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use std::cmp;

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

fn is_valid_hair_color(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    }

    RE.is_match(value)
}

fn is_valid_eye_color(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    }

    RE.is_match(value)
}

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

fn day6() {
    println!("--- Day 6: Custom Customs ---");
    println!("--- Part 1                ---");

    let input_path = "input_data/day6_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let customs_answers_unparsed: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    let mut answer_list: Vec<HashSet<char>> = vec![];
    let mut group_answer: HashSet<char> = HashSet::new();

    // Loop to read through the input and parse it into a vector of group_answer
    for line in &customs_answers_unparsed {
        let line_tokens: String = line.split_whitespace().collect();

        if line_tokens.is_empty() {
            // Once there is a blank line, store the current working answer group and then clear it to
            // start on the next one
            answer_list.push(group_answer.clone());
            group_answer.clear();
        } else {
            for letter in line_tokens.chars() {
                group_answer.insert(letter);
            }
        }
    }

    let mut answer_sum = 0;
    for group_answer in answer_list {
        answer_sum += group_answer.len();
    }

    println!("Sum of answers: {}", answer_sum);
}

fn day6_part2() {
    println!("--- Day 6: Custom Customs ---");
    println!("--- Part 2                ---");

    let input_path = "input_data/day6_input.txt";
    let all_group_answers: Vec<String> = std::fs::read_to_string(input_path).unwrap()
        .split("\n\n")
        .map(|group| group.to_string())
        .collect();

    let possible_answers = "abcdefghijklmnopqrstuvwxyz";

    let mut answer_sum: usize = 0;
    for group_answers in all_group_answers {

        // Iterate over all the possible answers and count how many times they show up in the
        // group's answers. If they show up the same number of times as the number of people in
        // the group, that counts toward the sum.
        let result = possible_answers.chars()
            .map(|character| (character, group_answers.matches(character).count()))
            .collect::<std::collections::HashMap<_, _>>();

        let number_of_people = group_answers.split("\n").count();

        answer_sum += result.values()
            .filter(|&value| *value == number_of_people)
            .count();
    }

    println!("Sum of answers: {}", answer_sum);
    assert_eq!(answer_sum, 3435);
}

fn search_for_bag(bag: &str, total_list: &HashMap<String, HashMap<String, usize>>, bag_node_list: &HashMap<String, usize>) -> bool {
    // If the bag node contains the bag search string, then return right away since it was found.
    // No need to search deeper.
    if bag_node_list.contains_key(bag) {
        return true
    }

    for bag_node in bag_node_list {
        let inner_bag_node_list = total_list.get(bag_node.0).unwrap();

        if inner_bag_node_list.is_empty() == false {
            if inner_bag_node_list.contains_key(bag) || search_for_bag(bag, &total_list, &inner_bag_node_list) {
                // Found the bag so return from here
                return true
            }
        }
    }

    false
}

fn count_bags_in_bag(total_list: &HashMap<String, HashMap<String, usize>>, bag_node_list: &HashMap<String, usize>) -> usize {
    let mut bag_sum: usize = 0;
    for bag_node in bag_node_list {
        bag_sum += bag_node.1;
        bag_sum += bag_node.1 * count_bags_in_bag(&total_list, total_list.get(bag_node.0).unwrap());
    }

    bag_sum
}

fn day7() {
    println!("--- Day 7: Handy Haversacks ---\n");

    type Bag = HashMap<String, HashMap<String, usize>>;

    let input_path = "input_data/day7_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let bag_rules_list: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    let mut parsed_bag_rule_list: Bag = HashMap::new();

    for bag_rule in bag_rules_list {
        let mut rule_iter = bag_rule.split(" bags contain ");
        let bag_type: String = rule_iter.next().unwrap().to_string();

        lazy_static! {
            static ref RE: Regex = Regex::new(" bag[s]?[,.][ ]?").expect("");
        }
        let rules: Vec<&str> = RE.split(rule_iter.next().unwrap()).into_iter().collect();

        let ruleset: HashMap<String, usize> = rules.iter()
            .filter(|&rule| rule.is_empty() == false && *rule != "no other")
            .map(|&rule| {
                let count_str = rule.split(" ").next().unwrap();
                let count = count_str.parse::<usize>().unwrap_or_default();

                // Use the count to construct a delimiter for splitting the bag type from the count
                let count_split_pattern = format!("{} ", count_str);
                let bag = rule.split(&count_split_pattern).nth(1).unwrap_or_default();

                (bag.to_string(), count)
            })
            .collect::<HashMap<_, _>>();

        parsed_bag_rule_list.insert(bag_type, ruleset);
    }

    let bag_count = parsed_bag_rule_list.values()
        .into_iter()
        .filter(|&bag_node| {
            search_for_bag("shiny gold", &parsed_bag_rule_list, bag_node) == true
        })
        .count();

    assert_eq!(bag_count, 211);
    println!("Bag count: {}", bag_count);

    let bag_sum = count_bags_in_bag(&parsed_bag_rule_list, &parsed_bag_rule_list.get("shiny gold").unwrap());
    println!("Bag sum: {}", bag_sum);
    assert_eq!(bag_sum, 12414);
}

fn test_bootcode(bootcode: &Vec<String>) -> (bool, i32) {
    let mut accumlator = 0;
    let mut instruction_pointer: i32 = 0;
    let mut instruction_tracker: HashSet<i32> = HashSet::new();

    while instruction_pointer < bootcode.len() as i32 {
        let instruction = bootcode.get(instruction_pointer as usize).unwrap();
        if instruction_tracker.insert(instruction_pointer) == false {
            break;
        }
        match instruction.split(" ").nth(0).unwrap() {
            "acc" => {
                accumlator += instruction.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                instruction_pointer += 1;
            },
            "jmp" => {
                instruction_pointer += instruction.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            }
            _ => instruction_pointer += 1,
        }
    }

    if instruction_pointer >= bootcode.len() as i32 {
        return (true, accumlator)
    } else {
        return (false, accumlator)
    }
}

fn day8() {
    println!("--- Day 8: Handheld Halting ---\n");
    let input_path = "input_data/day8_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let bootcode: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    let result = test_bootcode(&bootcode);

    assert_eq!(result.1, 1331);
    println!("Final accumulator value: {}", result.1);
}

fn day8_part2() {
    println!("--- Day 8: Handheld Halting ---");
    println!("--- Part 2                  ---\n");
    let input_path = "input_data/day8_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let bootcode: Vec<String> = reader.lines().map(|l| l.expect("Failed to read input line")).collect();

    for (ii, instruction) in bootcode.iter().enumerate() {
        // @TODO: Is there a better way to swap these instructions?
        let new_instruction: &str;
        match instruction.split(" ").nth(0).unwrap() {
            "nop" => new_instruction = "jmp",
            "jmp" => new_instruction = "nop",

            // For all other cases, skip this iteration since it doesn't change the behavior of
            // the bootcode
            _ => continue,
        }

        // Try swapping the nop/jmp command and running the code. If it completes successfully,
        // then that's the value we're looking for.
        let mut bootcode_to_test = bootcode.clone();
        bootcode_to_test[ii] = format!("{} {}", new_instruction.to_string(), instruction.split(" ").nth(1).unwrap());

        let result = test_bootcode(&bootcode_to_test);
        if result.0 {
            assert_eq!(result.1, 1121);
            println!(">>>> Accumulator result: {}", result.1);
        }
    }
}

fn find_two_values_for_sum(slice: &[u32], sum: u32) -> bool {
    let mut result = false;
    'outer: for value1 in slice {
        for value2 in slice {
            if value1 + value2 == sum {
                //println!("value check: {} | {}", expense1, expense2);
                result = true;
                break 'outer
            }
        }
    }

    result
}

fn day9() {
    println!("--- Day 9: Encoding Error ---\n");
    let input_path = "input_data/day9_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let xmas_series: Vec<u32> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    const PREAMBLE_LENGTH: usize = 25;

    let mut rule_breaker: u32 = 0;
    for (index, value) in xmas_series.iter().enumerate() {
        if index < PREAMBLE_LENGTH {
            continue
        }

        let slice = &xmas_series[index - PREAMBLE_LENGTH..index];

        if find_two_values_for_sum(slice, *value) == false {
            assert_eq!(*value, 25918798);
            println!("Rule breaker value: {}", *value);
            rule_breaker = *value;
            break;
        }
    }

    let mut start_index = 0;
    let mut end_index = 2;

    while end_index < xmas_series.len() {
        let slice: &[u32] = &xmas_series[start_index..end_index];
        let sum: u32 = slice.iter().fold(0, |sum, value| sum + value);
        //println!("{:?} | sum: {}", slice, sum);

        if sum > rule_breaker {
            start_index += 1;
            end_index = start_index + 2;
        } else if sum == rule_breaker {
            let min = slice.iter().min().unwrap();
            let max = slice.iter().max().unwrap();

            assert_eq!(min + max, 3340942);
            println!("min: {} | max: {} | sum: {}", min, max, min + max);
            break;
        } else {
            end_index += 1;
        }
    }
}

fn day10() {
    println!("--- Day 10: Adapter Array ---\n");

    let input_path = "input_data/day10_input.txt";

    let file = File::open(input_path);
    let reader = io::BufReader::new(file.unwrap());
    let mut adapter_list: Vec<u32> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    // 0 is not included in the input data
    adapter_list.push(0);
    adapter_list.sort();

    // The final value is always 3 more than the highest value in the vector
    adapter_list.push(adapter_list.last().unwrap() + 3);

    let differences: Vec<u32> = adapter_list.windows(2)
        .map(|window| window[1]- window[0])
        .collect();
    let count_of_ones = differences.iter()
        .filter(|&value| *value == 1)
        .count();
    let count_of_threes = differences.iter()
        .filter(|&value| *value == 3)
        .count();

    assert_eq!(count_of_ones * count_of_threes, 2574);
    println!("Joltage product: {}", count_of_ones * count_of_threes);
    
    let mut sequence_tracker: HashMap<u32, u32> = HashMap::new();
    let mut sequence_count: u32 = 0;
    for diff in differences.iter() {
        match *diff {
            1 => sequence_count += 1,
            _ => {
                // Update or insert the number of times this sequence has been seen
                (*sequence_tracker.entry(sequence_count).or_insert(0)) += 1;
                sequence_count = 0;
            }
        }
    }

    // The number of permutations is:
    // 1^(sequences of 0) * 1^(sequences of 2) * 2^(sequences of 3) * 4^(sequences of 4) * 7^(sequences of 5) * 13^(sequences of 6)
    //
    // The first 2 items can be ignored in the math since they always work out to 1; however, it
    // makes the fold operation a bit easier for grabbing the index to leave them in.
    let pattern: Vec<u32> = vec![1, 1, 2, 4, 7, 13];
    let permutations: u128 = pattern.iter()
        .enumerate()
        .fold(1, |product, (index, value)| {
            product * (*value).pow(*sequence_tracker.get(&(index as u32)).unwrap_or(&0)) as u128
        });

    assert_eq!(permutations, 2_644_613_988_352);
    println!("Number of permutations: {}", permutations);
}

fn count_occupied_seats(row_index: i32, col_index: i32, nearby: bool, seat_list: &Vec<Vec<char>>) -> u32 {

    const DIRECTIONS: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    let mut occupied_count = 0;

    for direction in DIRECTIONS.iter() {
        let mut working_row = row_index;
        let mut working_col = col_index;
        loop {
            let seat_address = (working_row + direction.0, working_col + direction.1);

            if seat_address.0 < 0 || seat_address.1 < 0 ||
                seat_address.0 >= seat_list.len() as i32 || seat_address.1 >= seat_list[0].len() as i32 {
                // Hit a "wall" while looking for occupied seats in this direction
                break;
            }

            match seat_list[seat_address.0 as usize][seat_address.1 as usize] {
                'L' => {
                    break;
                },
                '#' => {
                    occupied_count += 1;
                    break;
                }
                _ => {
                    if nearby {
                        // Count only immediate neighbors so break out of the loop even for empty
                        // floor tiles
                        break;
                    } else {
                        // For Part 2, the rules change to continue looking in the same direction
                        // when seeing an empty floor tile
                        working_row += direction.0;
                        working_col += direction.1;
                    }
                },
            }
        }
    }

    occupied_count as u32
}

fn assign_seat(row_index: usize, col_index: usize, assignment: char, seat_list: &mut Vec<Vec<char>>) {
    seat_list[row_index][col_index] = assignment;
}

#[allow(dead_code)]
fn print_seating_chart(seat_list: &Vec<Vec<char>>) {
    print!("\n");
    for row in seat_list {
        row.iter().for_each(|seat| print!("{}", seat));
        print!("\n");
    }
    print!("\n");
}

fn iterate_seating(seat_list: &Vec<Vec<char>>, crowding: u32, nearby: bool) -> Option<Vec<Vec<char>>> {

    let mut working_seat_list: Vec<Vec<char>> = seat_list.clone();
    let mut seat_list_changed = false;

    //print_seating_chart(seat_list);

    for (row_index, row) in seat_list.iter().enumerate() {
        for (col_index, seat) in row.iter().enumerate() {
            match seat {
                'L' => {
                    if count_occupied_seats(row_index as i32, col_index as i32, nearby, &seat_list) == 0 {
                        assign_seat(row_index, col_index, '#', &mut working_seat_list);
                        seat_list_changed = true;
                    }
                },
                '#' => {
                    if count_occupied_seats(row_index as i32, col_index as i32, nearby, &seat_list) >= crowding {
                        assign_seat(row_index, col_index, 'L', &mut working_seat_list);
                        seat_list_changed = true;
                    }
                },
                _ => {
                    // Nothing to do here
                }
            }
        }
    }

    return if seat_list_changed {
        Some(working_seat_list)
    } else {
        None
    }
}

fn day11() {
    println!("--- Day 11: Seating System ---\n");
    let input_path = "input_data/day11_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let input_seat_list: Vec<String> = reader.lines()
        .map(|l| l.expect("Failed to read input line"))
        .collect();

    let mut seat_list: Vec<Vec<char>> = vec![];
    for seat_line in input_seat_list {
        seat_list.push(seat_line.chars().collect());
    }

    while let Some(seating_result) = iterate_seating(&seat_list, 4, true) {
        seat_list = seating_result;
    }

    //print_seating_chart(&seat_list);

    let total_occupied_seats = seat_list.iter()
        .fold(0, |sum, row| sum + row.iter()
            .filter(|&seat| *seat == '#')
            .count());
    assert_eq!(total_occupied_seats, 2126);
    println!("Total occupied seat count: {}", total_occupied_seats);
}

fn day11_part2() {
    println!("--- Day 11: Seating System ---");
    println!("--- Part 2                 ---\n");
    let input_path = "input_data/day11_input.txt";
    let reader = io::BufReader::new(File::open(input_path).unwrap());
    let input_seat_list: Vec<String> = reader.lines()
        .map(|l| l.expect("Failed to read input line"))
        .collect();

    let mut seat_list: Vec<Vec<char>> = vec![];
    for seat_line in input_seat_list {
        seat_list.push(seat_line.chars().collect());
    }

    while let Some(seating_result) = iterate_seating(&seat_list, 5, false) {
        seat_list = seating_result;

        //print_seating_chart(&seat_list);
    }

    let total_occupied_seats = seat_list.iter()
        .fold(0, |sum, row| sum + row.iter()
            .filter(|&seat| *seat == '#')
            .count());

    assert_eq!(total_occupied_seats, 1914);
    println!("Total occupied seat count: {}", total_occupied_seats);
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

    // Setup a silly match statement to stop having to put #[allow(dead_code)] everywhere
    const DAY_TO_RUN: u32 = 11;
    match DAY_TO_RUN {
        1 => day1(),
        2 => {
            day2_part1();
            day2_part2();
        },
        3 => {
            day3();
            day3_part2();
        },
        4 => day4(),
        5 => day5(),
        6 => {
            day6();
            day6_part2();
        },
        7 => day7(),
        8 => {
            day8();
            day8_part2();
        },
        9 => day9(),
        10 => day10(),
        11 => {
            day11();
            day11_part2();
        }
        _ => {},
    }
}
