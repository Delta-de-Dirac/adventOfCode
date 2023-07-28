fileName = "adventofcode.com_2022_day_5_input.txt"

with open(file=fileName, mode="r") as f:
    stacks = dict()
    for i in range(1, 10):
        stacks[i] = list()
    for i, line in enumerate(f):
        if i >= 8:
            break
        stackIndex = 1
        for j in range(1, len(line), 4):
            if line[j] != ' ':
                stacks[stackIndex].insert(0, line[j])
            stackIndex += 1
    for i, line in enumerate(f):
        if line[0] != 'm':
            continue
        line = line.split(' ')
        moveAmount, moveFrom, moveTo = int(line[1]), int(line[3]), int(line[5])
        for j in range(moveAmount):
            stacks[moveTo].append(stacks[moveFrom].pop())
result = ''
for i in stacks:
    result += stacks[i][-1]
print(result)
with open(file=fileName, mode="r") as f:
    stacks = dict()
    for i in range(1, 10):
        stacks[i] = list()
    for i, line in enumerate(f):
        if i >= 8:
            break
        stackIndex = 1
        for j in range(1, len(line), 4):
            if line[j] != ' ':
                stacks[stackIndex].insert(0, line[j])
            stackIndex += 1
    for i, line in enumerate(f):
        if line[0] != 'm':
            continue
        line = line.split(' ')
        moveAmount, moveFrom, moveTo = int(line[1]), int(line[3]), int(line[5])
        stacks[moveTo] += stacks[moveFrom][-moveAmount:]
        del stacks[moveFrom][-moveAmount:]
result = ''
for i in stacks:
    result += stacks[i][-1]
print(result)
