fileName = "adventofcode.com_2022_day_10_input.txt"

currentCycle = 0
xValue = 1
signalCycles = [x for x in range(20,221,40)]
signals = list()
with open(file=fileName, mode="r") as f:
    for line in f:
        currentCycle += 1
        if currentCycle in signalCycles:
            signals.append(xValue*currentCycle)
        if line.split(" ")[0] == "addx":
            currentCycle += 1
            if currentCycle in signalCycles:
                signals.append(xValue*currentCycle)
            xValue += int(line.split(" ")[1][:-1])
print(sum(signals))

currentCycle = 0
xValue = 1
cyclesToAddLineBreak = [x for x in range(40, 241, 40)]
crt = ""
with open(file=fileName, mode="r") as f:
    for line in f:
        currentCycle += 1
        if xValue - 1 <= (currentCycle-1)%40+1 <= xValue + 1:
            crt += "#"
        else:
            crt += "."
        if currentCycle in cyclesToAddLineBreak:
            crt += "\n"
        if line.split(" ")[0] == "addx":
            currentCycle += 1
            xValue += int(line.split(" ")[1][:-1])
            if xValue - 1<= (currentCycle-1)%40+1 <= xValue + 1:
                crt += "#"
            else:
                crt += "."
            if currentCycle in cyclesToAddLineBreak:
                crt += "\n"
            
print(crt)