from typing import List


def parse_input(file_name: str) -> List[int]:
    with open(file_name, 'r') as file:
        start_states = file.read().split(',')
        start_states = [int(x) for x in start_states]
        return start_states


def get_number_of_fish(st, number_of_days):
    for day in range(0, number_of_days):
        for i in range(0, len(st)):
            if st[i] == 0:
                st[i] = 6
                st.append(8)
            else:
                st[i] -= 1
    return len(st)


def get_number_of_fish_optimized(st, number_of_days):
    result = dict([(k, 0) for k in range(0, 9)])
    for i in range(0, len(st)):
        result[st[i]] += 1

    for day in range(0, number_of_days):
        for j in range(len(result) - 1, -1, -1):
            if j == 8:
                previous = result[j]
                result[j] = result[0]
            elif j == 0:
                result[8] = result[j]
                result[6] += result[j]
                result[j] = previous
            else:
                current = result[j]
                result[j] = previous
                previous = current

    return result


if __name__ == "__main__":
    start_timers = parse_input('Day6/1.1/input.txt')
    number_of_fish = get_number_of_fish_optimized(start_timers, 256)
    print(number_of_fish)
    print(sum(number_of_fish.values()))
