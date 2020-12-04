use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use crate::Day3WireSegmentDirection::{WireRight, WireLeft, WireUnknown, WireDown, WireUp};
use std::fmt;

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

#[derive(Clone, Copy)]
struct Day3WireSegment {
    origin_x: i32,
    origin_y: i32,
    dest_x: i32,
    dest_y: i32,
}

impl fmt::Debug for Day3WireSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WireSegment")
            .field("origin_x", &self.origin_x)
            .field("origin_y", &self.origin_y)
            .field("dest_x", &self.dest_x)
            .field("dest_y", &self.dest_y)
            .finish()
    }
}

enum Day3WireSegmentDirection {
    WireUnknown,
    WireRight,
    WireLeft,
    WireUp,
    WireDown,
}

struct ParsedWireSegment {
    direction: Day3WireSegmentDirection,
    length: i32,
}

struct Point {
    x: f32,
    y: f32,
}

fn parse_wire_path_segment(segment: &String) -> ParsedWireSegment {
    let mut parsed_segment = ParsedWireSegment { direction: WireUnknown, length: 0 };

    // The first character will indicate the direction. Force it to uppercase to normalize it.
    let direction = segment.chars().next().unwrap();
    match direction {
        'R' => parsed_segment.direction = WireRight,

        'L' => parsed_segment.direction = WireLeft,

        'U' => parsed_segment.direction = WireUp,

        'D' => parsed_segment.direction = WireDown,

        _ => parsed_segment.direction = WireUnknown,
    }

    // The rest of the segment string represents the length
    let length_tokens: Vec<&str> = segment.split(|c| c == 'R' || c == 'L' || c == 'U' || c == 'D').collect();
    parsed_segment.length = length_tokens.last().unwrap().parse::<i32>().unwrap();

    return parsed_segment;
}

fn day3_build_wire(wire: Vec<String>) -> Vec<Day3WireSegment> {
    let mut result_wire: Vec<Day3WireSegment> = vec![];

    let mut wire_segment = Day3WireSegment { origin_x: 0, origin_y: 0, dest_x: 0, dest_y: 0 };
    let mut local_origin_x = 0;
    let mut local_origin_y = 0;

    let wire_iter = wire.iter();
    for raw_segment in wire_iter {
        wire_segment.origin_x = local_origin_x;
        wire_segment.origin_y = local_origin_y;

        let parsed_segment = parse_wire_path_segment(raw_segment);

        match parsed_segment.direction {
            WireUp => {
                wire_segment.dest_y += parsed_segment.length;
                local_origin_y += parsed_segment.length;
            },
            WireDown => {
                wire_segment.dest_y -= parsed_segment.length;
                local_origin_y -= parsed_segment.length;
            },
            WireRight => {
                wire_segment.dest_x += parsed_segment.length;
                local_origin_x += parsed_segment.length;
            },
            WireLeft => {
                wire_segment.dest_x -= parsed_segment.length;
                local_origin_x -= parsed_segment.length;
            },
            _ => {}
        }

        result_wire.push(wire_segment);
    }

    return result_wire;
}

fn get_does_intersect(start_a: &Point, end_a: &Point, start_b: &Point, end_b: &Point) -> bool {
    let ax = end_a.x - start_a.x; // direction of line a
    let ay = end_a.y - start_a.y; // ax and ay as above

    let bx = start_b.x - end_b.x; // direction of line b, reversed
    let by = start_b.y - end_b.y; // really -by and -by as above

    let dx = start_b.x - start_a.x; // right-hand side
    let dy = start_b.y - start_a.y;

    let det = (ax * by) - (ay * bx);

    if det == 0f32 {
        return false;
    }

    let r = ((dx * by) - (dy * bx)) / det;
    let s = ((ax * dy) - (ay * dx)) / det;

    return false == (r < 0f32 || r > 1f32 || s < 0f32 || s > 1f32);
}

