fileName = "input"

cubes = set()

def neighbors(cubePosition):
	neighbors = set()
	neighbors.add((
		cubePosition[0] + 1,
		cubePosition[1], 
		cubePosition[2]
	))
	neighbors.add((
		cubePosition[0] - 1,
		cubePosition[1], 
		cubePosition[2]
	))
	neighbors.add((
		cubePosition[0],
		cubePosition[1] + 1, 
		cubePosition[2]
	))
	neighbors.add((
		cubePosition[0],
		cubePosition[1] - 1, 
		cubePosition[2]
	))
	neighbors.add((
		cubePosition[0],
		cubePosition[1], 
		cubePosition[2] + 1
	))
	neighbors.add((
		cubePosition[0],
		cubePosition[1], 
		cubePosition[2] - 1
	))
	return neighbors
	

with open(file=fileName, mode="r") as f:
	for line in f:
		cubes.add(tuple(map(int, line.strip().split(','))))

totalSides = 0

for cube in cubes:
	nCubesNeighbor = 0
	for neighbor in neighbors(cube):
		if neighbor in cubes:
			nCubesNeighbor += 1
	totalSides += 6 - nCubesNeighbor

print(totalSides)

# part 2

totalSides = 0

validX = (min(cubes, key=lambda x: x[0])[0] - 1, max(cubes, key=lambda x: x[0])[0] + 1)
validY = (min(cubes, key=lambda x: x[1])[1] - 1, max(cubes, key=lambda x: x[1])[1] + 1)
validZ = (min(cubes, key=lambda x: x[2])[2] - 1, max(cubes, key=lambda x: x[2])[2] + 1)

water = set()

water.add((
	validX[0],
	validY[0],
	validZ[0]
))

waterLast = set()
while waterLast != water:
	waterLast = water.copy()
	for i in range(validX[0], validX[1] + 1):
		for j in range(validY[0], validY[1] + 1):
			for k in range(validZ[0], validZ[1] + 1):
				if (i,j,k) not in cubes and (i,j,k) not in water:
					if any([neighbor in water for neighbor in neighbors((i,j,k))]):
						water.add((i,j,k))

for i in range(validX[0], validX[1] + 1):
		for j in range(validY[0], validY[1] + 1):
			for k in range(validZ[0], validZ[1] + 1):
				if (i,j,k) not in cubes and (i,j,k) not in water:
					cubes.add((i,j,k))

for cube in cubes:
	nCubesNeighbor = 0
	for neighbor in neighbors(cube):
		if neighbor in cubes:
			nCubesNeighbor += 1
	totalSides += 6 - nCubesNeighbor

print(totalSides)







