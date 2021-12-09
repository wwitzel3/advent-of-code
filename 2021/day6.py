from helper import run

counts = {
    '0': 0,
    '1': 0,
    '2': 0,
    '3': 0,
    '4': 0,
    '5': 0,
    '6': 0,
    '7': 0,
    '8': 0,
}


def simulate(input):
    number = input[0].replace(',', '')

    # establish baseline
    for f in number:
        counts[f] += 1

    for n in range(256):
        advance(counts)

    return counts, sum(counts.values())


def advance(counts):
    # advance
    new_fish, reset_fish = counts['0'], counts['0']

    counts['0'] = counts['1']
    counts['1'] = counts['2']
    counts['2'] = counts['3']
    counts['3'] = counts['4']
    counts['4'] = counts['5']
    counts['5'] = counts['6']
    counts['6'] = counts['7'] + reset_fish
    counts['7'] = counts['8']
    counts['8'] = new_fish


def main():
    print(run(simulate, 'day6.puzzle.input'))


if __name__ == '__main__':
    main()
