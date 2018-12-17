import sys

def get_frequency(path):
    frequency = 0
    check = set()
    while frequency not in check:
        check.add(frequency)
        with open(path) as fh:
            for line in fh:
                check.add(frequency)
                sign = line[0]
                value = int(line[1:])
                if sign == '+':
                    frequency += value
                elif sign == '-':
                    frequency -= value
                else:
                    print("ERROR: We encountered an input that doesn't have a +/- as the first char: {}".format(sign))
                    sys.exit(1)
                if frequency in check:
                    return frequency
    return frequency

def main():
    print("First repeated frequency: {}".format(get_frequency("input.txt")))

if __name__ == '__main__':
    main()


