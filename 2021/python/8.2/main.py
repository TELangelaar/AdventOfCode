from typing import List


def parse_input(file_name: str) -> List[int]:
    with open(file_name, 'r') as file:
        co = file.read().splitlines()
        co = [item.split('|') for item in co]
        co = [(item[0].strip().split(' '), item[1].strip().split(' ')) for item in co]
        co = [[["".join(sorted(digit))for digit in digits] for digits in tup] for tup in co]
        return co


def get_answer(input):
    sum_answers = 0
    for tup in input:
        answer = ''

        mapping = dict([(k, '') for k in ['a', 'b', 'c', 'd', 'e', 'f', 'g']])
        segments = dict([(k, []) for k in range(0, 10)])

        # map the easiest ones
        for digits in tup[0]:
            if len(digits) == 2:
                segments[1].append(digits)
            elif len(digits) == 3:
                segments[7].append(digits)
            elif len(digits) == 4:
                segments[4].append(digits)
            elif len(digits) == 5:
                segments[5].append(digits)
            elif len(digits) == 6:
                segments[6].append(digits)
            elif len(digits) == 7:
                segments[8].append(digits)

        # we know these lists always have only one element
        mapping['a'] = [x for x in segments[7][0] if x not in segments[1][0]][0]

        six_copy = segments[6].copy()
        for item in six_copy:
            mapping_g = []
            for char in item:
                if char not in segments[4][0] + mapping['a']:
                    mapping_g.append(char)
            if len(mapping_g) == 1:
                mapping['g'] = mapping_g[0]
                segments[6].remove(item)
                segments[9].append(item)
                break

        six_copy2 = segments[6].copy()
        for item in six_copy2:
            mapping_e = []
            for char in item:
                if char in segments[7][0]:
                    mapping_e.append(char)
                if char not in segments[9][0]:
                    mapping['e'] = char
            if len(mapping_e) == 3:
                segments[6].remove(item)
                segments[0].append(item)
                break

        for digit in segments[0][0]:
            if digit not in segments[6][0]:
                mapping['c'] = digit

        mapping['f'] = [digit for digit in segments[1][0] if digit not in mapping['c']][0]

        mapping_d = []
        five_copy = segments[5].copy()
        for digits in five_copy:
            for digit in digits:
                if digit not in segments[1][0] + mapping['e'] + mapping['a'] + mapping['g']:
                    mapping_d.append(digit)
            if len(mapping_d) == 2:
                for digits2 in five_copy:
                    if digits2 != digits:
                        segments[5].remove(digits2)
                        segments[2].append(digits2)

        for digits in segments[5]:
            for digit in digits:
                if digit not in segments[2]:
                    mapping['b'] = digit

        two_copy = segments[2].copy()
        for digits in two_copy:
            if mapping['e'] not in digits:
                segments[2].remove(digits)
                segments[3].append(digits)
                break

        for item in tup[1]:
            for i in range(0, len(segments)):
                if segments[i][0] == item:
                    answer = answer + str(i)
        sum_answers += int(answer)
    return sum_answers



if __name__ == "__main__":
    input = parse_input('Day8/1.2/input.txt')
    answer = get_answer(input)
    print(answer)
