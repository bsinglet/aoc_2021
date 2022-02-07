use std::fs;

fn read_lines(filename: &str) -> Vec<[i32;12]> {
    let mut result: Vec<[i32;12]> = Vec::new();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let lines: Vec<&str> = contents.split_terminator("\n").collect();
    for each_line in lines {
        let mut digits: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for (index, value) in each_line.trim().chars().enumerate()  {
            digits[index] = value as i32 - 0x30;
        };
        result.push(digits);
    }
    result
}

fn process_lines(result: &Vec<[i32;12]>) -> i32 {
    let mut gamma_rate: [i32;12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut epsilon_rate: [i32;12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // add all the bits of a column together.
    for each_line in result {
        for (index, value) in each_line.iter().enumerate() {
            gamma_rate[index] += value;
        }
    }
    // convert these sums to averages, so each position is a 0 or 1.
    let mut final_gamma_rate = 0;
    let mut final_epsilon_rate = 0;
    for index in 0..gamma_rate.len() {
        gamma_rate[index] = ((gamma_rate[index] as f64) / (result.len() as f64)).round() as i32;
        epsilon_rate[index] = match gamma_rate[index] {
            0 => 1,
            1 => 0,
            _ => {
                println!("Yikes, trying to invert bit number {} with value {}", index, gamma_rate[index]);
                0
            },
        };
        //println!("Digit {} of gamma rate is {}", index, gamma_rate[index]);
        //println!("So, increasing final gamma rate by {}", gamma_rate[index] * i32::pow(2, (12 - (index as i32)) as u32));
        final_gamma_rate += gamma_rate[index] * i32::pow(2, (11 - (index as i32)) as u32);
        final_epsilon_rate += epsilon_rate[index] * i32::pow(2, (11 - (index as i32)) as u32);
    }
    println!("Part 1 - Gamma rate: {}", final_gamma_rate);
    println!("Part 1 - Epsilon rate: {}", final_epsilon_rate);
    final_gamma_rate * final_epsilon_rate
}

fn find_most_common(input: &Vec<[i32;12]>, index: usize) -> Vec<[i32;12]> {
    let (ones, zeroes): (Vec<[i32;12]>, Vec<[i32;12]>) = input
            .iter()
            .partition(|&x| x[index] == 1);

    if ones.len() >= zeroes.len() {
        return ones;
    }
    zeroes
}

fn find_least_common(input: &Vec<[i32;12]>, index: usize) -> Vec<[i32;12]> {
    let (ones, zeroes): (Vec<[i32;12]>, Vec<[i32;12]>) = input
            .iter()
            .partition(|&x| x[index] == 1);

    if zeroes.len() <= ones.len() {
        return zeroes;
    }
    ones
}

// there's probably a more idiomatic way to do this, but this works well
fn deep_copy(result: &Vec<[i32;12]>) -> Vec<[i32;12]> {
    let mut candidates: Vec<[i32;12]> = Vec::new();
    for each_line in result {
        let mut digits: [i32;12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for (index, value) in each_line.iter().enumerate() {
            digits[index] = *value;
        }
        candidates.push(digits);
    }
    candidates
}

fn bits_array_to_int(input: [i32;12]) -> i32 {
    let mut result = 0;
    for index in 0..input.len() {
        result += input[index] * i32::pow(2, (11 - (index as i32)) as u32);
    }
    result
}

fn find_rating(result: &Vec<[i32;12]>, most_common: bool) -> i32 {
    // we're going to start with a deep copy of our vector
    let mut candidates = deep_copy(result);

    // we're going to go narrow down the numbers, one position at a time
    for index in 0..12 {
        candidates = match most_common {
            true => find_most_common(&candidates, index),
            false => find_least_common(&candidates, index),
        };

        // we need to quit once we've narrowed it down to only one number
        if candidates.len() == 1 {
            break;
        }
    }

    // collapse the array of bits into an integer
    bits_array_to_int(candidates[0])
}

fn process_lines_part_two(result: &Vec<[i32;12]>) -> i32 {
    let oxygen_rating = find_rating(&result, true);
    let co2_rating = find_rating(&result, false);
    println!("Part 2 - Oxygen rating: {}", oxygen_rating);
    println!("Part 2 - CO2 rating: {}", co2_rating);
    oxygen_rating * co2_rating
}

pub fn main() {
    let result = read_lines("day03_input.txt");
    println!("Part 1 - The power consumption of the submarine is {}", process_lines(&result));
    println!("Part 2 - The life support rating of the submarine is {}", process_lines_part_two(&result));
}
