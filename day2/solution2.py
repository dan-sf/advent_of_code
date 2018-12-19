from collections import Counter

def get_common_chars(box_one, box_two):
    output = ''
    for a, b in zip(box_one, box_two):
        if a == b:
            output += a
    return output

def find_common_id():
    with open("input.txt") as fh:
        box_ids = fh.readlines()
        box_ids.sort()

        for i in range(len(box_ids)):
            diff = 0
            if i != len(box_ids) - 1:
                for a, b in zip(box_ids[i], box_ids[i+1]):
                    if a != b:
                        diff += 1
                if diff == 1:
                    return get_common_chars(box_ids[i], box_ids[i+1])

def main():
    print("Common letters in the box ids: {}".format(find_common_id()))

if __name__ == '__main__':
    main()

