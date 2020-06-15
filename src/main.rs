use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

mod intcode_computer;

fn module_fuel(mass: i32) -> i32 {
    (mass / 3).saturating_sub(2)
}

fn module_fuel_all(mass: i32) -> i32 {
    return if mass <= 0 {
        0
    } else {
        let mut fuel = module_fuel(mass);
        let fuel_for_fuel = module_fuel_all(fuel);

        if fuel_for_fuel > 0 {
            fuel += module_fuel_all(fuel);
        }
        fuel
    }
}

#[allow(dead_code)]
fn exec_day1() {
    println!("##### Day 1: The Tyranny of the Rocket Equation");

    println!("##### Part 1");
    let day1_input_file = File::open("input/day1_input.txt");
    let reader = BufReader::new(day1_input_file.unwrap());
    let day1_input_vec: Vec<i32> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    let total: i32 = day1_input_vec.iter().cloned().map(module_fuel).sum();

    let mut total_mass = 0;
    for mass in day1_input_vec {
        total_mass += module_fuel(mass);
    }

    println!(">>>>> Total mass:{}", total_mass);
    println!(">>>>> Total from map:{}", total);

    println!("##### Part 2");
    let day1_input_file = File::open("input/day1_input.txt");
    let reader = BufReader::new(day1_input_file.unwrap());
    let day1_input_vec: Vec<i32> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    let mut total_mass_with_fuel: i32 = 0;
    for mass in day1_input_vec {
        total_mass_with_fuel += module_fuel_all(mass);
    }

    println!(">>>>> Total mass with fuel:{}", total_mass_with_fuel);
}

fn read_file_to_vector(input_file_name: &str) -> Vec<i32> {
    let mut final_vector = vec![];
    let input_file = File::open(input_file_name);
    let reader = BufReader::new(input_file.unwrap());

    for line in reader.lines() {
        for value in line.unwrap().split(',') {
            final_vector.push(value.parse::<i32>().unwrap());
        }
    }

    final_vector
}

#[allow(dead_code)]
fn exec_day2() {
    println!("\n##### Day 2");

    let _test_program1 = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    let _test_program2: Vec<i32> = vec![1,1,1,4,99,5,6,0,99];

    let mut day2_program: Vec<i32> = read_file_to_vector("input/day2_input.txt");

    // Change the program as instructed by AoC day 2 part 1
    day2_program[1] = 12;
    day2_program[2] = 2;

    let mut computer = intcode_computer::IntcodeComputer::new();
    let intcode_value = computer.run_program(day2_program, 0);

    println!(">>>> Final value:{}", intcode_value);

    println!("##### Part 2");

    let day2_original: Vec<i32> = read_file_to_vector("input/day2_input.txt");
    let mut program_output: i32;
    let mut noun_final: i32 = 0;
    let mut verb_final: i32 = 0;
    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut day2_program_test = day2_original.clone();
            day2_program_test[1] = noun;
            day2_program_test[2] = verb;
            program_output = computer.run_program(day2_program_test, 0);

            if program_output == 19690720 {
                noun_final = noun;
                verb_final = verb;
                break 'outer;
            }
        }
    }

    println!(">>>>> noun:{} | verb:{} | answer:{}", noun_final, verb_final, 100 * noun_final + verb_final);
}

fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn is_password_viable_part1(password: u32) -> bool {
    let password_vec = number_to_vec(password);
    let mut password_sorted = password_vec.clone();

    // If the sorted password and the original password are the same, then the numbers are all in
    // ascending order, thus passing the first test
    password_sorted.sort();

    let mut is_viable: bool = false;
    if password_vec == password_sorted {
        let mut previous_digit: u32 = 0;
        for digit in password_vec {
            if digit == previous_digit {
                is_viable = true;
                break;
            }

            previous_digit = digit;
        }
    }

    return is_viable;
}

fn is_password_viable_part2(password: u32) -> bool {
    let password_vec = number_to_vec(password);
    let mut password_sorted = password_vec.clone();

    // If the sorted password and the original password are the same, then the numbers are all in
    // ascending order, thus passing the first test
    password_sorted.sort();

    let mut is_viable: bool = false;
    if password_vec == password_sorted {
        // Create a hashmap of digits and the number of times that digit is seen
        let mut digit_map: HashMap<u32, u32> = HashMap::new();
        for digit in password_vec {
            let digit_counter = digit_map.entry(digit).or_insert(0);
            *digit_counter += 1;
        }

        for count in digit_map.values() {
            // If even a single digit has a count of 2, then the whole password is viable (after
            // passing the sort test)
            if *count == 2u32 {
                is_viable = true;
                break;
            }
        }
    }

    return is_viable;
}

fn exec_day4() {
    println!("##### Day 4 Part 1");
    let mut viable_passwords = 0u32;
    for password in 372_304..847_060 {
        let is_viable = is_password_viable_part1(password);

        if is_viable {
            viable_passwords = viable_passwords + 1;
        }
    }

    println!(">>>> Number of viable passwords:{}", viable_passwords);

    println!("##### Day 4 Part 2");
    viable_passwords = 0;

    for password in 372_304..847_060 {
        let is_viable = is_password_viable_part2(password);

        if is_viable {
            viable_passwords = viable_passwords + 1;
        }
    }
    println!(">>>> Number of viable passwords:{}", viable_passwords);
}

fn main() {
    //exec_day1();
    //exec_day2();

    exec_day4();
}

#[cfg(test)]
mod day1_tests {
    use crate::{module_fuel, module_fuel_all};

    #[test]
    fn test_module_fuel() {
        assert_eq!(module_fuel(12), 2);
        assert_eq!(module_fuel(14), 2);
        assert_eq!(module_fuel(1969), 654);
        assert_eq!(module_fuel(100_756), 33583);
    }

    #[test]
    fn test_module_fuel_all() {
        assert_eq!(module_fuel_all(1969), 966);
        assert_eq!(module_fuel_all(100756), 50346);
    }
}

mod day2_tests {
}