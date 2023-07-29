def isVisible(position, treeMap):
    line = position[0]
    column = position[1]
    target = treeMap[line][column]
    if all(map(lambda x: x < target, treeMap[line][:column])):
        return True
    if all(map(lambda x: x < target, treeMap[line][column+1:])):
        return True
    if all(map(lambda x: x < target, [x[column] for x in treeMap[:line]])):
        return True
    if all(map(lambda x: x < target, [x[column] for x in treeMap[line+1:]])):
        return True
    return False

def getScore(position, treeMap):
    line = position[0]
    column = position[1]
    target = treeMap[line][column]
    scoreFactors = []
    
    searchRange = treeMap[line][:column].copy()
    searchRange.reverse()
    if len(searchRange) == 0:
        return 0
    currentFactor = 0
    for treeHeight in searchRange:
        currentFactor += 1
        if treeHeight >= target:
            break
    scoreFactors.append(currentFactor)
    currentFactor = 1
 
    searchRange = treeMap[line][column+1:].copy()
    if len(searchRange) == 0:
        return 0
    currentFactor = 0
    for treeHeight in searchRange:
        currentFactor += 1
        if treeHeight >= target:
            break
    scoreFactors.append(currentFactor)
    currentFactor = 1
    
    searchRange = [x[column] for x in treeMap[:line]].copy()
    searchRange.reverse()
    if len(searchRange) == 0:
        return 0
    currentFactor = 0
    for treeHeight in searchRange:
        currentFactor += 1
        if treeHeight >= target:
            break
    scoreFactors.append(currentFactor)
    currentFactor = 1
    
    searchRange = [x[column] for x in treeMap[line+1:]].copy()
    if len(searchRange) == 0:
        return 0
    currentFactor = 0
    for treeHeight in searchRange:
        currentFactor += 1
        if treeHeight >= target:
            break
    scoreFactors.append(currentFactor)
    currentFactor = 1
    
    print(scoreFactors)
    score = 1
    for factor in scoreFactors:
        score *= factor
    return score

fileName = "adventofcode.com_2022_day_8_input.txt"
treeMap = []
with open(file=fileName, mode="r") as f:
    for line in f:
        treeMap.append(list(map(int, list(line)[:-1])))
count = 0
for i in range(len(treeMap)):
    for j in range(len(treeMap[0])):
        count += int(isVisible([i,j], treeMap))
print(count)

scores = []
for i in range(len(treeMap)):
    for j in range(len(treeMap[0])):
        scores.append(getScore([i,j], treeMap))
        if i == 1 and j == 7:
            print(scores[-1])
print(max(scores))