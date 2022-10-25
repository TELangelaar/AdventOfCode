
def get_coordinates(file_name: str) -> tuple:
    with open(file_name, 'r') as file:

        result = []
        for i in file.readlines():
            tmp = i.split(" ")
            result.append((tmp[0], int(tmp[1])))

        horizontal: int = 0
        vertical: int = 0
        for (direction, amount) in result:
            if direction == 'forward':
                horizontal += amount
            elif direction == 'up':
                vertical -= amount
            else: #direction is down
                vertical += amount

        return horizontal, vertical




if __name__ == '__main__':
    (horizontal, vertical) = get_coordinates('Day2/1.1/input.txt')
    print(f"horizontal position: {horizontal}")
    print(f"vertical position: {vertical}")
    result = horizontal * vertical
    print(f"answer = horizontal x vertical, answer = {result}")
