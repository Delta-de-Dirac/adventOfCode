import time
fileName = "test_input"

memoDecision = dict()

memoDecisionElephant = dict()

memoAlt = dict()

memoAltElephant = dict()

tunnels = dict()

flowRate = dict()

sPaths = dict()

valveBit = dict()

def getOrderedStringSet(mySet):
	x = list(mySet)
	x.sort()
	s = '.'.join(x)
	return s

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



def maxSumElephant(currentValve, elephantCurrentValve, remainingMinutes, valvesNotOperated):
    if elephantCurrentValve < currentValve:
        tmp = elephantCurrentValve
        elephantCurrentValve = currentValve
        currentValve = tmp
    if remainingMinutes <= 0:
            return 0
    if (currentValve, elephantCurrentValve, remainingMinutes, getOrderedStringSet(valvesNotOperated)) in memoDecisionElephant:
        decision = memoDecisionElephant[(currentValve, elephantCurrentValve, remainingMinutes, getOrderedStringSet(valvesNotOperated))]
        if decision  == ('op','op'):
            valvesNotOperatedNext = valvesNotOperated.copy()
            valvesNotOperatedNext.remove(currentValve)
            valvesNotOperatedNext.remove(elephantCurrentValve)
            result = maxSumElephant(currentValve, elephantCurrentValve, remainingMinutes-1,valvesNotOperatedNext)
            if currentValve != elephantCurrentValve:
                return (remainingMinutes-1)*(flowRate[currentValve] + flowRate[elephantCurrentValve]) + result
            return (remainingMinutes-1)*flowRate[currentValve] + result
        if decision[0] == 'op':
            valvesNotOperatedNext = valvesNotOperated.copy()
            valvesNotOperatedNext.remove(currentValve)
            elephantNextValve = decision[1]
            result = maxSumElephant(currentValve, elephantNextValve, remainingMinutes-1,valvesNotOperatedNext)
            return (remainingMinutes-1)*flowRate[currentValve] + result
        if decision[1] == 'op':
            valvesNotOperatedNext = valvesNotOperated.copy()
            valvesNotOperatedNext.remove(elephantCurrentValve)
            nextValve = decision[0]
            result = maxSumElephant(nextValve, elephantCurrentValve, remainingMinutes-1,valvesNotOperatedNext)
            return (remainingMinutes-1)*flowRate[elephantCurrentValve] + result
        else:
            nextValve = decision[0]
            elephantNextValve = decision[1]
            result = maxSumElephant(nextValve, elephantNextValve, remainingMinutes-1,valvesNotOperated.copy())
            return result
    if len(valvesNotOperated) == 0:
        return 0
    
    possibilities = list()
    
    if currentValve in valvesNotOperated:
        valvesNotOperatedNext = valvesNotOperated.copy()
        valvesNotOperatedNext.remove(currentValve)
        for elephantNextValve in tunnels[elephantCurrentValve]:
            result = maxSumElephant(currentValve, elephantNextValve ,remainingMinutes-1 ,valvesNotOperatedNext)
            possibilities.append(((remainingMinutes-1)*flowRate[currentValve] + result,('op',elephantNextValve)))
    
    if elephantCurrentValve in valvesNotOperated:
        valvesNotOperatedNext = valvesNotOperated.copy()
        valvesNotOperatedNext.remove(elephantCurrentValve)
        for nextValve in tunnels[currentValve]:
            result = maxSumElephant(nextValve, elephantCurrentValve ,remainingMinutes-1 ,valvesNotOperatedNext)
            possibilities.append(((remainingMinutes-1)*flowRate[elephantCurrentValve] + result,(nextValve,'op')))
    
    if currentValve in valvesNotOperated and elephantCurrentValve in valvesNotOperated and currentValve != elephantCurrentValve:
        valvesNotOperatedNext = valvesNotOperated.copy()
        valvesNotOperatedNext.remove(currentValve)
        valvesNotOperatedNext.remove(elephantCurrentValve) 
            
        result = maxSumElephant(currentValve, elephantCurrentValve ,remainingMinutes-1 ,valvesNotOperatedNext)
        if currentValve != elephantCurrentValve:
            possibilities.append(((remainingMinutes-1)*(flowRate[currentValve] + flowRate[elephantCurrentValve]) + result, ('op','op') ))
        else:
            possibilities.append(((remainingMinutes-1)*flowRate[currentValve] + result,('op','op')))
    
    for nextValve, elephantNextValve in [(m,n) for m in tunnels[currentValve] for n in tunnels[elephantCurrentValve]]:
        result = maxSumElephant(nextValve, elephantNextValve ,remainingMinutes-1 ,valvesNotOperated.copy())
        possibilities.append((result,(nextValve,elephantNextValve)))
    decision = max(possibilities, key=lambda x : x[0])[1]
    memoDecisionElephant[(currentValve, elephantCurrentValve, remainingMinutes, getOrderedStringSet(valvesNotOperated))] = decision
    return max([possibility[0] for possibility in possibilities])
    
