use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn _day1_part1() {
    println!("--- Day 1: Trebuchet ---");
    println!("--- Part 1           ---\n");

    let input_path = "input_data/day1_input.txt";

    let file = File::open(input_path);
    let reader = io::BufReader::new(file.unwrap());
    let day1_input: Vec<String> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    let mut calibration_total: i32 = 0;
    for artsy_calibration in day1_input {
        let mut first_digit = ' ';
        let mut last_digit = ' ';
        for calibration_char in artsy_calibration.chars() {
            if calibration_char.is_numeric() {
                if false == first_digit.is_numeric() {
                    // If the first_digit has not been assigned yet, this must be the first digit
                    first_digit = calibration_char;
                }

                // Always assign the last_digit value from calibration_char because we want to overwrite with
                // what will become the last numeric value
                last_digit = calibration_char;
            }
        }

        let calibration = format!("{first_digit}{last_digit}");
        calibration_total += calibration.parse::<i32>().unwrap();
    }

    println!(">>>> calibration sum:{calibration_total}");
}

/// Attempts to convert the given string into a numeric character. Returns a tuple with a boolean indicating success
/// or failure and the numeric character.
fn _convert_word_to_numeric(word: &str) -> (bool, char) {
    if word.ends_with("one") {
        (true, '1' as char)
    } else if word.ends_with("two") {
        (true, '2' as char)
    } else if word.ends_with("three") {
        (true, '3' as char)
    } else if word.ends_with("four") {
        (true, '4' as char)
    } else if word.ends_with("five") {
        (true, '5' as char)
    } else if word.ends_with("six") {
        (true, '6' as char)
    } else if word.ends_with("seven") {
        (true, '7' as char)
    } else if word.ends_with("eight") {
        (true, '8' as char)
    } else if word.ends_with("nine") {
        (true, '9' as char)
    } else {
        (false, '0' as char)
    }
}

fn _day1_part2() {
    println!("--- Day 1: Trebuchet ---");
    println!("--- Part 2           ---\n");

    let input_path = "input_data/day1_input.txt";

    let file = File::open(input_path);
    let reader = io::BufReader::new(file.unwrap());
    let day1_input: Vec<String> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    let mut calibration_total: i32 = 0;
    for artsy_calibration in day1_input {
        let mut first_digit = ' ';
        let mut last_digit = ' ';
        let mut digit_buffer: String = Default::default();
        for calibration_char in artsy_calibration.chars() {
            if calibration_char.is_numeric() {
                if false == first_digit.is_numeric() {
                    // If the first_digit has not been assigned yet, this must be the first digit
                    first_digit = calibration_char;
                }

                // Always assign the last_digit value from calibration_char because we want to overwrite with
                // what will become the last numeric value
                last_digit = calibration_char;

                digit_buffer.clear();
            } else {
                digit_buffer.push(calibration_char);

                let converted = _convert_word_to_numeric(&digit_buffer);
                // .0 contains whether or not the conversion was successful
                if converted.0 {
                    if false == first_digit.is_numeric() {
                        first_digit = converted.1
                    }

                    // Always assign the last_digit value from calibration_char because we want to overwrite with
                    // what will become the last numeric value
                    last_digit = converted.1;

                    // Unlike with 0-9 characters, don't clear the digit_buffer here because a value such as "sevenine" should yield 79
                }
            }
        }

        let calibration = format!("{first_digit}{last_digit}");
        calibration_total += calibration.parse::<i32>().unwrap();
    }

    println!(">>>> calibration sum:{calibration_total}");
}

fn _day2_part1() {
    println!("--- Day 2: Cube Conundrum ---");
    println!("--- Part 1                ---\n");

    let input_path = "input_data/day2_input.txt";

    let file = File::open(input_path);
    let reader = io::BufReader::new(file.unwrap());
    let day2_input: Vec<String> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    let max_red_cubes: i32 = 12;
    let max_green_cubes: i32 = 13;
    let max_blue_cubes: i32 = 14;

    let mut count: i32 = 0;
    let mut game_index: i32 = 0;

    for game in day2_input {
        let mut impossible_hand: bool = false;

        game_index += 1;
        
        let game_hands = game.split_once(':').unwrap().1;
        for handful in game_hands.split(';') {
            let mut red_total = 0;
            let mut green_total = 0;
            let mut blue_total = 0;
            for color_group in handful.split(',') {
                let _tmp = color_group.trim_start().split_once(' ').unwrap().0;
                let number_of_cubes: i32 = color_group.trim_start().split_once(' ').unwrap().0.parse::<i32>().unwrap();
                if color_group.ends_with("red") {
                    red_total += number_of_cubes;
                } else if color_group.ends_with("green") {
                    green_total += number_of_cubes;
                } else if color_group.ends_with("blue") {
                    blue_total += number_of_cubes;
                }
            }

            if red_total > max_red_cubes
                || green_total > max_green_cubes
                || blue_total > max_blue_cubes {

                impossible_hand = true;
                break;
            }
        }

        if false == impossible_hand {
            count += game_index;
        }
    }

    println!(">>>> sum: {count}");
}

fn _day2_part2() {
    println!("--- Day 2: Cube Conundrum ---");
    println!("--- Part 2                ---\n");

    let input_path = "input_data/day2_input.txt";

    let file = File::open(input_path);
    let reader = io::BufReader::new(file.unwrap());
    let day2_input: Vec<String> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    let mut count: i32 = 0;

    for game in day2_input {
        let mut local_red_max: i32 = 0;
        let mut local_green_max: i32 = 0;
        let mut local_blue_max: i32 = 0;
        
        let game_hands = game.split_once(':').unwrap().1;
        for handful in game_hands.split(';') {
            let mut red_total = 0;
            let mut green_total = 0;
            let mut blue_total = 0;

            for color_group in handful.split(',') {
                let _tmp = color_group.trim_start().split_once(' ').unwrap().0;
                let number_of_cubes: i32 = color_group.trim_start().split_once(' ').unwrap().0.parse::<i32>().unwrap();
                if color_group.ends_with("red") {
                    red_total += number_of_cubes;
                } else if color_group.ends_with("green") {
                    green_total += number_of_cubes;
                } else if color_group.ends_with("blue") {
                    blue_total += number_of_cubes;
                }
            }

            local_red_max = cmp::max(local_red_max, red_total);
            local_green_max = cmp::max(local_green_max, green_total);
            local_blue_max = cmp::max(local_blue_max, blue_total);
        }

        count += local_red_max * local_green_max * local_blue_max;
    }

    println!(">>>> sum: {count}");
}

fn _day3_part1() {
    println!("--- Day 3: Gear Ratios ---");
    println!("--- Part 1             ---\n");

    let input_path = "input_data/day3_input.txt";

    let file = File::open(input_path);
    let reader = io::BufReader::new(file.unwrap());
    let day3_input: Vec<String> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    let mut schematic: Vec<Vec<char>> = vec![];

    for line in day3_input {
        let schematic_line: Vec<char> = line.chars().collect::<Vec<char>>();
        schematic.push(schematic_line);
    }

    let mut line_index = 0;

    for line in schematic {
        line_index += 1;

        let mut col_index = 0;
        for digit in line {
            col_index += 1;
            print!("{digit}");
        }

        print!("\n");
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
    println!("... .. .......... Advent of Code 2023 ................... ... ..");

    _day3_part1();
}
