import copy

def load_input(filename):
        my_lines = ''
        with open(filename, 'r') as my_file:
                my_lines = my_file.read()
        my_lines = my_lines.strip()
        # change any repeated spaces to single spaces
        my_lines = my_lines.replace('  ', ' ')
        moves = [int(x) for x in my_lines.split('\n')[0].strip().split(',')]
        my_lines = '\n'.join(my_lines.split('\n')[2:])
        boards = my_lines.split("\n\n")
        boards = [x.strip() for x in boards]
        for index, board in enumerate(boards):
                if board.strip() == '':
                        continue
                board = [row.split(' ') for row in board.split('\n')]
                if len(board[-1]) == 0:
                        board = board[:-2]
                for row_index, row in enumerate(board):
                        # print(f"Processing row {row_index} of board {board}")
                        board[row_index] = [int(x) for x in row if x != '']
                boards[index] = board
        # remove the last (empty) element
        if len(boards[-1]) == 0:
                boards = boards[:-2]
        return moves, boards

def winning_position(my_board):
        size_x, size_y = (len(my_board[0]), len(my_board))
        for row in range(size_y):
                if sum(my_board[row]) == size_x:
                        return True
        for column in range(size_x):
                if sum([my_board[y][column] for y in range(size_y)]) == size_y:
                        return True
        return False

def setup_boards(size_x, size_y, num_boards):
        marked_boards = list()
        my_board = list()
        for y in range(size_y):
                my_board.append([0 for x in range(size_x)])
        # print(f"Setting up marked board:\n{my_board}")
        for index in range(num_boards):
                marked_boards.append(copy.deepcopy(my_board))
        return marked_boards

def mark_board(board, marked_board, move):
        for y in range(len(board)):
                for x in range(len(board[0])):
                        if board[y][x] == move:
                                marked_board[y][x] = 1
        return marked_board

def get_score(board, marked_board):
        score = 0
        # print(f"Calculating score of winning board\n{board}")
        # print(f"With marked board\n{marked_board}")
        for y in range(len(board)):
                # print(f"Checking row {y} of the board")
                # we simply add up all unmarked numbers on the board
                score += sum([board[y][x] for x in range(len(board[0])) if marked_board[y][x] == 0])
        return score

def play_moves(moves, boards):
        marked_boards = setup_boards(len(boards[0][0]), len(boards[0]), len(boards))
        # print(f"There are {len(boards)} boards and {len(marked_boards)} marked boards.")
        remaining_boards = set(range(len(boards)))
        for move_number, each_move in enumerate(moves):
                current_remaining = copy.deepcopy(remaining_boards)
                for each_index in current_remaining:
                        marked_boards[each_index] = mark_board(boards[each_index], marked_boards[each_index], each_move)
                        if winning_position(marked_boards[each_index]):
                                if len(remaining_boards) > 1:
                                        remaining_boards.discard(each_index)
                                        continue
                                else:
                                        print(f'Hurray! Winning number was {each_move} on board number {each_index}.')
                                        print(f'Move sequence was {moves[0:move_number+1]}')
                                        score = get_score(boards[each_index], marked_boards[each_index])
                                        print(f"The final score is {score} * {each_move} = {score * each_move}.")
                                        return
        print(f"Uhhh, reached the end of moves without finding any winners.")

def main():
        moves, boards = load_input('day04.txt')
        play_moves(moves, boards)


if __name__ == '__main__':
        main()

