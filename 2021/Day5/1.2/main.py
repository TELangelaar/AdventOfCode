from typing import List
import pprint


def parse_input(file_name: str) -> List[List[int]]:
    with open(file_name, 'r') as file:
        co = file.read().splitlines()
        co = [x.split(' -> ') for x in co]
        co = [x.split(',') for pair in co for x in pair]
        co = [list(map(int, x)) for x in co]
        return co


def fill_diagram(co, dia=None):
    if dia is None:
        dia = get_empty_diagram()

    for idx_pair in range(0, len(co)-1, 2):
        x1, y1 = co[idx_pair][0], co[idx_pair][1]
        x2, y2 = co[idx_pair + 1][0], co[idx_pair + 1][1]

        diff_x, diff_y = abs(x1-x2), abs(y1-y2)
        if diff_x == diff_y:
            x_smallest = min((x1, y1), (x2, y2), key=lambda x: x[0])
            x_biggest = max((x1, y1), (x2, y2), key=lambda x: x[0])
            if x_smallest[1] < x_biggest[1]:
                for i, j in zip(range(x_smallest[0], x_biggest[0] + 1), range(x_smallest[1], x_biggest[1] + 1)):
                    place_dia_value(dia, i, j)
            else:
                for i, j in zip(range(x_smallest[0], x_biggest[0] + 1), range(x_smallest[1], x_biggest[1] - 1, -1)):
                    place_dia_value(dia, i, j)

        elif x1 == x2:
            y_smallest = min(y1, y2)
            y_biggest = max(y1, y2)
            ys = [x for x in range(y_smallest, y_biggest + 1)]
            for y in ys:
                place_dia_value(dia, x1, y)
        elif y1 == y2:
            x_smallest = min(x1, x2)
            x_biggest = max(x1, x2)
            xs = [x for x in range(x_smallest, x_biggest + 1)]
            for x in xs:
                place_dia_value(dia, x, y1)

    return dia


def place_dia_value(dia, x, y):
    if dia[y][x] == '.':
        dia[y][x] = '1'
    else:
        current = int(dia[y][x])
        dia[y][x] = str(current + 1)


def get_empty_diagram(size=9) -> [List[List[str]]]:
    return [['.' for _ in range(0, size + 1)] for _ in range(0, size + 1)]


if __name__ == "__main__":
    coordinates = parse_input('Day5/1.2/input.txt')
    diagram = get_empty_diagram(max(list(map(max, coordinates))))
    fill_diagram(coordinates, diagram)
    n_overlapping_coordinates = [1 for y in diagram for x in y if x != '.' and int(x) > 1]
    print(f"number of overlapping points: {sum(n_overlapping_coordinates)}")
