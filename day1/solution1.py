import sys

def main():
    frequency = 0
    with open("input.txt") as fh:
        for line in fh:
            frequency += int(line)
    print("Final frequency: {}".format(frequency))

if __name__ == '__main__':
    main()

