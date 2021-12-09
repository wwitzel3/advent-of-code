from helper import run


def move(input):
    pos = [int(s) for s in input[0].split(',')]
    mx = max(pos)

    cheapest = []
    print("Checking", mx, "combinations")
    for i in range(mx+1):
        cost = 0
        for p in [j for j in pos if j != i]:
            val = abs(p - i)
            # cost += val
            cost += sum(range(1, val+1))
        cheapest.append(cost)
    return min(cheapest)


def main():
    print(run(move, 'day7.puzzle.input'))


if __name__ == '__main__':
    main()
