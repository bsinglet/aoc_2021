use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<i32> {
    let mut contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut population: Vec<i32> = Vec::new();
    for each in contents.trim().replace("  ", " ").split(",").map(|x| i32::from_str(x)) {
        if each.is_ok() {
            population.push(each.unwrap());
        }
    }
    population
}

fn process_lines(line_segments: &Vec<i32>) -> i32 {
    0
}

pub fn main() {
    let population = read_lines("day06_input.txt");
    println!("{:#?}", population);
    /*let mut num_days = 80;
    println!("Part 1 - The population of the lanternship after {} days is {}", num_days, process_lines(&line_segments));
    num_days = 256;
    println!("Part 2 - The population of the lanternship after {} days is {}", num_days, process_lines_part_two(&line_segments));*/
}
