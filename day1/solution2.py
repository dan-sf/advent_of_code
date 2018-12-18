import sys

def get_frequency(path):
    check = set()
    frequency = 0
    while frequency not in check:
        with open(path) as fh:
            for line in fh:
                check.add(frequency)
                frequency += int(line)
                if frequency in check:
                    return frequency
    return frequency

def main():
    print("First repeated frequency: {}".format(get_frequency("input.txt")))

if __name__ == '__main__':
    main()