def maxSum(currentValve, remainingMinutes, valvesNotOperated):
    if remainingMinutes <= 0:
        return 0
    if (currentValve, remainingMinutes, getOrderedStringSet(valvesNotOperated)) in memoDecision:
        if memoDecision[(currentValve, remainingMinutes, getOrderedStringSet(valvesNotOperated))] == 'op':
            valvesNotOperatedNext = valvesNotOperated.copy()
            valvesNotOperatedNext.remove(currentValve)
            result = maxSum(currentValve,remainingMinutes-1,valvesNotOperatedNext)
            return (remainingMinutes-1)*flowRate[currentValve] + result
        else:
            result = maxSum(memoDecision[(currentValve, remainingMinutes, getOrderedStringSet(valvesNotOperated))],remainingMinutes-1,valvesNotOperated.copy())
            return result
    if len(valvesNotOperated) == 0:
        return 0
    possibilities = list()
    if currentValve in valvesNotOperated:
        valvesNotOperatedNext = valvesNotOperated.copy()
        valvesNotOperatedNext.remove(currentValve)
        result = maxSum(currentValve,remainingMinutes-1,valvesNotOperatedNext)
        possibilities.append(((remainingMinutes-1)*flowRate[currentValve] + result,'op'))
    for nextValve in tunnels[currentValve]:
        result = maxSum(nextValve,remainingMinutes-1,valvesNotOperated.copy())
        possibilities.append((result,nextValve))
    decision = max(possibilities, key=lambda x : x[0])[1]
    memoDecision[(currentValve, remainingMinutes, getOrderedStringSet(valvesNotOperated))] = decision
    return max([possibility[0] for possibility in possibilities])

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
                
    
    """
    
    
    if ttReady == 0 and ttReadyElephant == 0:
        for nextValve, nextValveElephant in [(i,j) for i in valvesNotOperated for j in valvesNotOperated]:
            if remainingMinutes-sPaths[currentValve][nextValve] >= 0 and remainingMinutes-sPaths[currentValveElephant][nextValveElephant] >= 0:
                leastPath = min(sPaths[currentValve][nextValve],sPaths[currentValveElephant][nextValveElephant])
                if nextValve == nextValveElephant:
                    pressure = (remainingMinutes-leastPath)*flowRate[nextValve]
                else:
                    pressure = (remainingMinutes-sPaths[currentValve][nextValve])*flowRate[nextValve]
                    pressure += (remainingMinutes-sPaths[currentValveElephant][nextValveElephant])*flowRate[nextValveElephant]
                maxFound = max(pressure+maxSumElephantAlt(
                    nextValve, 
                    nextValveElephant, 
                    sPaths[currentValve][nextValve] - leastPath,
                    sPaths[currentValveElephant][nextValveElephant] - leastPath,
                    remainingMinutes - leastPath, 
                    valvesNotOperatedBits&(~((0b1<<valveBit[nextValve])|(0b1<<valveBit[nextValveElephant])))), maxFound)
    
    """
    
    
    if ttReady == 0 and ttReadyElephant != 0:
        for nextValve in valvesNotOperated:
            if remainingMinutes-sPaths[currentValve][nextValve] >= 0:
                pressure = (remainingMinutes-sPaths[currentValve][nextValve])*flowRate[nextValve]
                leastPath = min(sPaths[currentValve][nextValve],ttReadyElephant)
                maxFound = max(pressure+maxSumElephantAlt(
                    nextValve, 
                    currentValveElephant, 
                    sPaths[currentValve][nextValve] - leastPath,
                    ttReadyElephant - leastPath,
                    remainingMinutes - leastPath, 
                    valvesNotOperatedBits&(~(0b1<<valveBit[nextValve]))), maxFound)
    if ttReady != 0 and ttReadyElephant == 0:
        for nextValveElephant in valvesNotOperated:
            if remainingMinutes-sPaths[currentValveElephant][nextValveElephant] >= 0:
                pressure = (remainingMinutes-sPaths[currentValveElephant][nextValveElephant])*flowRate[nextValveElephant]
                leastPath = min(sPaths[currentValveElephant][nextValveElephant],ttReady)
                maxFound = max(pressure+maxSumElephantAlt(
                    currentValve, 
                    nextValveElephant, 
                    ttReady - leastPath,
                    sPaths[currentValveElephant][nextValveElephant] - leastPath,
                    remainingMinutes - leastPath, 
                    valvesNotOperatedBits&(~(0b1<<valveBit[nextValveElephant]))), maxFound)
    memoAltElephant[memoKey] = maxFound
    return maxFound

def maxSumAlt(currentValve, remainingMinutes, valvesNotOperated):
    
    l = list(valvesNotOperated)
    l.sort()
    l = tuple(l)
    
    if remainingMinutes <= 0:
        return 0
    if len(valvesNotOperated) <= 0:
        return 0
    if (currentValve, remainingMinutes, l ) in memoAlt:
        return memoAlt[(currentValve, remainingMinutes, l )]
    possibilities = [0]
    for nextValve in valvesNotOperated:
        if remainingMinutes-sPaths[currentValve][nextValve] >= 0:
            pressure = (remainingMinutes-sPaths[currentValve][nextValve])*flowRate[nextValve]
            possibilities.append(pressure+maxSumAlt(nextValve,remainingMinutes-sPaths[currentValve][nextValve], valvesNotOperated-{nextValve}))
    
    memoAlt[(currentValve, remainingMinutes, l )] = max(possibilities)
    return max(possibilities)

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

for i in list(sPaths.keys()):
    for j in list(sPaths[i].keys()):
        sPaths[i][j] += 1

ts = time.time()
#print(maxSum('AA', 30, valvesNotOperated))
print(maxSumAlt('AA', 30, valvesNotOperated))
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