from typing import List


def parse_input(file_name: str) -> List[int]:
    with open(file_name, 'r') as file:
        co = file.read().split(',')
        co = [int(x) for x in co]
        return co


def get_minimal_fuel(co):
    min_val = min(co)
    max_val = max(co)

    last_diff = 0
    best_fit = 0
    for avg in range(min_val, max_val + 1):
        diff = 0
        for c in co:
            diff += abs(avg-c)

        if last_diff == 0:
            last_diff = diff
            continue
        if diff < last_diff:
            last_diff = diff
            best_fit = avg
    return last_diff


if __name__ == "__main__":
    coordinates = parse_input('Day7/1.1/input.txt')
    minimal_fuel_usage = get_minimal_fuel(coordinates)
    print(minimal_fuel_usage)

