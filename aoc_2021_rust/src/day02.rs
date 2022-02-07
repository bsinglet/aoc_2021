use std::fs;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward
}

fn read_lines(filename: &str) -> Vec<(Direction, i32)> {
    let mut result: Vec<(Direction, i32)> = Vec::new();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let lines: Vec<&str> = contents.split_terminator("\n").collect();
    for each_pair in lines {
        let (mut x, mut y) = (Direction::Up, 0);
        for (index, value) in each_pair.split_whitespace().enumerate()  {
            match index {
                0 => {
                    x = match value {
                        "forward" => Direction::Forward,
                        "down" => Direction::Down,
                        "up" => Direction::Up,
                        _ => {
                            println!("Not sure what to do with direction {}", value);
                            Direction::Up
                        },
                    };
                },
                1 => {
                    y = value.parse()
                        .expect("Enter an int here.");
                },
                _ => {
                    println!("Not sure what to do with index {}", index);
                },
            };
        }
        result.push((x, y));
    }
    result
}

fn process_lines(result: &Vec<(Direction, i32)>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for (direction, magnitude) in result {
        match direction {
            Direction::Up => y -= magnitude,
            Direction::Down => y += magnitude,
            Direction::Forward => x += magnitude,
            _ => println!("Don't recognize direction {:?}", direction),
        };
    }
    x * y
}

fn process_lines_part_two(result: &Vec<(Direction, i32)>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (direction, magnitude) in result {
        match direction {
            Direction::Up => aim -= magnitude,
            Direction::Down => aim += magnitude,
            Direction::Forward => {
                x += magnitude;
                y += aim * magnitude;
            },
            _ => println!("Don't recognize direction {:?}", direction),
        };
    }
    x * y
}

pub fn main() {
    let result = read_lines("day02_input.txt");
    println!("Part 1 - The final product of x and y is: {}", process_lines(&result));
    println!("Part 2 - The final product of x and y is: {}", process_lines_part_two(&result));
}
