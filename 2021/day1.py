inputs = ""

def main():
    increased = 0
    prev = None
    pings = inputs.split('\n')
    print(len(pings))
    for step in range(len(pings)):
        try:
            input = generate(pings, step)
        except IndexError:
            break
        if prev is not None:
            if int(prev) < int(input):
                increased += 1
        prev = input
    print(increased)

def generate(pings, step):
    first = 0 + step
    second = 1 + step
    third = 2 + step

    v1 = int(pings[first])
    v2 = int(pings[second])
    v3 = int(pings[third])

    return v1 + v2 + v3

if __name__ == '__main__':
    main()
