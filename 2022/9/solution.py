def updateKnotsPositions(knotsPositions):
    for i, knotPosition in enumerate(knotsPositions[1:]):
        dx = knotsPositions[i][0] - knotPosition[0]
        dy = knotsPositions[i][1] - knotPosition[1]
        if (abs(dx) >= 2 and dy == 0):
            knotPosition[0] += (lambda x: 1 if x > 0 else -1)(dx)
            continue
        if (abs(dy) >= 2 and dx == 0):
            knotPosition[1] += (lambda x: 1 if x > 0 else -1)(dy)
            continue
        if (abs(dy) <= 1 and abs(dx) <= 1):
            continue
        knotPosition[0] += (lambda x: 1 if x > 0 else -1)(dx)
        knotPosition[1] += (lambda x: 1 if x > 0 else -1)(dy)
        continue
    return tuple(knotsPositions[-1])
fileName = "adventofcode.com_2022_day_9_input.txt"

hx = 0
hy = 0

tx = 0
ty = 0
tailVisited = set()
tailVisited.add((tx,ty))

dx = 0
dy = 0

with open(file=fileName, mode="r") as f:
    for line in f:
        command, magnitude = line.split(" ")[0], int(line.split(" ")[1][:-1])
        if command == "R":
            for i in range(magnitude):
                # move head
                hx += 1
                dx += 1
                # move tail
                # in-line
                if (abs(dx) >= 2 and dy == 0):
                    tx += (lambda x: 1 if x > 0 else -1)(dx)
                    dx -= (lambda x: 1 if x > 0 else -1)(dx)
                    tailVisited.add((tx,ty))
                    continue
                if (abs(dy) >= 2 and dx == 0):
                    ty += (lambda x: 1 if x > 0 else -1)(dy)
                    dy -= (lambda x: 1 if x > 0 else -1)(dy)
                    tailVisited.add((tx,ty))
                    continue
                if (abs(dy) <= 1 and abs(dx) <= 1):
                    continue
                tx += (lambda x: 1 if x > 0 else -1)(dx)
                dx -= (lambda x: 1 if x > 0 else -1)(dx)
                ty += (lambda x: 1 if x > 0 else -1)(dy)
                dy -= (lambda x: 1 if x > 0 else -1)(dy)
                tailVisited.add((tx,ty))
                continue
                
        if command == "U":
            for i in range(magnitude):
                # move head
                hy += 1
                dy += 1
                # move tail
                if (abs(dx) >= 2 and dy == 0):
                    tx += (lambda x: 1 if x > 0 else -1)(dx)
                    dx -= (lambda x: 1 if x > 0 else -1)(dx)
                    tailVisited.add((tx,ty))
                    continue
                if (abs(dy) >= 2 and dx == 0):
                    ty += (lambda x: 1 if x > 0 else -1)(dy)
                    dy -= (lambda x: 1 if x > 0 else -1)(dy)
                    tailVisited.add((tx,ty))
                    continue
                if (abs(dy) <= 1 and abs(dx) <= 1):
                    continue
                tx += (lambda x: 1 if x > 0 else -1)(dx)
                dx -= (lambda x: 1 if x > 0 else -1)(dx)
                ty += (lambda x: 1 if x > 0 else -1)(dy)
                dy -= (lambda x: 1 if x > 0 else -1)(dy)
                tailVisited.add((tx,ty))
                continue
        
        if command == "L":
            for i in range(magnitude):
                # move head
                hx -= 1
                dx -= 1
                # move tail
                # in-line
                if (abs(dx) >= 2 and dy == 0):
                    tx += (lambda x: 1 if x > 0 else -1)(dx)
                    dx -= (lambda x: 1 if x > 0 else -1)(dx)
                    tailVisited.add((tx,ty))
                    continue
                if (abs(dy) >= 2 and dx == 0):
                    ty += (lambda x: 1 if x > 0 else -1)(dy)
                    dy -= (lambda x: 1 if x > 0 else -1)(dy)
                    tailVisited.add((tx,ty))
                    continue
                if (abs(dy) <= 1 and abs(dx) <= 1):
                    continue
                tx += (lambda x: 1 if x > 0 else -1)(dx)
                dx -= (lambda x: 1 if x > 0 else -1)(dx)
                ty += (lambda x: 1 if x > 0 else -1)(dy)
                dy -= (lambda x: 1 if x > 0 else -1)(dy)
                tailVisited.add((tx,ty))
                continue
        
        if command == "D":
            for i in range(magnitude):
                # move head
                hy -= 1
                dy -= 1
                # move tail
                # in-line
                if (abs(dx) >= 2 and dy == 0):
                    tx += (lambda x: 1 if x > 0 else -1)(dx)
                    dx -= (lambda x: 1 if x > 0 else -1)(dx)
                    tailVisited.add((tx,ty))
                    continue
                if (abs(dy) >= 2 and dx == 0):
                    ty += (lambda x: 1 if x > 0 else -1)(dy)
                    dy -= (lambda x: 1 if x > 0 else -1)(dy)
                    tailVisited.add((tx,ty))
                    continue
                if (abs(dy) <= 1 and abs(dx) <= 1):
                    continue
                tx += (lambda x: 1 if x > 0 else -1)(dx)
                dx -= (lambda x: 1 if x > 0 else -1)(dx)
                ty += (lambda x: 1 if x > 0 else -1)(dy)
                dy -= (lambda x: 1 if x > 0 else -1)(dy)
                tailVisited.add((tx,ty))
                continue
print(len(tailVisited))
knotsPositions = [[0,0] for i in range(10)]

hx = 0
hy = 0

tailVisited = set()
tailVisited.add((0,0))

with open(file=fileName, mode="r") as f:
    for line in f:
        command, magnitude = line.split(" ")[0], int(line.split(" ")[1][:-1])
        if command == "R":
            for i in range(magnitude):
                # move head
                knotsPositions[0][0] += 1
                tailVisited.add(updateKnotsPositions(knotsPositions))
        if command == "U":
            for i in range(magnitude):
                # move head
                knotsPositions[0][1] += 1
                tailVisited.add(updateKnotsPositions(knotsPositions))
        if command == "L":
            for i in range(magnitude):
                # move head
                knotsPositions[0][0] -= 1
                tailVisited.add(updateKnotsPositions(knotsPositions))
        if command == "D":
            for i in range(magnitude):
                # move head
                knotsPositions[0][1] -= 1
                tailVisited.add(updateKnotsPositions(knotsPositions))
print(len(tailVisited))