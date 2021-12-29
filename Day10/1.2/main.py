import numpy as np
from numpy import ndarray


class Mapping:
    open = ['(', '[', '{', '<']
    closed = [')', ']', '}', '>']
    mappings = {'(': ')',
                '[': ']',
                '{': '}',
                '<': '>'}
    scores = {')': 3,
              ']': 57,
              '}': 1197,
              '>': 25137}


def parse_input(file_name: str) -> ndarray:
    with open(file_name, 'r') as file:
        co = file.read().splitlines()
        co = [[char for char in item] for item in co]
        return co


def get_illegal_char_from_corrupted_lines(inp: ndarray):
    illegal_chars = []
    for line in inp:
        tmp = []
        for char in line:
            if char in Mapping.open:
                tmp.append(char)
            else:
                if char != Mapping.mappings[tmp[-1]]:
                    illegal_chars.append(char)
                    break
                else:
                    tmp.pop()
    return illegal_chars


if __name__ == "__main__":
    input_arr = parse_input('Day10/1.1/input.txt')
    illegal_chars = get_illegal_char_from_corrupted_lines(input_arr)
    scores = 0
    for char in illegal_chars:
        scores += Mapping.scores[char]
    print(scores)