fn line_segment_intersection(ax: f32, ay: f32, mut bx: f32, mut by: f32, mut cx: f32, mut cy: f32, mut dx: f32, mut dy: f32) -> (bool, Point) {
    let mut intersect_point = Point { x: 0f32, y: 0f32 };

    // Fail if either line segment is zero-length
    if (ax == bx && ay == by) || (cx == dx && cy == dy) {
        return (false, intersect_point);
    }

    // Fail if the segments share an end-point
    if (ax == cx && ay == cy) || (bx == cx && by == cy) ||  (ax == dx && ay == dy) || (bx == dx && by == dy) {
        return (false, intersect_point);
    }

    // (1) Translate the system so that point A is on the origin
    bx -= ax;
    by -= ay;
    cx -= ax;
    cy -= ay;
    dx -= ax;
    dy -= ay;

    // Discover the length of segment A-B
    let distance_ab = (bx * bx + by * by).sqrt();

    // (2) Rotate the system so that point B is on the positive X axis
    let the_cos = bx / distance_ab;
    let the_sin = by / distance_ab;
    let mut new_x = cx * the_cos + cy * the_sin;
    cy = cy * the_cos - cx * the_sin;
    cx = new_x;
    new_x = dx * the_cos + dy * the_sin;
    dy = dy * the_cos - dx * the_sin;
    dx = new_x;

    // Fail if segment C-D doesn't cross line A-B
    if (cy < 0f32 && dy < 0f32) || (cy >= 0f32 && dy >= 0f32) {
        return (false, intersect_point);
    }

    // (3) Discover the position of the interesection point along line A-B
    let ab_pos = dx + (cx - dx) * dy / (dy - cy);

    // Fail if segment C-D crosses line A-B outside of segment A-B
    if ab_pos < 0f32 || ab_pos > distance_ab {
        return (false, intersect_point);
    }

    // (4) Apply the discovered position to line A-B in the original coordinate system
    intersect_point.x = ax + ab_pos * the_cos;
    intersect_point.y = ay + ab_pos * the_sin;

    return (true, intersect_point);
}

