from helper import run

up = -1
left = -1
right = 1
down = 1


def move(input):
    heightmap = [[int(c) for c in hm] for hm in input]

    height = len(heightmap) - 1
    levels = []
    for x, row in enumerate(heightmap):
        width = len(row) - 1
        for y, col in enumerate(row):
            if x == 0:
                if y == 0:
                    cur = heightmap[x][y]
                    right = heightmap[x][y+1]
                    down = heightmap[x+1][y]
                    if cur < right and cur < down:
                        levels.append(cur)
                elif y < width:
                    cur = heightmap[x][y]
                    left = heightmap[x][y-1]
                    right = heightmap[x][y+1]
                    down = heightmap[x+1][y]
                    if cur < right and cur < left and cur < down:
                        levels.append(cur)
                else:
                    cur = heightmap[x][y]
                    left = heightmap[x][y-1]
                    down = heightmap[x+1][y]
                    if cur < left and cur < down:
                        levels.append(cur)
            elif x < height:
                if y == 0:
                    cur = heightmap[x][y]
                    right = heightmap[x][y+1]
                    down = heightmap[x+1][y]
                    up = heightmap[x-1][y]
                    if cur < right and cur < down and cur < up:
                        levels.append(cur)
                elif y < width:
                    cur = heightmap[x][y]
                    left = heightmap[x][y-1]
                    right = heightmap[x][y+1]
                    down = heightmap[x+1][y]
                    up = heightmap[x-1][y]
                    if cur < right and cur < left and cur < down and cur < up:
                        levels.append(cur)
                else:
                    cur = heightmap[x][y]
                    left = heightmap[x][y-1]
                    down = heightmap[x+1][y]
                    up = heightmap[x-1][y]
                    if cur < left and cur < down and cur < up:
                        levels.append(cur)
            else:
                if y == 0:
                    cur = heightmap[x][y]
                    right = heightmap[x][y+1]
                    up = heightmap[x-1][y]
                    if cur < right and cur < up:
                        levels.append(cur)
                elif y < width:
                    cur = heightmap[x][y]
                    left = heightmap[x][y-1]
                    right = heightmap[x][y+1]
                    up = heightmap[x-1][y]
                    if cur < right and cur < left and cur < up:
                        levels.append(cur)
                else:
                    cur = heightmap[x][y]
                    left = heightmap[x][y-1]
                    up = heightmap[x-1][y]
                    if cur < left and cur < up:
                        levels.append(cur)

    return sum(levels) + len(levels)


def main():
    print(run(move, 'day9.puzzle.input'))


if __name__ == '__main__':
    main()
