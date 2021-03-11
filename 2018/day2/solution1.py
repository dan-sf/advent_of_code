from collections import Counter

def main():
    two = 0
    three = 0
    with open("input.txt") as fh:
        for line in fh:
            line = line.rstrip('\n')
            c = Counter(line)
            if 2 in c.values():
                two += 1
            if 3 in c.values():
                three += 1
    print("Input checksum: {}".format(two*three))

if __name__ == '__main__':
    main()

