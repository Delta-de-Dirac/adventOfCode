fileName = "input"

snafuDict = {
    '=': -2,
    '-': -1,
    '0': 0,
    '1': 1,
    '2': 2
    }

snafuDecrement = {
    '-': '=',
    '0': '-',
    '1': '0',
    '2': '1'
    }


def decrementSnafuStringAtPosition(snafu_string, position):
    """ decrements once snafu string at position"""
    result = list(snafu_string)
    if snafu_string[-(position+1)] not in snafuDecrement.keys():
        print(snafu_string[-(position+1)])
        raise ValueError
    else:
        result[-(position+1)] = snafuDecrement[result[-(position+1)]]
        return ''.join(result)

def snafu2integer(snafu_string):
    """ Converts SNAFU string (type str) into integer (type int) """
    digitCount = len(snafu_string)
    i = digitCount-1
    result = 0
    while i >= 0:
        result += snafuDict[snafu_string[-(i+1)]]*(5**i)
        i -= 1
        continue
    return result

def integer2snafu(integer):
    """ Converts integer (type int) into SNAFU string (type str) """
    # find initial string
    # test '2' -> '22' -> '222' , etc.
    snafu_string = '2'
    while True:
        if snafu2integer(snafu_string) < integer:
            snafu_string = "2" + snafu_string
            continue
        if snafu2integer(snafu_string) >= integer:
            break
    i = len(snafu_string) - 1
    if snafu2integer(snafu_string) == integer:
        return snafu_string
    while i >= 0:
        while snafu_string[-(i+1)] != '=':
            candidate = decrementSnafuStringAtPosition(snafu_string, i)
            if snafu2integer(candidate) == integer:
                return candidate
            if snafu2integer(candidate) > integer:
                snafu_string = candidate
                continue
            if snafu2integer(candidate) < integer:
                i -= 1
                continue
            raise ValueError
        i -= 1
    raise ValueError



with open(file=fileName, mode="r") as f:
    inputData = f.readlines()

fuel_reqs = []
for line in inputData:
    fuel_reqs.append(line.strip())

print("Sum of fuel reqs in decimal:", sum(map(snafu2integer, fuel_reqs)))
print("Sum of fuel reqs in snafu:", integer2snafu(sum(map(snafu2integer, fuel_reqs))))
