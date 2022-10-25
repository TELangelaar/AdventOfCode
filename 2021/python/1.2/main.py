# This is a sample Python script.

# Press Shift+F10 to execute it or replace it with your code.
# Press Double Shift to search everywhere for classes, files, tool windows, actions, and settings.

def find_larger_than_previous_measurements(name: str) -> int:
    with open(name, 'r') as file:
        measurements = file.read().split()

        larger = 0
        last = -1
        for i in range(0, len(measurements)-2):
            sum_of_3 = int(measurements[i]) + int(measurements[i+1]) + int(measurements[i+2])
            if last == -1:
                last = sum_of_3
                print(str(sum_of_3) + "(N/A - no previous measurement)")
                continue

            if sum_of_3 > last:
                # print("measurement was larger than the previous one!")
                # print(f"{measurement} > {last}")
                larger += 1
                print(str(sum_of_3) + "(INCREASED)")
            else:
                print(str(sum_of_3))
            last = sum_of_3

    return larger


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    answer = find_larger_than_previous_measurements('Day1/1.2/input.txt')
    print(answer)
