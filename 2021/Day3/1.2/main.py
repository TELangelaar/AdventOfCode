from typing import List


def read_diagnostics_report(name: str) -> List[str]:
    with open(name, 'r') as file:
        binary_list = file.read().split()

        return binary_list


def find_oxygen_generator_rating(binary_list: List[str], bit_position=0) -> List[str]:
    n_zeros = 0
    n_ones = 0
    for i in range(0, len(binary_list)):
        if binary_list[i][bit_position] == '0':
            n_zeros += 1
        else:
            n_ones += 1

    copy_list = binary_list.copy()
    for j in range(0, len(binary_list)):
        if n_ones >= n_zeros:
            if binary_list[j][bit_position] == '0':
                copy_list.remove(binary_list[j])
        else:
            if binary_list[j][bit_position] == '1':
                copy_list.remove(binary_list[j])

    if len(copy_list) == 1:
        return copy_list
    else:
        return find_oxygen_generator_rating(copy_list, bit_position+1)


def find_scrubber_rating(binary_list: List[str], bit_position=0) -> List[str]:
    n_zeros = 0
    n_ones = 0
    for i in range(0, len(binary_list)):
        if binary_list[i][bit_position] == '0':
            n_zeros += 1
        else:
            n_ones += 1

    copy_list = binary_list.copy()
    for j in range(0, len(binary_list)):
        if n_zeros > n_ones:
            if binary_list[j][bit_position] == '0':
                copy_list.remove(binary_list[j])
        else:
            if binary_list[j][bit_position] == '1':
                copy_list.remove(binary_list[j])

    if len(copy_list) == 1:
        return copy_list
    else:
        return find_scrubber_rating(copy_list, bit_position+1)


if __name__ == '__main__':
    binary_list = read_diagnostics_report('Day3/1.2/input.txt')

    oxygen_generator_rating = find_oxygen_generator_rating(binary_list)
    CO2_scrubber_rating = find_scrubber_rating(binary_list)

    decimal_oxygen = int(oxygen_generator_rating[0], 2)
    decimal_CO2 = int(CO2_scrubber_rating[0], 2)
    result =  decimal_oxygen * decimal_CO2
    print(result)
