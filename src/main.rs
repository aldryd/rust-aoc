use std::io::{BufReader, BufRead};
use std::fs::File;

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

fn main() {
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

#[cfg(test)]
mod tests {
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