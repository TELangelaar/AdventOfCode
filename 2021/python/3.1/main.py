
def read_diagnostics_report(name: str) -> tuple:
    with open(name, 'r') as file:
        binary_list = file.read().split()

        result_dict = {}
        for binary_number in binary_list:
            for i in range(0, len(binary_number)):
                if str(i) not in result_dict:
                    result_dict[str(i)] = {'0': 0, '1': 0}
                result_dict[str(i)][binary_number[i]] += 1

        gamma: str = ''
        epsilon: str = ''
        for value in result_dict.values():
            if value['0'] > value['1']:
                gamma = gamma + '0'
                epsilon = epsilon + '1'
            else:
                gamma = gamma + '1'
                epsilon = epsilon + '0'

        return int(gamma, 2), int(epsilon, 2)

if __name__ == '__main__':
    gamma, epsilon = read_diagnostics_report('Day3/1.1/input.txt')
    result = gamma * epsilon
    print(result)
