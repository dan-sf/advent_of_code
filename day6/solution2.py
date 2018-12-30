from collections import Counter

def load_coordinates():
    coords = {}
    with open("input.txt") as fh:
        for i, line in enumerate(fh):
            line = line.rstrip('\n')
            x, y = line.split(', ')
            x, y = int(x), int(y)
            coords[i] = x, y
    return coords

def get_max_coords(coords):
    grid = []
    max_x, max_y = 0, 0
    for x, y in coords.values():
        if x > max_x:
            max_x = x
        if y > max_y:
            max_y = y

    # We need to add 1 to the max x and y to account for the origin
    max_x += 1
    max_y += 1

    return max_x, max_y

def get_safe_region_area(max_x, max_y, coords):
    safe_region = 0
    for row in range(max_y):
        for col in range(max_x):
            dist = sum([ get_manhattan_dist((col, row), coords[point]) for point in coords ])
            if dist < 10000:
                safe_region += 1
    return safe_region

def get_manhattan_dist(start, end):
    output = abs(start[0] - end[0]) + abs(start[1] - end[1])
    return output

def main():
    coords = load_coordinates()
    max_x, max_y = get_max_coords(coords)
    safe_region = get_safe_region_area(max_x, max_y, coords)
    print("Size of the safe region: {}".format(safe_region))

if __name__ == '__main__':
    main()

