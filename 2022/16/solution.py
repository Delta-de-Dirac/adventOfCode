import time
fileName = "input"

memoAlt = dict()

memoAltElephant = dict()

tunnels = dict()

flowRate = dict()

sPaths = dict()

valveBit = dict()

def getShortestPaths(tunnels):
	valveSet = set(tunnels.keys())
	sPaths = dict()
	for sValve in valveSet:
		sPaths[sValve] = dict()
		Q = set()
		for valve in valveSet:
			sPaths[sValve][valve] = float('inf')
			Q.add(valve)
		sPaths[sValve][sValve] = 0
		while len(Q) > 0:
			u = min([v for v in Q],key=lambda v: sPaths[sValve][v])
			Q.remove(u)
			for v in [i for i in tunnels[u] if i in Q]:
				alt = sPaths[sValve][u] + 1
				if alt < sPaths[sValve][v]:
					sPaths[sValve][v] = alt
	return sPaths

def maxSumElephantAlt(currentValve, currentValveElephant, ttReady, ttReadyElephant, remainingMinutes, valvesNotOperatedBits):
	if currentValveElephant < currentValve:
		currentValve, currentValveElephant = currentValveElephant, currentValve
		ttReady, ttReadyElephant = ttReadyElephant, ttReady
	if remainingMinutes <= 0:
		return 0
	if valvesNotOperatedBits <= 0b0:
		return 0
	memoKey = (
		currentValve,
		currentValveElephant,
		ttReady,
		ttReadyElephant,
		remainingMinutes,
		valvesNotOperatedBits
	)
	if memoKey in memoAltElephant:
		return memoAltElephant[memoKey]
	valvesNotOperated = [valve for valve in valveBit if (valvesNotOperatedBits>>valveBit[valve])&0b1 == 1]
	maxFound = 0
	if ttReady == 0 and ttReadyElephant == 0:
		canReach = {valve for valve in valvesNotOperated if remainingMinutes-sPaths[currentValve][valve] >= 0}
		canReachElephant = {valve for valve in valvesNotOperated if remainingMinutes-sPaths[currentValveElephant][valve] >= 0}
		for nextValve in canReach:
			if len(canReachElephant - {nextValve}) > 0:
				for nextValveElephant in canReachElephant - {nextValve}:
					leastPath = min(
						sPaths[currentValve][nextValve],
						sPaths[currentValveElephant][nextValveElephant])
					pressure = (remainingMinutes-sPaths[currentValve][nextValve])*flowRate[nextValve]
					pressure += (remainingMinutes-sPaths[currentValveElephant][nextValveElephant])*flowRate[nextValveElephant]
					maxFound = max(pressure+maxSumElephantAlt(
						nextValve, 
						nextValveElephant, 
						sPaths[currentValve][nextValve] - leastPath,
						sPaths[currentValveElephant][nextValveElephant] - leastPath,
						remainingMinutes - leastPath, 
						valvesNotOperatedBits&(~((0b1<<valveBit[nextValve])|(0b1<<valveBit[nextValveElephant])))), maxFound)
			else:
				pressure = (remainingMinutes-sPaths[currentValve][nextValve])*flowRate[nextValve]
				maxFound = max(pressure+maxSumElephantAlt(
					nextValve, 
					currentValveElephant, 
					0,
					0,
					remainingMinutes - sPaths[currentValve][nextValve], 
					valvesNotOperatedBits&(~(0b1<<valveBit[nextValve]))), maxFound)  
		for nextValveElephant in canReachElephant:
			if len(canReach - {nextValveElephant}) > 0:
				for nextValve in canReach - {nextValveElephant}:
					leastPath = min(
						sPaths[currentValve][nextValve],
						sPaths[currentValveElephant][nextValveElephant])
					pressure = (remainingMinutes-sPaths[currentValve][nextValve])*flowRate[nextValve]
					pressure += (remainingMinutes-sPaths[currentValveElephant][nextValveElephant])*flowRate[nextValveElephant]
					maxFound = max(pressure+maxSumElephantAlt(
						nextValve, 
						nextValveElephant, 
						sPaths[currentValve][nextValve] - leastPath,
						sPaths[currentValveElephant][nextValveElephant] - leastPath,
						remainingMinutes - leastPath, 
						valvesNotOperatedBits&(~((0b1<<valveBit[nextValve])|(0b1<<valveBit[nextValveElephant])))), maxFound)
			else:
				pressure = (remainingMinutes-sPaths[currentValveElephant][nextValveElephant])*flowRate[nextValveElephant]
				maxFound = max(pressure+maxSumElephantAlt(
					currentValve, 
					nextValveElephant, 
					0,
					0,
					remainingMinutes - sPaths[currentValveElephant][nextValveElephant], 
					valvesNotOperatedBits&(~(0b1<<valveBit[nextValveElephant]))), maxFound)
	if ttReady == 0 and ttReadyElephant != 0:
		canReach = {valve for valve in valvesNotOperated if remainingMinutes-sPaths[currentValve][valve] >= 0}
		if len(canReach) > 0:
			for nextValve in canReach:
				pressure = (remainingMinutes-sPaths[currentValve][nextValve])*flowRate[nextValve]
				leastPath = min(sPaths[currentValve][nextValve],ttReadyElephant)
				maxFound = max(pressure+maxSumElephantAlt(
					nextValve, 
					currentValveElephant, 
					sPaths[currentValve][nextValve] - leastPath,
					ttReadyElephant - leastPath,
					remainingMinutes - leastPath, 
					valvesNotOperatedBits&(~(0b1<<valveBit[nextValve]))), maxFound)
		else:
			maxFound = max(maxSumElephantAlt(
					currentValve, 
					currentValveElephant, 
					0,
					0,
					remainingMinutes - ttReadyElephant, 
					valvesNotOperatedBits), maxFound)
			
	if ttReady != 0 and ttReadyElephant == 0:
		canReachElephant = {valve for valve in valvesNotOperated if remainingMinutes-sPaths[currentValveElephant][valve] >= 0}
		if len(canReachElephant) > 0:
			for nextValveElephant in canReachElephant:
				pressure = (remainingMinutes-sPaths[currentValveElephant][nextValveElephant])*flowRate[nextValveElephant]
				leastPath = min(sPaths[currentValveElephant][nextValveElephant],ttReady)
				maxFound = max(pressure+maxSumElephantAlt(
					currentValve, 
					nextValveElephant, 
					ttReady - leastPath,
					sPaths[currentValveElephant][nextValveElephant] - leastPath,
					remainingMinutes - leastPath, 
					valvesNotOperatedBits&(~(0b1<<valveBit[nextValveElephant]))), maxFound)
		else:
			maxFound = max(maxSumElephantAlt(
					currentValve, 
					currentValveElephant, 
					0,
					0,
					remainingMinutes - ttReady, 
					valvesNotOperatedBits), maxFound)
	memoAltElephant[memoKey] = maxFound
	return maxFound

