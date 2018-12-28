
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
    with open("output.txt") as fh:
        reduced_polymer = fh.readlines()[0].rstrip('\n')
    polymers = set(reduced_polymer.lower())
    fixed_reduction = None

    # Here we just brute force check for the reduced polymer that is the smallest
    for p in polymers:
        filtered_polymer = ''.join(list(filter(lambda c: c.lower() != p, reduced_polymer)))
        reduction_try = reduce_polymer(filtered_polymer)
        if fixed_reduction is None or len(reduction_try) < len(fixed_reduction):
            fixed_reduction = reduction_try
    print("Number of units in the fixed reduced polymer: {}".format(len(fixed_reduction)))

if __name__ == '__main__':
    main()

