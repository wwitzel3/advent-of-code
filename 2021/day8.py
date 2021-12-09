from helper import run

SIGNAL_MAP = {
    '0': ['top', 'uleft', 'uright', 'bleft', 'bright', 'bottom'],
    '1': ['uright', 'bright'],
    '2': ['top', 'uright', 'middle', 'bleft', 'bottom'],
    '3': ['top', 'uright', 'middle', 'bright', 'bottom'],
    '4': ['uleft', 'uright', 'middle', 'bright'],
    '5': ['top', 'uleft', 'middle', 'bright', 'bottom'],
    '6': ['top', 'uleft', 'middle', 'bleft', 'bright', 'bottom'],
    '7': ['top', 'uright', 'bright'],
    '8': ['top', 'uleft', 'uright', 'middle', 'bleft', 'bright', 'bottom'],
    '9': ['top', 'uleft', 'uright', 'middle', 'bright', 'bottom'],
}


def count_easy(line):
    _, output = line.split('|')
    output = output.strip().split(' ')
    count = 0
    for o in output:
        if len(o) in [2, 3, 4, 7]:
            count += 1
    return count


def generate_number(outputs, num_map):
    char_map = {k: v for k, v in zip(num_map.values(), num_map.keys())}

    def get_values(outputs):
        values = ''
        for out in outputs:
            mapping = []
            for c in out:
                mapping.append(char_map[c])

            #print("MAPPING", mapping)
            #print("CHAR_MAP", char_map)
            for k, v in SIGNAL_MAP.items():
                if sorted(mapping) == sorted(v):
                    #print("MATCH", k)
                    values += k
                    break
        return values
    values = get_values(outputs)
    # since we took a WAG for these inputs swap and try again if the length is wrong
    if len(values) < len(outputs):
        bright = num_map['bright']
        num_map['bright'] = num_map['uright']
        num_map['uright'] = bright
        char_map = {k: v for k, v in zip(num_map.values(), num_map.keys())}
        values = get_values(outputs)
    return int(values)


def process_line(line):
    patterns, outputs = line.split('|')
    patterns = patterns.strip().split(' ')
    outputs = outputs.strip().split(' ')

    new_map = {}
    num_patterns = {'1': None, '4': None, '7': None, '8': None}

    total = 0
    for pattern in sorted(patterns, key=len):
        chars = [p for p in pattern]
        if len(pattern) == 2:
            new_map['uright'] = chars
            new_map['bright'] = chars
            num_patterns['1'] = chars
        elif len(pattern) == 3:
            right = new_map['bright'] or new_map['uright'] or None
            if right:
                diff = set(chars).difference(right)
                new_map['top'] = diff.pop()
            else:
                new_map['top'] = chars
            num_patterns['7'] = chars
        elif len(pattern) == 4:
            right = new_map['bright'] or new_map['uright'] or None
            top = new_map['top'] or None
            if right and top:
                combined = []
                combined.extend(right)
                combined.append(top)
                diff = set(chars).difference(combined)
                new_map['middle'] = list(diff)
                new_map['uleft'] = list(diff)
            else:
                new_map['middle'] = chars
            num_patterns['4'] = chars
        elif len(pattern) == 6:
            four_seven = num_patterns['4'] + num_patterns['7']
            if len(set(num_patterns['4']).difference(chars)) == 0:
                # number 9
                diff = set(chars).difference(four_seven)
                new_map['bottom'] = diff.pop()

    for pattern in sorted(patterns, key=len):
        chars = [p for p in pattern]
        if len(pattern) == 5:
            seven_bottom = num_patterns['7'] + [new_map['bottom']]
            diff = set(chars).difference(seven_bottom)
            if len(diff) == 1:
                # number 3
                middle = new_map['middle']
                new_map['middle'] = diff.pop()
                new_map['uleft'] = set(middle).difference(
                    new_map['middle']).pop()
        if len(pattern) == 6:
            if new_map['middle'] not in chars:
                # 0
                values = [new_map['top'], new_map['uleft'],
                          new_map['bottom']] + new_map['uright']
                diff = set(chars).difference(values)
                new_map['bleft'] = diff.pop()
    a, b = new_map['uright']
    new_map['uright'] = a
    new_map['bright'] = b

    return generate_number(outputs, new_map)


def calc(input):
    total = 0
    for line in input:
        total += process_line(line)
    return total


def main():
    print(run(calc, 'day8.puzzle.input'))


if __name__ == '__main__':
    main()