def maxSumAlt(currentValve, remainingMinutes, valvesNotOperatedBits):
	if remainingMinutes <= 0:
		return 0
	if valvesNotOperatedBits <= 0b0:
		return 0
	memoKey = (
		currentValve,
		remainingMinutes,
		valvesNotOperatedBits
	)
	valvesNotOperated = [valve for valve in valveBit if (valvesNotOperatedBits>>valveBit[valve])&0b1 == 1]
	if memoKey in memoAlt:
		return memoAlt[memoKey]
	maxFound = 0
	for nextValve in valvesNotOperated:
		if remainingMinutes-sPaths[currentValve][nextValve] >= 0:
			pressure = (remainingMinutes-sPaths[currentValve][nextValve])*flowRate[nextValve]
			maxFound = max(pressure+maxSumAlt(nextValve,remainingMinutes-sPaths[currentValve][nextValve], valvesNotOperatedBits&(~(0b1<<valveBit[nextValve]))), maxFound)
	
	memoAlt[memoKey] = maxFound
	return maxFound

valvesNotOperated = set()

with open(file=fileName, mode="r") as f:
	for line in f:
		line = line.split(' ')
		valve = line[1]
		flowRate[valve] = int(line[4][5:-1])
		tunnels[valve] = list()
		if flowRate[valve] != 0:
			valvesNotOperated.add(valve)
		for tunnel in line[9:]:
			tunnels[valve].append(tunnel.strip(',').strip('\n'))

sPaths = getShortestPaths(tunnels)

valveBit = {valve:bit for valve,bit in zip(valvesNotOperated,range(0,len(valvesNotOperated)))}

valvesNotOperatedBits = 0

for valve in valveBit:
	valvesNotOperatedBits |= 0b1 << valveBit[valve]

for i in list(sPaths.keys()):
	for j in list(sPaths[i].keys()):
		sPaths[i][j] += 1

ts = time.time()
print(maxSumAlt('AA', 30, valvesNotOperatedBits))
te = time.time()

print("Memo dict length part 1:",len(memoAlt))
print("Time part 1:", te-ts)
# part 2

tunnels = dict()

flowRate = dict()

valvesNotOperated = set()

with open(file=fileName, mode="r") as f:
	for line in f:
		line = line.split(' ')
		valve = line[1]
		flowRate[valve] = int(line[4][5:-1])
		tunnels[valve] = list()
		if flowRate[valve] != 0:
			valvesNotOperated.add(valve)
		for tunnel in line[9:]:
			tunnels[valve].append(tunnel.strip(',').strip('\n'))
			
valveBit = {valve:bit for valve,bit in zip(valvesNotOperated,range(0,len(valvesNotOperated)))}

valvesNotOperatedBits = 0

for valve in valveBit:
	valvesNotOperatedBits |= 0b1 << valveBit[valve]

ts = time.time()
print(maxSumElephantAlt(
	currentValve='AA', 
	currentValveElephant='AA', 
	ttReady=0, 
	ttReadyElephant=0, 
	remainingMinutes=26, 
	valvesNotOperatedBits=valvesNotOperatedBits))
te = time.time()

# print("Memo dict length part 2:",len(memoDecisionElephant))
print("Time part 2: ", te-ts)