fileName = "input"
heightMap = list()

def getHeightFromLetter(letter):
    if letter == 'S':
        return 0
    if letter == 'E':
        return ord('z') - ord('a')
    if ord('a') <= ord(letter) <= ord('z'):
        return ord(letter) - ord('a')

def getNeighbor(v,heightMap):
    mapHeight = len(heightMap)
    mapWidth = len(heightMap[0])
    result = []
    #north
    if v[0] - 1 >= 0:
        if heightMap[v[0] - 1][v[1]] <= heightMap[v[0]][v[1]] + 1:
            result.append( (v[0] - 1,v[1]) )
    #south
    if v[0] + 1 < mapHeight:
        if heightMap[v[0] + 1][v[1]] <= heightMap[v[0]][v[1]] + 1:
            result.append( (v[0] + 1,v[1]) )
    #east
    if v[1] + 1 < mapWidth:
        if heightMap[v[0]][v[1] + 1] <= heightMap[v[0]][v[1]] + 1:
            result.append( (v[0], v[1] + 1) )
    #west
    if v[1] - 1 >= 0:
        if heightMap[v[0]][v[1] - 1] <= heightMap[v[0]][v[1]] + 1:
            result.append( (v[0], v[1] - 1) )
    return result
    
def getNeighborBackward(v,heightMap):
    mapHeight = len(heightMap)
    mapWidth = len(heightMap[0])
    result = []
    #north
    if v[0] - 1 >= 0:
        if heightMap[v[0] - 1][v[1]] >= heightMap[v[0]][v[1]] - 1:
            result.append( (v[0] - 1,v[1]) )
    #south
    if v[0] + 1 < mapHeight:
        if heightMap[v[0] + 1][v[1]] >= heightMap[v[0]][v[1]] - 1:
            result.append( (v[0] + 1,v[1]) )
    #east
    if v[1] + 1 < mapWidth:
        if heightMap[v[0]][v[1] + 1] >= heightMap[v[0]][v[1]] - 1:
            result.append( (v[0], v[1] + 1) )
    #west
    if v[1] - 1 >= 0:
        if heightMap[v[0]][v[1] - 1] >= heightMap[v[0]][v[1]] - 1:
            result.append( (v[0], v[1] - 1) )
    return result

sPos = None
ePos = None

i = 0
with open(file=fileName, mode="r") as f:
    for line in f:
        if sPos == None:
            if line.find('S') != -1:
                sPos = (i,line.find('S'))
        
        if ePos == None:
            if line.find('E') != -1:
                ePos = (i,line.find('E'))
            
        heightLine = list(map(getHeightFromLetter, list(line[:-1])))
        heightMap.append(heightLine)
        i += 1

Q = set()
dist = dict()
prev = dict()

for i in range(len(heightMap)):
    for j in range(len(heightMap[0])):
        v = (i,j)
        dist[v] = float('inf')
        prev[v] = None
        Q.add(v)
        
dist[sPos] = 0
while len(Q) > 0:
    u = min([v for v in Q],key=lambda v: dist[v]) # vertex in Q with min dist[u]
    if u == ePos:
        break
    Q.remove(u)
    
    for v in [i for i in getNeighbor(u,heightMap) if i in Q]: # for each neighbor v of u still in Q:
        alt = dist[u] + 1 # alt = dist[u] + Graph.Edges(u, v)
        if alt < dist[v]:
            dist[v] = alt
            prev[v] = u

print(dist[ePos])

# part 2

Q = set()
dist = dict()
prev = dict()

for i in range(len(heightMap)):
    for j in range(len(heightMap[0])):
        v = (i,j)
        dist[v] = float('inf')
        prev[v] = None
        Q.add(v)
        
dist[ePos] = 0
while len(Q) > 0:
    u = min([v for v in Q],key=lambda v: dist[v]) # vertex in Q with min dist[u]
    Q.remove(u)
    for v in [i for i in getNeighborBackward(u,heightMap) if i in Q]: # for each neighbor v of u still in Q:
        alt = dist[u] + 1 # alt = dist[u] + Graph.Edges(u, v)
        if alt < dist[v]:
            dist[v] = alt
            prev[v] = u

print(min([dist[v] for v in dist.keys() if heightMap[v[0]][v[1]] == 0]))