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

pub fn main() {
    let result = read_lines("day03_input.txt");
    //println!("{:?}", result);
    println!("Part 1 - The power consumption of the submarine is {}", process_lines(&result));
}