fn exec_day3() {
    println!("##### Day 3 Part 1");

    // let wire1_raw = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    // let wire2_raw = "U62,R66,U55,R34,D71,R55,D58,R83";

    // let wire1_raw = "R8,U5,L5,D3";
    // let wire2_raw = "U7,R6,D4,L4";

    // let wire1_raw = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
    // let wire2_raw= "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    let wire1_raw = "R1006,U867,R355,D335,L332,U787,L938,U987,L234,U940,R393,D966,R57,U900,R619,D693,L606,U272,L45,D772,R786,U766,R860,U956,L346,D526,R536,D882,L156,U822,L247,D279,R515,U467,R208,D659,R489,D295,R18,D863,L360,D28,R674,U203,L276,U518,L936,D673,L501,D414,L635,U497,R402,D530,L589,D247,L140,U697,R626,D130,L109,D169,L316,D2,R547,D305,L480,U871,R551,D48,L710,D655,R562,D395,L925,D349,L795,U308,L861,D265,R88,U116,L719,D204,R995,D197,R167,U786,R459,U978,L506,D232,L37,U530,L808,D75,R79,D469,L851,D152,R793,D362,L293,D760,L422,U516,L400,D275,L498,U877,R202,D302,L89,U924,L55,U161,L945,D578,R861,U853,R358,D427,L776,U571,R670,D29,R52,D116,R879,U359,R493,D872,L567,U382,R804,D168,R316,D376,R711,U627,R36,D241,R876,U407,L481,D360,R679,U561,L947,U449,R232,U176,R677,U850,R165,D257,R683,D666,L31,U237,L906,U810,R198,U890,L462,D928,R915,D778,L689,U271,L486,D918,L995,U61,R748,U516,L80,D109,L328,U649,L784,D546,R584,D751,L543,U217,L976,D400,L795,U332,R453,U399,L761,U823,R142,U532,R133,U255,R722,D726,L862,D845,L813,U981,R272,D800,L918,D712,R259,U972,R323,D214,R694,D629,L817,D814,L741,U111,L678,D627,L743,D509,R195,U875,R46,D344,L361,D102,L656,U897,L841,U124,L95,D770,L785,U767,L504,D309,L955,D142,L401,U914,R117,D897,R715,D117,R675,U248,R182,D725,L751,U562,R385,D120,R730,U185,L842,D446,L432,U640,R994,D482,R576,U915,R645,U109,R77,D983,L327,D209,R686,D486,R566,D406,R130,D759,R441,U895,R597,U443,L773,D704,R219,U222,R244,D11,L723,U804,L264,D121,L81,D454,R279,D642,L773,D653,R127,D199,R119,U509,L530";
    let wire2_raw = "L1003,D933,L419,D202,L972,U621,L211,U729,R799,U680,R925,U991,L167,U800,R198,U214,R386,D385,R117,D354,L914,D992,L519,U797,L28,D125,R163,D894,R390,D421,L75,D577,L596,U95,L403,U524,L160,D39,R209,D373,L464,U622,L824,D750,L857,U658,L109,D188,R357,D295,L227,U904,L268,U814,L483,U897,R785,U194,R865,U300,L787,U812,L321,D637,R761,U560,R800,U281,R472,D283,L490,D629,L207,D589,L310,D980,R613,U129,R668,U261,R82,D594,R627,D210,L865,U184,R387,U995,R497,U68,L776,U657,R559,D38,R981,D485,L196,D934,R313,D128,R269,D225,L32,U677,R425,U728,L665,D997,R271,D847,R715,U300,L896,D481,L30,U310,L793,D600,L219,D944,R197,D945,L564,D603,L225,U413,L900,U876,R281,D26,R449,D506,L846,D979,L817,D794,R309,D841,R735,U11,R373,U530,R74,D534,L668,U185,L972,D436,L377,D164,L88,U249,L8,D427,R711,D530,L850,D921,L644,U804,L388,U500,L813,D223,L572,U246,R309,U241,R185,D48,L545,U678,L344,D964,L772,D985,L178,U686,R937,U821,R214,D346,L648,D824,L943,D651,R98,D225,R832,D883,L814,D894,L995,D975,R440,D502,L177,D320,R675,U5,R188,D866,R974,U169,R432,D627,L424,D5,L273,U184,R657,U830,R681,U610,R170,U106,L726,D861,L257,D381,L918,D607,L820,D757,R556,D621,R21,U510,L575,D545,L590,D302,R446,D225,L164,D817,L520,D204,L33,U272,L648,D478,R945,U369,L924,D932,R46,D584,R630,U592,R613,U136,R253,D343,L983,U328,L442,D311,L258,U173,L574,U658,R283,D927,L247,D37,R36,D61,L692,U663,L207,U48,L114,U511,L924,U229,L221,D504,R345,U51,R464,D516,L115,D311,L71,D418,R378,D173,R154,U436,L403,D871,L765,D803,R630,U108,L79,U625,R77,U176,R911";

    let mut wire1: Vec<String> = vec![];
    for value in wire1_raw.split(',') {
        wire1.push(value.to_string());
    }

    let mut wire2: Vec<String> = vec![];
    for value in wire2_raw.split(',') {
        wire2.push(value.to_string());
    }

    let wire1_segments = day3_build_wire(wire1);
    let wire2_segments = day3_build_wire(wire2);

    let mut distances: Vec<i32> = vec![];
    let mut step_distances: Vec<i32> = vec![];
    let mut wire1_step_distance = 0i32;
    let mut wire2_step_distance: i32;

    for segment1 in wire1_segments.iter() {
        wire1_step_distance += (segment1.dest_x - segment1.origin_x).abs() + (segment1.dest_y - segment1.origin_y).abs();
        wire2_step_distance = 0;

        for segment2 in wire2_segments.iter() {
            let start_a = Point { x: segment1.origin_x as f32, y: segment1.origin_y as f32 };
            let end_a = Point { x: segment1.dest_x as f32, y: segment1.dest_y as f32 };
            let start_b = Point { x: segment2.origin_x as f32, y: segment2.origin_y as f32 };
            let end_b = Point { x: segment2.dest_x as f32, y: segment2.dest_y as f32 };

            let does_intersect = get_does_intersect(&start_a, &end_a, &start_b, &end_b);

            wire2_step_distance += (segment2.dest_x - segment2.origin_x).abs() + (segment2.dest_y - segment2.origin_y).abs();

            if does_intersect == true {
                let point_and_validity: (bool, Point) = line_segment_intersection(start_a.x, start_a.y, end_a.x, end_a.y, start_b.x, start_b.y, end_b.x, end_b.y);
                let point_is_valid = point_and_validity.0;
                let intersect_point = point_and_validity.1;

                if point_is_valid {
                    let intersect_distance: i32 = (intersect_point.x.abs() + intersect_point.y.abs()) as i32;
                    if intersect_distance > 0i32 {
                        distances.push(intersect_distance);
                    }

                    // Determine the length of wire _after_ the intersection so it can be subtracted
                    // from the length of the final segment.
                    let overflow1: i32 = (intersect_point.x as i32 - segment1.dest_x).abs() + (intersect_point.y as i32 - segment1.dest_y).abs();
                    let overflow2: i32 = (intersect_point.x as i32 - segment2.dest_x).abs() + (intersect_point.y as i32 - segment2.dest_y).abs();

                    step_distances.push(wire1_step_distance - overflow1 + wire2_step_distance - overflow2);
                }
            }
        }
    }

    distances.sort();
    println!(">>>> Closest intersection distance: {}", distances.first().unwrap());

    step_distances.sort();
    println!(">>>> Shortest segment distance: {}", step_distances.first().unwrap());
}

#[allow(dead_code)]
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
    exec_day3();

    //exec_day4();
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