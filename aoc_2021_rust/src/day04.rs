use std::fs;
use std::str::FromStr;

fn read_lines(filename: &str) -> (Vec<i32>, Vec<[[i32;5];5]>) {
    let numbers: Vec<i32>;
    let mut boards: Vec<[[i32;5];5]> = Vec::new();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let lines: Vec<&str> = contents.split_terminator("\n").collect();

    numbers = lines[0].split(",").map(|x| i32::from_str(x).unwrap()).collect();

    let mut line_index = 2;
    while line_index < lines.len() {
        let mut board: [[i32;5];5] = [[0, 0, 0, 0, 0],
                                      [0, 0, 0, 0, 0],
                                      [0, 0, 0, 0, 0],
                                      [0, 0, 0, 0, 0],
                                      [0, 0, 0, 0, 0]];
        for row in 0..5 {
            for (index, value) in lines[line_index].trim().replace("  ", " ").split(" ").enumerate() {
                //println!("Trying to parse {} as an integer", value);
                board[row][index] = i32::from_str(value).unwrap();
            }
            line_index += 1;
        }
        boards.push(board);

        // there's a blank line after each board
        line_index += 1;
    }
    (numbers, boards)
}

fn find_winners(marked_boards: &Vec<[[bool;5];5]>, mut winners: Vec<i32>) -> Vec<i32> {
    for (board_number, each_board) in marked_boards.iter().enumerate() {
        // ignore boards that are already winners
        if winners.contains(&(board_number as i32)) {
            continue;
        }
        // check rows, columns, and diagonals for any five trues in a row
        for each_row in each_board {
            if each_row.iter().all(|&x| x == true) {
                if !winners.contains(&(board_number as i32)) {
                    winners.push(board_number as i32);
                }
                break;
            }
        }

        for each_column in 0..each_board[0].len() {
            let this_column = [each_board[0][each_column],
                                   each_board[1][each_column],
                                   each_board[2][each_column],
                                   each_board[3][each_column],
                                   each_board[4][each_column]];
            if this_column.iter().all(|&x| x == true) {
                if !winners.contains(&(board_number as i32)) {
                    winners.push(board_number as i32);
                }
                break;
            }
        }

        // check upper-left to lower-right diagnoal
        let mut this_diagonal = [each_board[0][0],
                                 each_board[1][1],
                                 each_board[2][2],
                                 each_board[3][3],
                                 each_board[4][4]];
        if this_diagonal.iter().all(|&x| x == true) {
            if !winners.contains(&(board_number as i32)) {
                winners.push(board_number as i32);
            }
        }

        // check lower-left to upper-right diagnoal
        this_diagonal = [each_board[4][0],
                                 each_board[3][1],
                                 each_board[2][2],
                                 each_board[1][3],
                                 each_board[0][4]];
        if this_diagonal.iter().all(|&x| x == true) {
            if !winners.contains(&(board_number as i32)) {
                winners.push(board_number as i32);
            }
        }
    }
    winners
}

#[test]
fn test_find_winners_empty() {
    let marked_boards: Vec<[[bool;5];5]> = Vec::new();
    assert_eq!(find_winners(&marked_boards).len(), 0);
}

#[test]
fn test_find_winners_none() {
    let mut marked_boards: Vec<[[bool;5];5]> = Vec::new();
    marked_boards.push([[false, false, false, false, false],
                                        [true, true, true, false, true],
                                        [false, false, false, false, false],
                                        [false, false, false, false, false],
                                        [false, false, false, false, false]]);
    assert_eq!(find_winners(&marked_boards).len(), 0);
}

#[test]
fn test_find_winners_horizontal() {
    let mut marked_boards: Vec<[[bool;5];5]> = Vec::new();
    marked_boards.push([[false, false, false, false, false],
                                        [true, true, true, true, true],
                                        [false, false, false, false, false],
                                        [false, false, false, false, false],
                                        [false, false, false, false, false]]);
    assert_eq!(find_winners(&marked_boards)[0], 0);
}

