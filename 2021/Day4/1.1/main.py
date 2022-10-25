from typing import List


def parse_input(file_name: str) -> (List[int], dict):
    with open(file_name, 'r') as file:
        drawn_numbers = file.readline()
        drawn_numbers_list = drawn_numbers.split(',')
        drawn_numbers_list = [int(x) for x in drawn_numbers_list]

        boards_raw = file.read().splitlines()
        boards = {}
        board_number = 0
        for string in boards_raw:
            if string == '':
                board_number += 1
            else:
                row = string.split(' ')
                row = [int(x) for x in row if x != '']
                if board_number not in boards:
                    boards[board_number] = []
                boards[board_number].append(row)
        return drawn_numbers_list, boards


def find_final_score(drawn_numbers: List[int], boards: dict) -> int:
    copy_boards = boards.copy()
    lastdrawn = -1
    for drawn in drawn_numbers:
        for board in boards.keys():
            for idx, row in zip(range(0, len(boards[board])), boards[board]):
                if len(row) == 0:
                    return sum_unmarked_elements(boards[board]) * lastdrawn
                for element in row:
                    if drawn == element:
                        copy_boards[board][idx].remove(element)
        lastdrawn = drawn
    return -1


def sum_unmarked_elements(board: List[int]) -> int:
    complete_sum = 0
    for row in board:
        complete_sum += sum(row)
    return complete_sum


if __name__ == '__main__':
    drawn_numbers, boards = parse_input('Day4/1.1/input.txt')
    score = find_final_score(drawn_numbers, boards)
    print(score)
