fileName = "input"

elvesMap = list()

nextDirection = {
    'N': 'S',
    'S': 'W',
    'W': 'E',
    'E': 'N'
    }

def scoreMap(elvesMap):
    #remove padding
    while '#' not in elvesMap[0]:
        elvesMap.pop(0)
    while '#' not in elvesMap[-1]:
        elvesMap.pop(-1)
    while '#' not in [elvesMap[x][-1] for x in range(len(elvesMap))]:
        for line in elvesMap:
            line.pop(-1)
    while '#' not in [elvesMap[x][0] for x in range(len(elvesMap))]:
        for line in elvesMap:
            line.pop(0)
    count = 0
    for line in elvesMap:
        for char in line:
            if char == ".":
                count += 1
    return count


def makeTurn(elvesMap, initialDirection):
    #padding
    if '#' in elvesMap[0]:
        elvesMap.insert(0, ['.']*len(elvesMap[0]))
    if '#' in elvesMap[-1]:
        elvesMap.append(['.']*len(elvesMap[0]))
    if '#' in [elvesMap[x][-1] for x in range(len(elvesMap))]:
        for line in elvesMap:
            line.append('.')
    if '#' in [elvesMap[x][0] for x in range(len(elvesMap))]:
        for line in elvesMap:
            line.insert(0, '.')

    #move
    pendingMoves = dict()
    for lineIndex, line in enumerate(elvesMap):
        for charIndex, char in enumerate(line):
            if char == '#':
                if (elvesMap[lineIndex-1][charIndex-1] ==
                   elvesMap[lineIndex-1][charIndex] ==
                   elvesMap[lineIndex-1][charIndex+1] ==
                   elvesMap[lineIndex][charIndex-1] ==
                   elvesMap[lineIndex][charIndex+1] ==
                   elvesMap[lineIndex+1][charIndex-1] ==
                   elvesMap[lineIndex+1][charIndex] ==
                   elvesMap[lineIndex+1][charIndex+1] ==
                   '.'):
                    continue
                direction = initialDirection
                while True:
                    if direction == "N":
                        if (elvesMap[lineIndex-1][charIndex-1] ==
                           elvesMap[lineIndex-1][charIndex] ==
                           elvesMap[lineIndex-1][charIndex+1] ==
                           '.'):
                            if (lineIndex-1, charIndex) in pendingMoves:
                                pendingMoves.pop((lineIndex-1, charIndex))
                                break

                            pendingMoves[(lineIndex-1, charIndex)] = (
                                lineIndex, charIndex)
                            break

                    if direction == "S":
                        if (elvesMap[lineIndex+1][charIndex-1] ==
                           elvesMap[lineIndex+1][charIndex] ==
                           elvesMap[lineIndex+1][charIndex+1] ==
                           '.'):
                            if (lineIndex+1, charIndex) in pendingMoves:
                                pendingMoves.pop((lineIndex+1, charIndex))
                                break
                            pendingMoves[(lineIndex+1, charIndex)] = (
                                lineIndex, charIndex)
                            break
                    if direction == "W":
                        if (elvesMap[lineIndex-1][charIndex-1] ==
                           elvesMap[lineIndex][charIndex-1] ==
                           elvesMap[lineIndex+1][charIndex-1] ==
                           '.'):
                            if (lineIndex, charIndex-1) in pendingMoves:
                                pendingMoves.pop((lineIndex, charIndex-1))
                                break
                            pendingMoves[(lineIndex, charIndex-1)] = (
                                lineIndex, charIndex)
                            break
                    if direction == "E":
                        if (elvesMap[lineIndex-1][charIndex+1] ==
                           elvesMap[lineIndex][charIndex+1] ==
                           elvesMap[lineIndex+1][charIndex+1] ==
                           '.'):
                            if (lineIndex, charIndex+1) in pendingMoves:
                                pendingMoves.pop((lineIndex, charIndex+1))
                                break
                            pendingMoves[(lineIndex, charIndex+1)] = (
                                lineIndex, charIndex)
                            break
                    direction = nextDirection[direction]
                    if direction == initialDirection:
                        break
                continue
            continue
    for lineIndex, columnIndex in pendingMoves:
        elvesMap[lineIndex][columnIndex] = '#'
        eraseLine, eraseColumn = pendingMoves[(lineIndex, columnIndex)]
        elvesMap[eraseLine][eraseColumn] = '.'
    if not pendingMoves:
        return True
    return False

def printMap(elvesMap):
    for line in elvesMap:
        for char in line:
            print(char, end='')
        print('',end="\n")

with open(file=fileName, mode="r") as f:
    for line in f:
        line = line.strip()
        elvesMap.append(list(line))

direction = "N"
for i in range(10):
    makeTurn(elvesMap, direction)
    direction = nextDirection[direction]
print(scoreMap(elvesMap))

#part 2

elvesMap = []
with open(file=fileName, mode="r") as f:
    for line in f:
        line = line.strip()
        elvesMap.append(list(line))
direction = "N"
count = 1
while not makeTurn(elvesMap, direction):
    direction = nextDirection[direction]
    count += 1
print(count)