#[test]
fn test_find_winners_vertical() {
    let mut marked_boards: Vec<[[bool;5];5]> = Vec::new();
    marked_boards.push([[false, true, false, false, false],
                                        [true, true, false, true, true],
                                        [false, true, false, false, false],
                                        [false, true, false, false, false],
                                        [false, true, false, false, false]]);
    assert_eq!(find_winners(&marked_boards)[0], 0);
}

fn mark_boards(boards: &Vec<[[i32;5];5]>, mut marked_boards: Vec<[[bool;5];5]>, number: i32) -> Vec<[[bool;5];5]> {
    for board_index in 0..boards.len() {
        for row in 0..boards[board_index].len() {
            for column in 0..boards[board_index][row].len() {
                if boards[board_index][row][column] == number {
                    marked_boards[board_index][row][column] = true;
                }
            }
        }
    }
    marked_boards
}

fn get_score(board: [[i32;5];5], marked: [[bool;5];5]) -> i32{
    let mut score: i32 = 0;
    for row in 0..board.len() {
        for column in 0..board.len() {
            if !marked[row][column] {
                score += board[row][column];
            }
        }
    }
    score
}

fn process_lines(numbers: &Vec<i32>, boards: &Vec<[[i32;5];5]>) -> i32 {
    let mut marked_boards: Vec<[[bool;5];5]> = Vec::new();
    // initialize our tracking of marked spaces on the bingo boards
    for _ in 0..boards.len() {
        let this_board: [[bool;5];5] = [[false, false, false, false, false],
                                            [false, false, false, false, false],
                                            [false, false, false, false, false],
                                            [false, false, false, false, false],
                                            [false, false, false, false, false]];
        marked_boards.push(this_board);
    }

    // iterate through the called numbers until one board wins
    let mut numbers_called = 0;
    let mut winners = find_winners(&marked_boards, Vec::<i32>::new());
    while winners.len() == 0 {
        marked_boards = mark_boards(&boards, marked_boards, numbers[numbers_called]);
        winners = find_winners(&marked_boards, winners);
        numbers_called += 1;
    }

    let score: i32 = get_score(boards[winners[0] as usize], marked_boards[winners[0] as usize]) * numbers[numbers_called-1 as usize];
    println!("After {} moves, winning board is number {}, with a score of {}", numbers_called-1, winners[0], score);
    score
}

fn process_lines_part_two(numbers: &Vec<i32>, boards: &Vec<[[i32;5];5]>) -> i32 {
    let mut marked_boards: Vec<[[bool;5];5]> = Vec::new();
    // initialize our tracking of marked spaces on the bingo boards
    for _ in 0..boards.len() {
        let this_board: [[bool;5];5] = [[false, false, false, false, false],
                                            [false, false, false, false, false],
                                            [false, false, false, false, false],
                                            [false, false, false, false, false],
                                            [false, false, false, false, false]];
        marked_boards.push(this_board);
    }

    // iterate through the called numbers until one board wins
    let mut numbers_called = 0;
    let mut winners = find_winners(&marked_boards, Vec::<i32>::new());
    while winners.len() < boards.len() {
        marked_boards = mark_boards(&boards, marked_boards, numbers[numbers_called]);
        winners = find_winners(&marked_boards, winners);
        //println!("At step {}, there are {} winning boards out of {}.", numbers_called, winners.len(), marked_boards.len());
        numbers_called += 1;
    }

    let winner = winners.pop().unwrap();
    let score: i32 = get_score(boards[winner as usize], marked_boards[winner as usize]) * numbers[numbers_called-1 as usize];
    println!("After {} moves, winning board is number {}, with a score of {}", numbers_called-1, winner, score);
    //println!("Move sequence was {:#?}", &numbers.iter().take(numbers_called));
    //println!("Winning board was marked {:#?}", &marked_boards[winner as usize]);
    score
}

pub fn main() {
    let (numbers, boards) = read_lines("day04_input.txt");
    println!("Part 1 - The final score will be {}\n", process_lines(&numbers, &boards));
    println!("Part 2 - The final score will be {}", process_lines_part_two(&numbers, &boards));
}
