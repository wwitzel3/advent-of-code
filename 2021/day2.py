from helper import run


def dive(input):
    horz = 0
    depth = 0
    aim = 0

    for i in input:
        s, n = i.split(" ")
        n = int(n)
        if s == "forward":
            horz += n
            depth += (aim * n)
        elif s == "up":
            aim -= n
        elif s == "down":
            aim += n

    return horz * depth


def main():
    print(run(dive, 'day2.puzzle.input'))


if __name__ == '__main__':
    main()
