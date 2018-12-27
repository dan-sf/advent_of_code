
# Recursion doesn't work here due to the size of the input
def reduce_polymer_recursive(polymer):
    for i in range(len(polymer) - 1):
        if polymer[i] == polymer[i+1]:
            continue
        if polymer[i].lower() == polymer[i+1].lower():
            return reduce_polymer(polymer[:i] + polymer[i+2:])
    return polymer

# Fist iterative attempt, correct but slow
def loop_it_slow(polymer):
    for i in range(len(polymer) - 1):
        if polymer[i] == polymer[i+1]:
            continue
        if polymer[i].lower() == polymer[i+1].lower():
            return polymer[:i] + polymer[i+2:], False
    return polymer, True

# Second iterative attempt using an index cache which is faster
def loop_it(polymer):
    output = ''
    removal_index = set()
    for i in range(len(polymer) - 1):
        if polymer[i] == polymer[i+1]:
            continue
        if i not in removal_index and polymer[i].lower() == polymer[i+1].lower():
            removal_index.add(i)
            removal_index.add(i+1)
    if removal_index:
        for i, p in enumerate(polymer):
            if i not in removal_index:
                output += p
        return output, False
    else:
        return polymer, True

def reduce_polymer(polymer):
    is_reduced = False
    while not is_reduced:
        polymer, is_reduced = loop_it(polymer)
        if is_reduced:
            return polymer

def main():
    with open("input.txt") as fh:
        polymer = fh.readlines()[0].rstrip('\n')
    print("Number of units in the reduced polymer: {}".format(len(reduce_polymer(polymer))))

if __name__ == '__main__':
    main()

