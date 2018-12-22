
def main():
    fabric = [[0] * 1000 for i in range(1000)]

    with open("input.txt") as fh:
        for line in fh:
            x, y = [int(i) for i in line.split(' ')[2][:-1].split(',')]
            width, height = [int(i) for i in line.split(' ')[-1].split('x')]
            for i in range(height):
                for j in range(width):
                    fabric[i+y][j+x] += 1

    with open("input.txt") as fh:
        for line in fh:
            x, y = [int(i) for i in line.split(' ')[2][:-1].split(',')]
            width, height = [int(i) for i in line.split(' ')[-1].split('x')]
            square = [fabric[i+y][j+x] for i in range(height) for j in range(width)]
            if len(square) == sum(square):
                print("Non-overlapping claim: {}".format(line.rstrip('\n')))
                break

if __name__ == '__main__':
    main()


