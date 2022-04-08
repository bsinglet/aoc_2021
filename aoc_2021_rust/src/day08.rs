use std::fs;
use std::str::FromStr;

fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut digits: Vec<String> = Vec::new();
    for digit in contents.trim().replace("  ", " ").split(" ") {
        digits.push(digit.to_string());
    }
    digits
}

fn process_lines(digits: &Vec<String>) -> i64 {
    0
}

pub fn main() {
    let digits = read_lines("day08_input.txt");
    println!("Part 1 - The digits 1, 4, 7, and 8 appear {} times.", process_lines(&digits));
}
