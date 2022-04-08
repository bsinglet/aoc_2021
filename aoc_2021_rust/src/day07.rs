use std::fs;
use std::str::FromStr;

fn read_lines(filename: &str) -> Vec<i64> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut population: Vec<i64> = Vec::new();
    for each in contents.trim().replace("  ", " ").split(",").map(|x| i64::from_str(x)) {
        if each.is_ok() {
            population.push(each.unwrap());
        }
    }
    population
}

fn process_lines(initial_positions: &Vec<i64>) -> i64 {
    let mut crab_positions = initial_positions.clone();
    crab_positions.sort();
    // the optimal position to align on is simply the median of all positions.
    let median = crab_positions[crab_positions.len() / 2];
    // calculate the cost for all the crabs to align on that position.
    let total_fuel = crab_positions.iter().map(|x| (x - median).abs());
    total_fuel.sum()
}

fn process_lines_part_two(initial_positions: &Vec<i64>) -> i64 {
    let mut crab_positions = initial_positions.clone();
    crab_positions.sort();
    let mut best_fuel: i64 = -1;
    let mut potential_candidates = Vec::<i64>::new();
    let min: i64 = crab_positions.iter().copied().min().unwrap();
    let max: i64 = crab_positions.iter().copied().max().unwrap();
    // create a list of every possible position for the crabs to align on.
    for position in min..max {
        potential_candidates.push(position)
    }
    // check how much fuel it would take to align on any position.
    for position in potential_candidates {
        let total_fuel = crab_positions
            .iter()
            // the cost of moving n spaces is equal to the
    		// nth triangular number = (n*(n+1))/2
            .map(|&x| ((position - x).abs() * ((position - x).abs()+1))/2)
            .sum();
        if best_fuel == -1 || total_fuel < best_fuel {
            best_fuel = total_fuel;
        }
    }
    best_fuel
}

pub fn main() {
    let crab_positions = read_lines("day07_input.txt");
    println!("Part 1 - The crab population needs to spend {} units of fuel to align.", process_lines(&crab_positions));
    println!("Part 2 - The crab population needs to spend {} units of fuel to align.", process_lines_part_two(&crab_positions));
}
