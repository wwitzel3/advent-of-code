from helper import run


def common(input):
    common = [None] * len(input[0])
    for i in input:
        for p, s in enumerate(i):
            val = 0 if common[p] == None else common[p]
            if int(s) == 1:
                val += 1
            else:
                val -= 1
            common[p] = val
    return common


def lifesupport(input):
    def process(in1, most_common, step=0):
        if len(in1) == 1:
            return in1[0]

        nums1 = common(in1)

        if most_common:
            match = "1" if nums1[step] >= 0 else "0"
        else:
            match = "0" if nums1[step] >= 0 else "1"

        keep = []
        for i in in1:
            if i[step] == match:
                keep.append(i)

        return process(keep, most_common, step+1)

    oxy = int(process(input, True), 2)
    co2 = int(process(input, False), 2)

    print("oxy", oxy)
    print("co2", co2)

    return oxy * co2


def diag(input):
    nums = common(input)

    gamma = ''.join(["1" if i > 0 else "0" for i in nums if i != 0])
    epsilon = ''.join(["0" if i > 0 else "1" for i in nums if i != 0])

    print(gamma)
    print(epsilon)

    print("life support", lifesupport(input))

    gamma = int(gamma, 2)
    epsilon = int(epsilon, 2)
    return gamma * epsilon


def main():
    print(run(diag, 'day3.puzzle.input'))


if __name__ == '__main__':
    main()
