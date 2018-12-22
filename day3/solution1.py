
def main():
    fabric = [[0] * 1000 for i in range(1000)]

    with open("input.txt") as fh:
        for line in fh:
            x, y = [int(i) for i in line.split(' ')[2][:-1].split(',')]
            width, height = [int(i) for i in line.split(' ')[-1].split('x')]
            for i in range(height):
                for j in range(width):
                    fabric[i+y][j+x] += 1

    total_inches = 0
    for row in fabric:
        for col in row:
            if col > 1:
                total_inches += 1
    print("Total square inches: {}".format(total_inches))

if __name__ == '__main__':
    main()

