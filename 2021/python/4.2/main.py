from typing import List
import copy


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
    bingo = []

    copy_boards = copy.deepcopy(boards)
    lastdrawn = -1
    number_of_boards = len(boards)
    for drawn in drawn_numbers:
        for board in boards.keys():
            # transponeren van matrix
            vert_bingo = []
            for ii in range(0, len(boards[board][0])):
                vert_bingo.append([])
                for jj in range(0, len(boards[board])):
                    vert_bingo[ii].append(copy_boards[board][jj][ii])

            for idx, row in zip(range(0, len(boards[board])), boards[board]):
                # horizontal bingo
                if len(remove_none(copy_boards[board][idx])) == 0:
                    bingo.append(board)
                # vertical bingo
                if len(remove_none(vert_bingo[idx])) == 0:
                    bingo.append(board)

                if len(set(bingo)) == number_of_boards:
                    return sum_unmarked_elements(copy_boards[board]) * lastdrawn

                for idx3, element in zip(range(0, len(row)), row):
                    if drawn == element:
                        copy_boards[board][idx][idx3] = None
        lastdrawn = drawn
    return -1


def sum_unmarked_elements(board: List[List[int]]) -> int:
    complete_sum = 0
    for row in board:
        complete_sum += sum(remove_none(row))
    return complete_sum


def remove_none(lst):
    return [x for x in lst if x is not None]


if __name__ == '__main__':
    drawn_numbers, boards = parse_input('Day4/1.2/input.txt')
    score = find_final_score(drawn_numbers, boards)
    print(score)
