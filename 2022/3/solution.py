fileName = "adventofcode.com_2022_day_3_input.txt"
letterScore = dict()
for x in range(ord('a'), ord('z')+1):
    letterScore[chr(x)] = x - ord('a') + 1
for x in range(ord('A'), ord('Z')+1):
    letterScore[chr(x)] = x - ord('A') + 27
currentSum = 0
with open(file=fileName, mode="r") as f:
    for line in f.readlines():
        currentSum += letterScore[
                list(set(line[:len(line)//2]) &
                     set(line[len(line)//2:])
                     )[0]]
print(currentSum)
with open(file=fileName, mode="r") as f:
    linesIter = iter(f.readlines())
    badges = list()
    for line0, line1, line2 in zip(linesIter, linesIter, linesIter):
        badges.append(letterScore[
            list(set(line0[:-1]) &
                 set(line1[:-1]) &
                 set(line2[:-1])
                 )[0]])
print(sum(badges))
