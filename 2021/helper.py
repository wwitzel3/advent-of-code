def read_input(filename):
    with open(filename) as f:
        return [l.strip() for l in f.readlines()]

def run(fn, inputf):
    input = read_input(inputf)
    return fn(input)