use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut digits: Vec<String> = Vec::new();
    for digit in contents.trim().replace("  ", " ").split("\n") {
        digits.push(digit.to_string());
    }
    digits
}

/*

+----------------------------------------+
|Number of Segments| Corresponding digits|
+----------------------------------------+
|                 2|                    1|
+----------------------------------------+
|                 3|                    7|
+----------------------------------------+
|                 4|                    4|
+----------------------------------------+
|                 5|              2, 3, 5|
+----------------------------------------+
|                 6|              0, 6, 9|
+----------------------------------------+
|                 7|                    8|
+----------------------------------------+

*/

fn process_lines(digits: &Vec<String>) -> i64 {
    // we don't actually have to decipher the wiring to count the occurrences
    // of the digits 1, 4, 7, and 8, because we can identify these by the
    // number of segments they have.
    // Specifically, digit 1 has 2 segments, 4 has 4 segments, 7 has 3 segments,
    // and 8 has 7 segments.
    let digit_lines = digits.clone();
    let mut total_count: i64 = 0;
    for each_line in digit_lines {
        total_count += each_line
            // we're only interested in the section of each line after the |
            .split("|").nth(1).unwrap().trim()
            // check each of the four 7-segment numerals
            .split(" ").filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            //.inspect(|x| println!("{}", x))
            .count() as i64;
    }
    total_count
}

fn process_lines_part_two(digits: &Vec<String>) -> i64 {
    let digit_lines = digits.clone();
    let mut total_sum: i64 = 0;
    let mut Vec<i64> four_digits = Vec::new();

}

pub fn main() {
    let digits = read_lines("day08_input.txt");
    println!("Part 1 - The digits 1, 4, 7, and 8 appear {} times.", process_lines(&digits));
}
