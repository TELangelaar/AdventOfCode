# This is a sample Python script.

# Press Shift+F10 to execute it or replace it with your code.
# Press Double Shift to search everywhere for classes, files, tool windows, actions, and settings.

def find_larger_than_previous_measurements(name: str) -> int:
    with open(name, 'r') as file:
        measurements = file.read().split()

        larger = 0
        last = -1
        for measurement in measurements:
            measurement = int(measurement)
            if last == -1:
                last = measurement
                print(str(measurement) + "(N/A - no previous measurement)")
                continue

            if measurement > last:
                # print("measurement was larger than the previous one!")
                # print(f"{measurement} > {last}")
                larger += 1
                print(str(measurement) + "(INCREASED)")
            else:
                print(str(measurement))
            last = measurement

    return larger


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    answer = find_larger_than_previous_measurements('Day1/1.1/input.txt')
    print(answer)
