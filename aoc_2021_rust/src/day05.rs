use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LineSegment {
    start_x: i32,
    start_y: i32,
    end_x:   i32,
    end_y:   i32,
}

fn read_lines(filename: &str) -> Vec<LineSegment> {
    let mut line_segments: Vec<LineSegment> = Vec::new();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    for each_line in contents.split_terminator("\n") {
        if each_line.trim() == "" {
            continue;
        }
        let each_line: Vec<&str> = each_line.split("->").collect();
        line_segments.push(LineSegment{
            start_x: i32::from_str(each_line[0].split(",").collect::<Vec<&str>>()[0]).unwrap(),
            start_y: i32::from_str(each_line[0].split(",").collect::<Vec<&str>>()[1].trim()).unwrap(),
            end_x:   i32::from_str(each_line[1].split(",").collect::<Vec<&str>>()[0].trim()).unwrap(),
            end_y:   i32::from_str(each_line[1].split(",").collect::<Vec<&str>>()[1].trim()).unwrap(),
        })
    }
    line_segments
}

fn plot_points(mut points: HashMap<(i32, i32), i32>, segment: &LineSegment) -> HashMap<(i32, i32), i32> {
    // vertical line
    if segment.start_x == segment.end_x {
        let mut start_y = segment.start_y;
        let mut end_y = segment.end_y;
        if segment.start_y > segment.end_y {
            start_y = segment.end_y;
            end_y = segment.start_y;
        }
        for y in start_y..end_y+1 {
            if let Some(value) = points.get_mut(&(segment.start_x, y)) {
                *value = *value + 1;
            }else {
                points.insert((segment.start_x, y), 1);
            }
        }
    // horizontal line
    } else if segment.start_y == segment.end_y {
        let mut start_x = segment.start_x;
        let mut end_x = segment.end_x;
        if segment.start_x > segment.end_x {
            start_x = segment.end_x;
            end_x = segment.start_x;
        }
        for x in start_x..end_x+1 {
            if let Some(value) = points.get_mut(&(x, segment.start_y)) {
                *value = *value + 1;
            }else {
                points.insert((x, segment.start_y), 1);
            }
        }
    }
    points
}

fn process_lines(line_segments: &Vec<LineSegment>) -> i32 {
    let mut points = HashMap::<(i32, i32), i32>::new();
    for each_segment in line_segments {
        points = plot_points(points, &each_segment);
    }
    //println!("{:#?}", points);
    points.values().filter(|&&x| x > 1).count().try_into().unwrap()
}

pub fn main() {
    let line_segments = read_lines("day05_input.txt");
    println!("Part 1 - The number of points where at least two lines cross is {}\n", process_lines(&line_segments));
    //println!("Part 2 - The final score will be {}", process_lines_part_two(&numbers, &boards));
}
