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

fn format_hashmap(my_hashmap: &HashMap::<i32, i32>) {
    //let mut output = String::new();
    print!("{{");
    for index in 0..my_hashmap.len() {
        print!("{}: {}, ", index, &my_hashmap[&(index as i32)]);
    }
    print!("}}");
}

fn process_lines(initial_population: &Vec<i32>, num_days: i32) -> i32 {
    let mut population = HashMap::<i32, i32>::new();
    let mut next_population = HashMap::<i32, i32>::new();
    // We track the population as a hashmap with keys 0 through 8 representing
    // the number of lanternfish of that age. E.g., next_population[3] is the
    // number of 3-day old lanternfish
    for index in 0..9 {
        population.insert(index, initial_population.iter().filter(|&x| x == &index).count().try_into().unwrap());
    }
    // for each day in the simulation age up and reproduce lanternfish as necessary.
    for day_number in 0..num_days {
        next_population.clear();
        // special cases for these ages
        next_population.insert(8, population[&0]);
        next_population.insert(7, population[&8]);
        next_population.insert(6, population[&0] + population[&7]);
        // age up the rest by one day
		for index in 0..6 {
            next_population.insert(index, population[&(index+1)]);
        }
        // lastly, set tomorrow's population to next_population so we can simulate the next day.
        //print!("On day {}, population is ", day_number);
        //format_hashmap(&population);
        //print!(" and next_population is ");
        //format_hashmap(&next_population);
        //println!("");
        for each_key in next_population.keys() {
            if let Some(mut value) = population.get_mut(each_key) {
                *value = next_population[each_key];
            }
        }
    }
    // the final population is the count of all lanternfish of any age.
    population.values().sum()
}

pub fn main() {
    let population = read_lines("day06_input.txt");
    //println!("{:#?}", population);
    let mut num_days = 80;
    println!("Part 1 - The population of the lanternfish after {} days is {}", num_days, process_lines(&population, num_days));
    num_days = 256;
    //println!("Part 2 - The population of the lanternfish after {} days is {}", num_days, process_lines_part_two(&population, num_days));
}
