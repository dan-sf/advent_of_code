import sys

def main():
    frequency = 0
    with open("input.txt") as fh:
        for line in fh:
            sign = line[0]
            value = int(line[1:])
            if sign == '+':
                frequency += value
            elif sign == '-':
                frequency -= value
            else:
                print("ERROR: We encountered an input that doesn't have a +/- as the first char: {}".format(sign))
                sys.exit(1)
    print("Final frequency: {}".format(frequency))

if __name__ == '__main__':
    main()

