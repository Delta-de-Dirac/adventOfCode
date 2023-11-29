fileName = "input"


def printMap(valleyMap):
    for line in valleyMap:
        print(line)


def testValidMinute(width, height, line, column, badMinutesHorizontal,
                    badMinutesVertical, minute):
    if line == -1 and column == 0:
        return True
    if line == height and column == width-1:
        return True
    if line < 0:
        return False
    if column < 0:
        return False
    if line >= height:
        return False
    if column >= width:
        return False
    # test horizontal
    if minute % width in badMinutesHorizontal[(line, column)]:
        return False
    # test vertical
    if minute % height in badMinutesVertical[(line, column)]:
        return False
    return True


def printValidMinute(width, height, badMinutesHorizontal,
                     badMinutesVertical, minute):
    for line in range(height):
        for column in range(width):
            if testValidMinute(width, height, line, column,
                               badMinutesHorizontal, badMinutesVertical,
                               minute):
                print('#', end='')
                continue
            print('.', end='')
        print('', end='\n')



inputData = []
with open(file=fileName, mode="r") as f:
    inputData=f.readlines()

valleyMap = inputData[1:-1]
for index, item in enumerate(valleyMap):
    valleyMap[index] = item.strip()[1:-1]

badMinutesHorizontal = dict()
badMinutesVertical = dict()


width = len(valleyMap[0])
height = len(valleyMap)

for line in range(height):
    for column in range(width):
        currentBadMinutesHorizontal = set()
        currentBadMinutesVertical = set()
        # check x
        for i in range(width):
            if valleyMap[line][i] == '>':
                currentBadMinutesHorizontal.add((column-i) % width)
            if valleyMap[line][i] == '<':
                currentBadMinutesHorizontal.add((i-column) % width)
        # check y
        for i in range(height):
            if valleyMap[i][column] == 'v':
                currentBadMinutesVertical.add((line-i) % height)
            if valleyMap[i][column] == '^':
                currentBadMinutesVertical.add((i-line) % height)
        badMinutesHorizontal[(line, column)] = currentBadMinutesHorizontal
        badMinutesVertical[(line, column)] = currentBadMinutesVertical

possiblePositions = {(-1, 0)}

time = 0

while True:
    newPossiblePositions = set()

    for position in possiblePositions:
        line, column = position
        # wait
        if testValidMinute(width, height, line, column, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line, column))
        # N
        if testValidMinute(width, height, line-1, column, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line-1, column))
        # S
        if testValidMinute(width, height, line+1, column, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line+1, column))
        # W
        if testValidMinute(width, height, line, column-1, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line, column-1))
        # E
        if testValidMinute(width, height, line, column+1, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line, column+1))
    if (height, width-1) in newPossiblePositions:
        print(time+1)
        break
    time = time+1
    possiblePositions = newPossiblePositions
    continue

# part 2

possiblePositions = {(-1, 0)}

time = 0


stage = 0

# stage 0 -> go to end
# stage 1 -> go back
# stage 2 -> go to end again

while True:
    newPossiblePositions = set()

    for position in possiblePositions:
        line, column = position
        # wait
        if testValidMinute(width, height, line, column, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line, column))
        # N
        if testValidMinute(width, height, line-1, column, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line-1, column))
        # S
        if testValidMinute(width, height, line+1, column, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line+1, column))
        # W
        if testValidMinute(width, height, line, column-1, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line, column-1))
        # E
        if testValidMinute(width, height, line, column+1, badMinutesHorizontal,
                           badMinutesVertical, time+1):
            newPossiblePositions.add((line, column+1))
    if stage == 0:
        if (height, width-1) in newPossiblePositions:
            time = time + 1
            possiblePositions = {(height, width-1)}
            stage = 1
            continue
    if stage == 1:
        if (-1, 0) in newPossiblePositions:
            time = time + 1
            possiblePositions = {(-1, 0)}
            stage = 2
            continue
    if stage == 2:
        if (height, width-1) in newPossiblePositions:
            print(time + 1)
            break
    time = time+1
    possiblePositions = newPossiblePositions
    continue
