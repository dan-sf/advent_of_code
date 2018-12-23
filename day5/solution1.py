
# Convert this to an iterative solution
def reduce_polymer(polymer):
    for i in range(len(polymer) - 1):
        if polymer[i] == polymer[i+1]:
            continue
        if polymer[i].lower() == polymer[i+1].lower():
            return reduce_polymer(polymer[:i] + polymer[i+2:])
    return polymer

def main():
    with open("input.txt") as fh:
        polymer = fh.readlines()[0].rstrip('\n')
    # Recursion doesn't work here due to the size of the input
    print("Reduced polymer: {}".format(reduce_polymer(polymer)))

if __name__ == '__main__':
    main()

