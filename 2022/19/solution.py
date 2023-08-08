import math
import time
from matplotlib import pyplot as plt
fileName = "input"

memo = {}

def getMaxGeode(time, production, stock, blueprint):
	if time == 1:
		return stock[3] + production[3]
	if time == 0:
		return stock[3]
	memoKey = (
		time,
		production,
		stock,
		blueprint
	)
	if memoKey in memo:
		return memo[memoKey]
	maxFound = 0
	cantBuildAnything = True
	shouldBuildOre = production[0] < max(blueprint[0][0],blueprint[1][0],blueprint[2][0],blueprint[3][0]) and stock[0] < (time-1)*max(blueprint[0][0],blueprint[1][0],blueprint[2][0],blueprint[3][0])
	shouldBuildClay = production[1] < blueprint[2][1] and stock[1] < (time-1)*blueprint[2][1]
	shouldBuildObsidian = production[1] > 0 and production[2] < blueprint[3][1] and stock[2] < (time-1)*blueprint[3][1]
	shouldBuildGeode = production[2] > 0
	# build ore next 
	if shouldBuildOre:
		timeToResource = max(
			math.ceil((blueprint[0][0] - stock[0])/production[0]), 
			0
		)
		waitTime = timeToResource + 1
		if waitTime < time:
			cantBuildAnything = False
			maxFound = max(
				getMaxGeode(
					time - waitTime,
					(
						production[0] + 1,
						production[1],
						production[2],
						production[3]
					),
					(
						stock[0] + waitTime*production[0] - blueprint[0][0],
						stock[1] + waitTime*production[1],
						stock[2] + waitTime*production[2],
						stock[3] + waitTime*production[3]
					),
					blueprint
				),
				maxFound
			)
	# build clay next 
	if shouldBuildClay:
		timeToResource = max(
			math.ceil((blueprint[1][0] - stock[0])/production[0]), 
			0
		)
		waitTime = timeToResource + 1
		if waitTime < time:
			cantBuildAnything = False
			maxFound = max(
				getMaxGeode(
					time - waitTime,
					(
						production[0],
						production[1] + 1,
						production[2],
						production[3]
					),
					(
						stock[0] + waitTime*production[0] - blueprint[1][0],
						stock[1] + waitTime*production[1],
						stock[2] + waitTime*production[2],
						stock[3] + waitTime*production[3]
					),
					blueprint
				),
				maxFound
			)
	# build obsidian next
	if shouldBuildObsidian:
		timeToResource = max(
			math.ceil((blueprint[2][0] - stock[0])/production[0]),
			math.ceil((blueprint[2][1] - stock[1])/production[1]), 
			0
		)
		waitTime = timeToResource + 1
		if waitTime < time:
			cantBuildAnything = False
			maxFound = max(
				getMaxGeode(
					time - waitTime,
					(
						production[0],
						production[1],
						production[2] + 1,
						production[3]
					),
					(
						stock[0] + waitTime*production[0] - blueprint[2][0],
						stock[1] + waitTime*production[1] - blueprint[2][1],
						stock[2] + waitTime*production[2],
						stock[3] + waitTime*production[3]
					),
					blueprint
				),
				maxFound
			)
	# build geode next
	if shouldBuildGeode:
		timeToResource = max(
			math.ceil((blueprint[3][0] - stock[0])/production[0]),
			math.ceil((blueprint[3][1] - stock[2])/production[2]), 
			0
		)
		waitTime = timeToResource + 1
		if waitTime < time:
			cantBuildAnything = False
			maxFound = max(
				getMaxGeode(
					time - waitTime,
					(
						production[0],
						production[1],
						production[2],
						production[3] + 1
					),
					(
						stock[0] + waitTime*production[0] - blueprint[3][0],
						stock[1] + waitTime*production[1],
						stock[2] + waitTime*production[2] - blueprint[3][1],
						stock[3] + waitTime*production[3]
					),
					blueprint
				),
				maxFound
			)
	if cantBuildAnything:
		maxFound = max(
			getMaxGeode(
				0,
				(
					production[0],
					production[1],
					production[2],
					production[3]
				),
				(
					stock[0] + time*production[0],
					stock[1] + time*production[1],
					stock[2] + time*production[2],
					stock[3] + time*production[3]
				),
				blueprint
			),
			maxFound
		)
	memo[memoKey] = maxFound
	return maxFound
	
	

blueprints = list()

initProduction = (1, 0, 0, 0)

initStock = (0, 0, 0, 0)

with open(file=fileName, mode="r") as f:
	for line in f:
		line = line.strip().split(' ')
		newBlueprint = (
			(int(line[6]),),
			(int(line[12]),),
			(int(line[18]), int(line[21])),
			(int(line[27]), int(line[30]))
		)
		blueprints.append(newBlueprint)

maxGeodes = []
sumQuality = 0
ts = time.time()
for index, blueprint in enumerate(blueprints):
	maxGeodes.append(getMaxGeode(24,initProduction,initStock,blueprint))
	sumQuality += maxGeodes[-1]*(index+1)
	memo = {}
te = time.time()
print(sumQuality, te-ts)


maxGeodes = []
ts = time.time()
for blueprint in blueprints[:3]:
	maxGeodes.append(getMaxGeode(32,initProduction,initStock,blueprint))
	memo = {}
te = time.time()

print(maxGeodes[0]*maxGeodes[1]*maxGeodes[2], te-ts)

