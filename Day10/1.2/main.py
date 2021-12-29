import copy
from typing import List

import numpy as np
from numpy import ndarray


class Mapping:
    open = ['(', '[', '{', '<']
    closed = [')', ']', '}', '>']
    mappings = {'(': ')',
                '[': ']',
                '{': '}',
                '<': '>'}
    scores = {')': 1,
              ']': 2,
              '}': 3,
              '>': 4}


def parse_input(file_name: str) -> ndarray:
    with open(file_name, 'r') as file:
        co = file.read().splitlines()
        co = [[char for char in item] for item in co]
        return np.array(co)


def delete_corrupted_lines_from_input(inp: ndarray):
    inp_copy = copy.deepcopy(inp)
    i_arr = []
    for i, line in zip(range(0, inp_copy.shape[0]), inp_copy):
        tmp = []
        for char in line:
            if char in Mapping.open:
                tmp.append(char)
            else:
                if char != Mapping.mappings[tmp[-1]]:
                    i_arr.append(i)
                    break
                else:
                    tmp.pop()
    inp = np.delete(inp, i_arr, 0)
    return inp


def get_completions_for_incomplete_lines(inp: ndarray):
    remainders = []
    for line in inp:
        tmp = []
        for char in line:
            if char in Mapping.open:
                tmp.append(char)
            else:
                if char == Mapping.mappings[tmp[-1]]:
                    tmp.pop()
                else:
                    print('this shouldnt happen...')
        remainders.append(tmp)

    completions = [[Mapping.mappings[char] for char in line[::-1]] for line in remainders]
    return completions


def get_autocomplete_score(completions: List[List[str]]):
    scores = []
    for line in completions:
        line_score = 0
        for char in line:
            line_score *= 5
            line_score += Mapping.scores[char]
        scores.append(line_score)

    scores.sort()
    index = int((len(scores) - 1) / 2)
    return scores[index]


if __name__ == "__main__":
    input_arr = parse_input('Day10/1.2/input.txt')
    input_arr = delete_corrupted_lines_from_input(input_arr)
    completions = get_completions_for_incomplete_lines(input_arr)
    print(completions)

    score = get_autocomplete_score(completions)
    print(score)
