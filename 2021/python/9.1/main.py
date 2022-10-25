import numpy as np
from numpy import ndarray


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


if __name__ == "__main__":
    input_arr = parse_input('Day9/1.1/input.txt')
    answer = find_low_points(input_arr)
    sums = 0
    for item in answer:
        sums += item['val'] + 1
    print(sums)
