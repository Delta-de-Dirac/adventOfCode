import math
fileName = "input"

memo = {}

def getMaxGeode(time, production, stock, blueprint):
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
	# build ore next 
	if production[0] < max(blueprint[0][0],blueprint[1][0],blueprint[2][0],blueprint[3][0]):
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
	if production[1] < blueprint[2][1]:
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
	if production[1] > 0 and production[2] < blueprint[3][1]:
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
	if production[2] > 0:
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


sumQuality = 0
for index, blueprint in enumerate(blueprints):
	print("processing bluprint ID:", index+1)
	sumQuality += getMaxGeode(24,initProduction,initStock,blueprint)*(index+1)
	print("quality so far:",sumQuality)
	memo = {}
print(sumQuality)
