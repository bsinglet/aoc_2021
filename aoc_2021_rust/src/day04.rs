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

fn find_winners(marked_boards: &Vec<[[bool;5];5]>) -> Vec<i32> {
    let mut winners: Vec<i32> = Vec::new();
    for (board_number, each_board) in marked_boards.iter().enumerate() {
        // check rows, columns, and diagonals for any five trues in a row
        for each_row in each_board {
            if each_row.iter().all(|&x| x == true) {
                winners.push(board_number as i32);
                break;
            }
        }

        for each_column in 0..each_board[0].len() {
            let mut this_column = [each_board[0][each_column],
                                   each_board[1][each_column],
                                   each_board[2][each_column],
                                   each_board[3][each_column],
                                   each_board[4][each_column]];
            if this_column.iter().all(|&x| x == true) {
                winners.push(board_number as i32);
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
            winners.push(board_number as i32);
            break;
        }

        // check lower-left to upper-right diagnoal
        this_diagonal = [each_board[4][0],
                                 each_board[3][1],
                                 each_board[2][2],
                                 each_board[1][3],
                                 each_board[0][4]];
        if this_diagonal.iter().all(|&x| x == true) {
            winners.push(board_number as i32);
            break;
        }
    }
    winners
}

fn mark_boards(boards: &Vec<[[i32;5];5]>, mut marked_boards: Vec<[[bool;5];5]>, number: i32) {
    for board_index in 0..boards.len() {
        for row in 0..boards[board_index].len() {
            let column = boards[board_index].iter().find(|&&x| x == number);
            if column != None {
                marked_boards[board_index][row][column] = true;
            }
        }
    }
}

fn process_lines(numbers: &Vec<i32>, boards: &Vec<[[i32;5];5]>) -> i32 {
    let mut marked_boards: Vec<[[bool;5];5]> = Vec::new();
    // initialize our tracking of marked spaces on the bingo boards
    for index in 0..boards.len() {
        let mut this_board: [[bool;5];5] = [[false, false, false, false, false],
                                            [false, false, false, false, false],
                                            [false, false, false, false, false],
                                            [false, false, false, false, false],
                                            [false, false, false, false, false]];
        marked_boards.push(this_board);
    }

    // iterate through the called numbers until one board wins
    let mut numbers_called = 0;
    let mut winners = find_winners(&marked_boards);
    while winners.len() == 0 {
        mark_boards(&boards, marked_boards, numbers[numbers_called]);
        winners = find_winners(&marked_boards);
        numbers_called += 1;
    }

    println!("After {} moves, winning board is number {}", numbers_called, winners[0]);
    5
}

pub fn main() {
    let (numbers, boards) = read_lines("day04_input.txt");
    println!("Part 1 - The final score will be {}", process_lines(&numbers, &boards));
}
