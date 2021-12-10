from helper import run
from collections import defaultdict
import pprint

ILLEGAL_VALUE = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}
AUTO_VALUE = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}

MAP = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>',
}
REVERSE = {k: v for v, k in MAP.items()}


class Bracket:
    def __init__(self, char, depth, closing=None):
        self.depth = depth
        self.char = char
        self.closing = closing

    def __repr__(self):
        return f"Bracket<{self.char}, {self.depth}, {self.closing}>"


def find_pair(line):
    ref = defaultdict(list)
    depth = 0
    for c in line:
        if c in MAP.keys():
            bracket = Bracket(c, depth)
            ref[str(depth)].append(bracket)
            depth += 1
        if c in MAP.values():
            depth -= 1
            bracket = Bracket(c, depth)
            ref[str(depth)].append(bracket)

    illegal_chars = []
    for k, v in ref.items():
        if len(v) == 1:
            continue
        for idx in range(len(v)):
            open = v[idx].char
            if open in MAP.values():
                continue
            if idx == len(v) - 1:
                break
            close = v[idx+1].char
            if MAP[open] == close:
                continue
            print("mismatch?", open, close, v[idx].depth)
            illegal_chars.append(close)

    illegal_total = sum([ILLEGAL_VALUE[c] for c in illegal_chars])

    closing = 0
    if illegal_total == 0:
        print("valid, but incomplete")
        for k, v in reversed(ref.items()):
            if len(v) % 2 == 0:
                continue
            key = MAP[v[-1].char]
            point = AUTO_VALUE[key]
            total = closing * 5
            closing = total + point

    return closing, illegal_total


def move(input):
    illegal = []
    closing = []
    for line in input:
        closing_total, illegal_total = find_pair(line)
        illegal.append(illegal_total)
        closing.append(closing_total)
    closing = sorted([c for c in closing if c != 0])
    return closing[int((len(closing) - 1) / 2)], sum(illegal)


def main():
    print(run(move, 'day10.puzzle.input'))


if __name__ == '__main__':
    main()
