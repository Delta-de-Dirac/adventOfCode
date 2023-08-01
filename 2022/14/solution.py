fileName = "input"

rock = set()
sand = set()

sandOrigin = (500,0)

def dropSand(rock, sand, sandOrigin, xMin, xMax, yMin, yMax):
    sandPos = list(sandOrigin)
    while True:
        if not (xMin <= sandPos[0] <= xMax):
            return False
        if not (yMin <= sandPos[1] <= yMax):
            return False
        if (sandPos[0],sandPos[1]+1) not in rock|sand:
            sandPos[1] += 1
            continue
        if (sandPos[0]-1,sandPos[1]+1) not in rock|sand:
            sandPos[0] -= 1
            sandPos[1] += 1
            continue
        if (sandPos[0]+1,sandPos[1]+1) not in rock|sand:
            sandPos[0] += 1
            sandPos[1] += 1
            continue
        break
    sand.add(tuple(sandPos))
    return True

def bfsDropSandFloor(rock, sand, sandOrigin, yFloor):
    sandPosToVisit = [sandOrigin]
    while len(sandPosToVisit) > 0:
        for sandPos in sandPosToVisit:
            while sandPos in sandPosToVisit:
                sandPosToVisit.remove(sandPos)
            if sandPos not in rock|sand:
                sand.add(sandPos)
                if sandPos[1]+1 < yFloor:
                    sandPosToVisit.append( (sandPos[0] - 1, sandPos[1]+1) )
                    sandPosToVisit.append( (sandPos[0], sandPos[1]+1) )
                    sandPosToVisit.append( (sandPos[0] + 1, sandPos[1]+1) )


# Brute force solution to part 2
def dropSandFloor(rock, sand, sandOrigin, yFloor):
    sandPos = list(sandOrigin)
    while True:
        if sandPos[1]+1 == yFloor:
            break
        if (sandPos[0],sandPos[1]+1) not in rock|sand:
            sandPos[1] += 1
            continue
        if (sandPos[0]-1,sandPos[1]+1) not in rock|sand:
            sandPos[0] -= 1
            sandPos[1] += 1
            continue
        if (sandPos[0]+1,sandPos[1]+1) not in rock|sand:
            sandPos[0] += 1
            sandPos[1] += 1
            continue
        break
    sand.add(tuple(sandPos))
    if tuple(sandPos) == sandOrigin:
        return False
    return True

def draw(rock, sand, sandOrigin, xMin, xMax, yMin, yMax):
    for i in range(yMin,yMax+1):
        for j in range(xMin,xMax+1):
            if (j, i) in rock:
                print("#",end='')
                continue
            if (j, i) in sand:
                print("o",end='')
                continue
            if (j, i) == sandOrigin:
                print("+",end='')
                continue
            print('.',end='')
        print()

with open(file=fileName, mode="r") as f:
    for line in f:
        rockPath = line.strip().split(' -> ')
        rockPath = list(map(lambda x: tuple(map(int, x.split(','))), rockPath))
         
        for i in range(len(rockPath) - 1):
            s = rockPath[i]
            e = rockPath[i+1]
            xRange = range(s[0], e[0]+1) if s[0] <= e[0] else range(e[0], s[0]+1)
            yRange = range(s[1], e[1]+1) if s[1] <= e[1] else range(e[1], s[1]+1)
            rock |= {(i, j) for i in xRange for j in yRange}

xMax = max([i[0] for i in rock|{sandOrigin}])
xMin = min([i[0] for i in rock|{sandOrigin}])
yMax = max([i[1] for i in rock|{sandOrigin}])
yMin = min([i[1] for i in rock|{sandOrigin}])

while(dropSand(rock, sand, sandOrigin, xMin, xMax, yMin, yMax)):
    pass

# draw(rock, sand, sandOrigin, xMin, xMax, yMin, yMax)
print(len(sand))

# part 2 VERY LONG TIME TO EXECUTE, NEED IMPLEMENT ALTERNATIVE SOLUTION FOR THIS CASE
sand = set()

yFloor = 2 + yMax

# while(dropSandFloor(rock, sand, sandOrigin, yFloor)):
#     pass

bfsDropSandFloor(rock, sand, sandOrigin, yFloor)

# draw(rock, sand, sandOrigin, xMin, xMax, yMin, yMax)

print(len(sand))
