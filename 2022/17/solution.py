fileName = "input"

def draw(restingRocks, width):
	top = max(restingRocks.keys())
	bottom = min(restingRocks.keys())
	for layer in range(top,0,-1):
		print('|',end='')
		for i in range(width):
			if i in restingRocks[layer]:
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
				if (testPosition[0] + section[0]) in restingRocks[testPosition[1] + section[1] - 1]:
					return False
		return True
	if movement == '>':
		for section in testShape:
			if (testPosition[0] + section[0] + 1) >= width:
				return False
			if testPosition[1] + section[1] in restingRocks:
				if (testPosition[0] + section[0] + 1) in restingRocks[testPosition[1] + section[1]]:
					return False
		return True
	if movement == '<':
		for section in testShape:
			if (testPosition[0] + section[0] - 1) < 0:
				return False
			if testPosition[1] + section[1] in restingRocks:
				if (testPosition[0] + section[0] - 1) in restingRocks[testPosition[1] + section[1]]:
					return False
		return True
	
def genJets(jets):
	i = 0
	while True:
		yield jets[i]
		i += 1
		if i >= len(jets):
			i = 0
		
jets = list()

fallenRock = 0

width = 7

targetRocks = 2022

highestRock = 0

restingRocks = dict()

shape = {
	0:[(0,0),(1,0),(2,0),(3,0)],
	1:[(0,1),(1,0),(1,1),(1,2),(2,1)],
	2:[(0,0),(1,0),(2,0),(2,1),(2,2)],
	3:[(0,0),(0,1),(0,2),(0,3)],
	4:[(0,0),(0,1),(1,0),(1,1)]
}

with open(file=fileName, mode="r") as f:
	jets = list(f.readline().strip())

jetGenerator = genJets(jets)

currentPosition = [2, highestRock+4]

while fallenRock < targetRocks:
	currentShape = shape[fallenRock%5]
	
	movement = next(jetGenerator)
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
	for section in currentShape:
		if currentPosition[1] + section[1] not in restingRocks:
			restingRocks[currentPosition[1] + section[1]] = list()
			highestRock = max(currentPosition[1] + section[1], highestRock)
		restingRocks[currentPosition[1] + section[1]].append(currentPosition[0] + section[0])
	# draw(restingRocks, width)
	fallenRock += 1
	currentPosition = [2, highestRock+4]
print(highestRock)
		
