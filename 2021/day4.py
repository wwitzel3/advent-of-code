from helper import run


def winning_checks():
    winning = []
    for n in range(5):
        winning.append([(n, 0), (n, 1), (n, 2), (n, 3), (n, 4)])
        winning.append([(0, n), (1, n), (2, n), (3, n), (4, n)])
    return winning


CHECKS = winning_checks()


def check(card):
    for check in CHECKS:
        win = True
        for x, y in check:
            if card[x][y] != 'x':
                win = False
                break
        if win:
            return True
    return False


def setup(input):
    drawing = input[0].split(',')
    print(drawing)

    cards = []
    current_board = []
    for line in input[2:]:
        if line == '':
            cards.append(current_board)
            current_board = []
            continue
        current_board.append([n for n in line.strip().split(' ') if n != ''])
    if len(current_board) > 0:
        cards.append(current_board)

    winners = {str(k): 0 for k in range(len(cards))}

    for n in drawing:
        for i, card in enumerate(cards):
            for x, row in enumerate(card):
                for y, item in enumerate(row):
                    if item == n:
                        card[x][y] = 'x'
            if check(card):
                print("Winner", card)
                values = []
                for row in card:
                    for v in row:
                        if v != 'x':
                            values.append(int(v))
                winners[str(i)] = 1
            if 0 not in winners.values():
                return sum(values) * int(n)
    return winners


def main():
    print(run(setup, 'day4.puzzle.input'))


if __name__ == '__main__':
    main()
