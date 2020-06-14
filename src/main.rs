use std::io::{BufReader, BufRead};
use std::fs::File;

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
}

fn main() {
    exec_day1();
    exec_day2();
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