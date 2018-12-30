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

def construct_grid(coords):
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
    grid = [ [0] * max_x for i in range(max_y) ]

    return grid

def fill_grid(grid, coords):
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            min_dist = len(grid) + len(grid[0])
            min_point = None
            mins = []
            for point in coords:
                dist = get_manhattan_dist((col, row), coords[point])
                if dist <= min_dist:
                    min_dist = dist
                    min_point = point
                    mins.append(min_dist)
            cmins = Counter(mins)
            if cmins[min_dist] > 1:
                grid[row][col] = -1
            else:
                grid[row][col] = min_point

def get_manhattan_dist(start, end):
    output = abs(start[0] - end[0]) + abs(start[1] - end[1])
    return output

def get_largest_area(grid):
    # Any point that is at the edge of the grid will be infinite so we create a
    # set of those point which we will ignore when getting the largest area
    infinite_set = set()
    for i in range(len(grid[0])):
        infinite_set.add(grid[0][i])
        infinite_set.add(grid[len(grid)-1][i])
    for j in range(len(grid)):
        infinite_set.add(grid[j][0])
        infinite_set.add(grid[j][len(grid[0])-1])

    count = Counter()
    for row in grid:
        count += Counter(row)

    largest_area = 0
    for point in count:
        if point != -1 and point not in infinite_set and count[point] > largest_area:
            largest_area = count[point]

    return largest_area


def main():
    coords = load_coordinates()
    grid = construct_grid(coords)
    fill_grid(grid, coords)
    largest_area = get_largest_area(grid)
    print("Size of the largest area: {}".format(largest_area))

if __name__ == '__main__':
    main()

