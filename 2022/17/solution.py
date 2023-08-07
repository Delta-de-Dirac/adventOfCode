fileName = "input"
from matplotlib import pyplot as plt


def getRelevantRegion(highestRock, restingRocks, width, shape):
	depth = getDepth(highestRock, restingRocks, width, shape)
	relevantRegion = list()
	for x in [restingRocks[i] for i in range(highestRock,highestRock - depth,-1)]:
		relevantRegion.append(x)
	return relevantRegion

def getDepth(top, restingRocks, width, shapes):
	validCurrent = set()
	for xPos in range(width):
		if canAnyShapeBeAt((xPos,top), restingRocks, width, shapes):
			validCurrent.add(xPos)
	depth = 0
	while len(validCurrent) > 0 and depth < top:
		depth += 1
		validAbove = validCurrent
		validCurrent = set()
		for xPos in range(width):
			if canAnyShapeBeAt((xPos,top-depth), restingRocks, width, shapes) and xPos in validAbove:
				validCurrent.add(xPos)
		for xPos in range(width):
			if canAnyShapeBeAt((xPos,top-depth), restingRocks, width, shapes) and (xPos + 1 in validCurrent or xPos - 1 in validCurrent):
				validCurrent.add(xPos)
	return depth


def canShapeBeAt(position, restingRocks, width, shape):
	for section in shape:
		if (position[0] + section[0]) >= width:
			return False
		if position[1] + section[1] not in restingRocks:
			continue
		if (restingRocks[position[1] + section[1]]>>(position[0] + section[0]))&0b1 == 0b1:
			return False
	return True

def canAnyShapeBeAt(position, restingRocks, width, shapes):
	return any([canShapeBeAt(position, restingRocks, width, x) for x in shapes])

def draw(restingRocks, width, depth):
	top = max(restingRocks.keys())
	for layer in range(top,top - depth,-1):
		print('|',end='')
		for i in range(width):
			if (restingRocks[layer]>>i)&0b1 == 0b1:
				print('#', end='')
				continue
			print('.', end='')
		print('|')
	print('+' + '-' * width + '+')
	print()

def canMove(testPosition, testShape, restingRocks, width, movement):
	if movement == 'v':
		for section in testShape:
			if (testPosition[1] + section[1] - 1) <= 0:
				return False
			if testPosition[1] + section[1] - 1 in restingRocks:
				if (restingRocks[testPosition[1] + section[1] - 1]>>(testPosition[0] + section[0]))&0b1 == 0b1:
					return False
		return True
	if movement == '>':
		for section in testShape:
			if (testPosition[0] + section[0] + 1) >= width:
				return False
			if testPosition[1] + section[1] in restingRocks:
				if (restingRocks[testPosition[1] + section[1]]>>(testPosition[0] + section[0] + 1))&0b1 == 0b1:
					return False
		return True
	if movement == '<':
		for section in testShape:
			if (testPosition[0] + section[0] - 1) < 0:
				return False
			if testPosition[1] + section[1] in restingRocks:
				if (restingRocks[testPosition[1] + section[1]]>>(testPosition[0] + section[0] - 1))&0b1 == 0b1:
					return False
		return True
	
def genJets(jets):
	i = 0
	while True:
		yield jets[i], i
		i += 1
		if i >= len(jets):
			i = 0
		
jets = list()

fallenRock = 0

width = 7

targetRocks = 1000000000000

highestRock = 0

restingRocks = dict()

shape = [
	((0,0),(1,0),(2,0),(3,0)),
	((0,1),(1,0),(1,1),(1,2),(2,1)),
	((0,0),(1,0),(2,0),(2,1),(2,2)),
	((0,0),(0,1),(0,2),(0,3)),
	((0,0),(0,1),(1,0),(1,1))
]

with open(file=fileName, mode="r") as f:
	jets = list(f.readline().strip())

jetGenerator = genJets(jets)

currentPosition = [2, highestRock+4]

currentShape = shape[fallenRock%5]

knownSituations = dict()

shifted = False

while fallenRock < targetRocks:
	movement, movementIndex = next(jetGenerator)
	currentRegion = tuple(getRelevantRegion(highestRock, restingRocks, width, shape))
	if not shifted:
		if (fallenRock%5,movementIndex,currentRegion) in knownSituations:
			print((fallenRock, highestRock), "I have seen this before at", knownSituations[(fallenRock%5,movementIndex,currentRegion)])
			towerShiftPerRockPeriod = highestRock - knownSituations[(fallenRock%5,movementIndex,currentRegion)][1]
			rockPeriod = fallenRock - knownSituations[(fallenRock%5,movementIndex,currentRegion)][0]
			periodsToAdvance = (targetRocks-fallenRock)//rockPeriod
			totalTowerShift = periodsToAdvance*towerShiftPerRockPeriod
			
			highestRock += totalTowerShift
			fallenRock += periodsToAdvance*rockPeriod
			currentPosition[1] += totalTowerShift
			for key in list(restingRocks.keys()):
				restingRocks[key+totalTowerShift] = restingRocks.pop(key)
			
			print("towerShiftPerRockPeriod:", towerShiftPerRockPeriod)
			print("rockPeriod:", rockPeriod)
			print("set highestRock to:", highestRock)
			print("set fallenRock to:", fallenRock)

			shifted = True
	# move jet
	if canMove(currentPosition, currentShape, restingRocks, width, movement):
		if movement == '>':
			currentPosition[0] += 1
		if movement == '<':
			currentPosition[0] -= 1
	# move down
	if canMove(currentPosition, currentShape, restingRocks, width, 'v'):
		currentPosition[1] -= 1
		continue
	
	if (fallenRock%5,movementIndex,currentRegion) not in knownSituations:
		knownSituations[(
			fallenRock%5,
			movementIndex,
			currentRegion
		)] = (fallenRock, highestRock)
	
	for section in currentShape:
		if currentPosition[1] + section[1] not in restingRocks:
			restingRocks[currentPosition[1] + section[1]] = 0b0
			highestRock = max(currentPosition[1] + section[1], highestRock)
		restingRocks[currentPosition[1] + section[1]] |= 0b1<<(currentPosition[0] + section[0])
	# draw(restingRocks, width)
	
	fallenRock += 1
	currentPosition = [2, highestRock+4]
	currentShape = shape[fallenRock%5]
	
	
	
print(highestRock)

