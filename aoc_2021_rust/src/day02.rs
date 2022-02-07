use std::fs;

const DIRECTION_UP: i32      = 0;
const DIRECTION_DOWN: i32    = 1;
const DIRECTION_FORWARD: i32 = 2;

fn read_lines(filename: &str) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let lines: Vec<&str> = contents.split_terminator("\n").collect();
    for each_pair in lines {
        let (mut x, mut y) = (0, 0);
        for (index, value) in each_pair.split_whitespace().enumerate()  {
            match index {
                0 => {
                    x = match value {
                        "forward" => DIRECTION_FORWARD,
                        "down" => DIRECTION_DOWN,
                        "up" => DIRECTION_UP,
                        _ => {
                            println!("Not sure what to do with direction {}", value);
                            -1
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

fn process_lines(result: &Vec<(i32, i32)>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for (direction, magnitude) in result {
        match direction {
            0 => y -= magnitude, // up
            1 => y += magnitude, // down
            2 => x += magnitude, // forward
            _ => println!("Don't recognize direction {}", direction),
        };
    }
    x * y
}

fn process_lines_part_two(result: &Vec<(i32, i32)>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (direction, magnitude) in result {
        match direction {
            0 => aim -= magnitude, // up
            1 => aim += magnitude, // down
            2 => {                 // forward
                x += magnitude;
                y += aim * magnitude;
            },
            _ => println!("Don't recognize direction {}", direction),
        };
    }
    x * y
}

pub fn main() {
    let result = read_lines("input.txt");
    println!("Part 1 - The final product of x and y is: {}", process_lines(&result));
    println!("Part 2 - The final product of x and y is: {}", process_lines_part_two(&result));
}
