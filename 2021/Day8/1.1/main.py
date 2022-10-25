from typing import List


def parse_input(file_name: str) -> List[int]:
    with open(file_name, 'r') as file:
        co = file.read().splitlines()
        co = [item.split('|') for item in co]
        co = [(item[0].strip().split(' '), item[1].strip().split(' ')) for item in co]
        return co


def get_answer(input):
    search_param = {1: 2, 4: 4, 7: 3, 8: 7}
    result = 0
    for tup in input:
        for item in tup[1]:
            if len(item) in search_param.values():
                result += 1
    return result


if __name__ == "__main__":
    input = parse_input('Day8/1.1/input.txt')
    answer = get_answer(input)
    print(answer)
