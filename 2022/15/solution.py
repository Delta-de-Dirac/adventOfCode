import re
fileName = "input"
y=2000000
def getInterval(sx,sy,y,bx,by):
	d = abs(bx-sx) + abs(by-sy)
	c = abs(y-sy)
	k = d - c
	I1 = sx + k
	I2 = sx - k
	if not (sy-d <= y <= sy+d):
		return None
	if I1 <= I2:
		return [I1,I2]
	else:
		return [I2,I1]

def compressIntervals(intervals):
	intervals.sort()
	currentStart = intervals[0][0]
	currentEnd = intervals[0][1]
	result = list()
	i = 0
	while i < len(intervals) - 1:
		if intervals[i+1][1] <= currentEnd:
			i += 1
			continue
		if intervals[i+1][1] > currentEnd:
			if intervals[i+1][0] > currentEnd + 1:
				result.append([currentStart,currentEnd])
				currentStart = intervals[i+1][0]
				currentEnd = intervals[i+1][1]
				i += 1
				continue
			currentEnd = intervals[i+1][1]
			i += 1
			continue
	result.append([currentStart,currentEnd])
	return result
				
	
totalInterval = set()
beaconsAtY = set()
with open(file=fileName, mode="r") as f:
	for line in f:
		line = re.split('=|,|:|\n',line)
		sx = int(line[1])
		sy = int(line[3])
		bx = int(line[5])
		by = int(line[7])
		if by == y:
			beaconsAtY.add(bx)
		interval = getInterval(sx,sy,y,bx,by)
		if interval == None:
			continue
		totalInterval |= set(range(interval[0],interval[1]+1))
print(len(totalInterval - beaconsAtY))

# Part 2

sx = list()
sy = list()
bx = list()
by = list()

with open(file=fileName, mode="r") as f:
	for line in f:
		line = re.split('=|,|:|\n',line)
		sx.append(int(line[1]))
		sy.append(int(line[3]))
		bx.append(int(line[5]))
		by.append(int(line[7]))


for y in range(0,4000001):
	intervals = list()
	for i in range(len(sx)):
		interval = getInterval(sx[i],sy[i],y,bx[i],by[i])
		if interval != None:
			intervals.append(interval)
	intervals = compressIntervals(intervals)
	if any(  map(lambda x: x[0] <= 0 and x[1] >= 4000000 , intervals )  ):
		continue
	print(y,intervals)
	break

