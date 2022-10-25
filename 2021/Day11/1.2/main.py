import numpy as np
from numpy import ndarray


def parse_input(file_name: str) -> ndarray:
    with open(file_name, 'r') as file:
        co = file.read().splitlines()
        co = [[int(char) for char in item] for item in co]
        return np.array(co)


def get_flashes(inp):
    n_step = 0

    while True:
        n_step += 1
        inp += 1

        isDone = [False]
        while False in isDone:
            isDone.clear()
            for y in range(0, inp.shape[0]):
                for x in range(0, inp.shape[1]):
                    if inp[y][x] > 9:
                        inp[y][x] = 0
                        increase_surrounders_by_one(y, x, inp)
            for i in range(20, 9, -1):
                if i not in inp:
                    isDone.append(True)
                else:
                    isDone.append(False)
        if np.all(inp == 0):
            break
    return n_step


def increase_surrounders_by_one(y, x, inp):
    # check west
    if x - 1 >= 0:
        # check north-west
        if y - 1 >= 0:
            inp[y - 1][x - 1] += 1 if inp[y - 1][x - 1] != 0 else 0
        # check south-west
        if y + 1 < inp.shape[0]:
            inp[y + 1][x - 1] += 1 if inp[y + 1][x - 1] != 0 else 0
        inp[y][x - 1] += 1 if inp[y][x - 1] != 0 else 0
    # check east
    if x + 1 < inp.shape[1]:
        # check north-east
        if y - 1 >= 0:
            inp[y - 1][x + 1] += 1 if inp[y - 1][x + 1] != 0 else 0
        # check south-east
        if y + 1 < inp.shape[0]:
            inp[y + 1][x + 1] += 1 if inp[y + 1][x + 1] != 0 else 0
        inp[y][x + 1] += 1 if inp[y][x + 1] != 0 else 0
    # check north
    if y - 1 >= 0:
        inp[y - 1][x] += 1 if inp[y - 1][x] != 0 else 0
    # check south
    if y + 1 < inp.shape[0]:
        inp[y + 1][x] += 1 if inp[y + 1][x] != 0 else 0


if __name__ == "__main__":
    input_arr = parse_input('Day11/1.2/input.txt')
    n_step = get_flashes(input_arr)
    print(input_arr)
    print(f"NUMBER OF STEPS: {n_step}")
