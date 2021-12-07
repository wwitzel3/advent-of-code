from helper import run

diagram = []
numbers = []
count = 0

def update_diagram(left, right, diagram):
    global count

    x1,y1 = left
    x2,y2 = right

    horiz = x1 == x2
    vert = y1 == y2
    if horiz:
        start = y1 if y1 < y2 else y2
        end = y1+1 if y1 > y2 else y2+1
        incr = 1 if start < end else -1
        for i in range(start, end):
            if diagram[i][x1] == '.':
                diagram[i][x1] = 1
            else:
                if diagram[i][x1] == 1:
                    count += 1
                diagram[i][x1] += 1
        return
    if vert:
        start = x1 if x1 < x2 else x2
        end = x1+1 if x1 > x2 else x2+1
        incr = 1 if start < end else -1
        for i in range(start, end):
            if diagram[y1][i] == '.':
                diagram[y1][i] = 1
            else:
                if diagram[y1][i] == 1:
                    count += 1
                diagram[y1][i] += 1
        return

    x_range = range(x1, x2-1, -1)
    if x1 < x2:
        x_range = range(x1, x2+1)

    y_range = range(y1, y2-1, -1)
    if y1 < y2:
        y_range = range(y1, y2+1)

    for x,y in zip(x_range, y_range):
        if diagram[y][x] == '.':
            diagram[y][x] = 1
        else:
            if diagram[y][x] == 1:
                count += 1
            diagram[y][x] += 1


def setup(input):
    moves = []
    for i in input:
        left, right = i.split(' -> ')
        left, right = left.split(','), right.split(',')

        numbers.append(int(left[0]))
        numbers.append(int(left[1]))
        numbers.append(int(right[0]))
        numbers.append(int(right[1]))

        moves.append(
            [
                (int(left[0]), int(left[1])),
                (int(right[0]), int(right[1])),
            ],
        )

    diagram = []
    for _ in range(max(numbers) + 1):
        diagram.append(['.'] * (max(numbers) + 1))

    for move in moves:
        left, right = move
        update_diagram(left, right, diagram)

    return count

def main():
   print(run(setup, 'day5.puzzle.input'))

if __name__ == '__main__':
    main()