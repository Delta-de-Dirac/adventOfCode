fileName = "adventofcode.com_2022_day_2_input.txt"
scoreForShape = {"X": 1,
                 "Y": 2,
                 "Z": 3}
scoreForOutcome = {"lose": 0,
                   "draw": 3,
                   "win": 6}
currentSum = 0
with open(file=fileName, mode="r") as f:
    for line in f.readlines():
        if line[0] == "A":
            if line[2] == "X":
                currentSum += 3 + 1
                continue
            if line[2] == "Y":
                currentSum += 6 + 2
                continue
            if line[2] == "Z":
                currentSum += 0 + 3
        if line[0] == "B":
            if line[2] == "X":
                currentSum += 0 + 1
                continue
            if line[2] == "Y":
                currentSum += 3 + 2
                continue
            if line[2] == "Z":
                currentSum += 6 + 3
            continue
        if line[0] == "C":
            if line[2] == "X":
                currentSum += 6 + 1
                continue
            if line[2] == "Y":
                currentSum += 0 + 2
                continue
            if line[2] == "Z":
                currentSum += 3 + 3
            continue
print(currentSum)
currentSum = 0
with open(file=fileName, mode="r") as f:
    for line in f.readlines():
        if line[0] == "A":
            if line[2] == "X":
                currentSum += 0 + 3
                continue
            if line[2] == "Y":
                currentSum += 3 + 1
                continue
            if line[2] == "Z":
                currentSum += 6 + 2
        if line[0] == "B":
            if line[2] == "X":
                currentSum += 0 + 1
                continue
            if line[2] == "Y":
                currentSum += 3 + 2
                continue
            if line[2] == "Z":
                currentSum += 6 + 3
            continue
        if line[0] == "C":
            if line[2] == "X":
                currentSum += 0 + 2
                continue
            if line[2] == "Y":
                currentSum += 3 + 3
                continue
            if line[2] == "Z":
                currentSum += 6 + 1
            continue
print(currentSum)
