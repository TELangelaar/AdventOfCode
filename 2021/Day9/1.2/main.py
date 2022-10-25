from typing import List

import numpy as np
from numpy import ndarray

from functools import reduce


def parse_input(file_name: str) -> ndarray:
    with open(file_name, 'r') as file:
        co = file.read().splitlines()
        co = [[int(x) for x in line] for line in co]
        return np.array(co, dtype=np.int8)


def find_low_points(inp: ndarray):
    result = []
    for y in range(0, inp.shape[0]):
        for x in range(0, inp.shape[1]):
            if is_lowest_compared_to_adjacent(x, y, inp):
                result.append({"x": x, "y": y, "val": inp[y][x]})
    return result


def is_lowest_compared_to_adjacent(x: int, y: int, inp: ndarray) -> bool:
    # check left
    if x - 1 >= 0:
        if inp[y][x - 1] <= inp[y][x]:
            return False
    # check right
    if x + 1 < inp.shape[1]:
        if inp[y][x + 1] <= inp[y][x]:
            return False
    # check above
    if y - 1 >= 0:
        if inp[y - 1][x] <= inp[y][x]:
            return False
    # check below
    if y + 1 < inp.shape[0]:
        if inp[y + 1][x] <= inp[y][x]:
            return False
    return True


def find_three_largest_basins(ans: List[dict], inp: ndarray) -> List[int]:
    for item in ans:
        item["basin_size"] = find_basin_size(item, inp)

    res = [item["basin_size"] for item in ans]
    res.sort(reverse=True)
    return res[:3]


def find_basin_size(it: dict, inp: ndarray) -> int:
    return find_n_largest_compared_to_adjacent(it['x'], it['y'], inp)


def find_n_largest_compared_to_adjacent(x: int, y: int, inp: ndarray) -> int:
    tmp_counter = 1

    # check left
    if x - 1 >= 0:
        inp[y][x] = -100
        target = inp[y][x - 1]
        if target != 9 and target != -100:
            tmp_counter += find_n_largest_compared_to_adjacent(x - 1, y, inp)
    # check right
    if x + 1 < inp.shape[1]:
        inp[y][x] = -100
        target = inp[y][x + 1]
        if target != 9 and target != -100:
            tmp_counter += find_n_largest_compared_to_adjacent(x + 1, y, inp)
    # check above
    if y - 1 >= 0:
        inp[y][x] = -100
        target = inp[y - 1][x]
        if target != 9 and target != -100:
            tmp_counter += find_n_largest_compared_to_adjacent(x, y - 1, inp)
    # check below
    if y + 1 < inp.shape[0]:
        inp[y][x] = -100
        target = inp[y + 1][x]
        if target != 9 and target != -100:
            tmp_counter += find_n_largest_compared_to_adjacent(x, y + 1, inp)

    return tmp_counter


if __name__ == "__main__":
    input_arr = parse_input('Day9/1.1/input.txt')
    answer = find_low_points(input_arr)
    answer = find_three_largest_basins(answer, input_arr)

    print(reduce(lambda x, y: x*y, answer))
