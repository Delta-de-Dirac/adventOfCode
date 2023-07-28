fileName = "adventofcode.com_2022_day_1_input.txt"
with open(file=fileName, mode="r") as f:
    listOfCaloriesPerElf = []
    currentSum = 0
    for line in f.readlines():
        if line == "\n":
            listOfCaloriesPerElf.append(currentSum)
            currentSum = 0
            continue
        currentSum += int(line[:-1])
listOfCaloriesPerElf.append(currentSum)
print(max(listOfCaloriesPerElf))
threeTop = [max(listOfCaloriesPerElf)]
listOfCaloriesPerElf.remove(max(listOfCaloriesPerElf))
threeTop.append(max(listOfCaloriesPerElf))
listOfCaloriesPerElf.remove(max(listOfCaloriesPerElf))
threeTop.append(max(listOfCaloriesPerElf))
print(sum(threeTop))
