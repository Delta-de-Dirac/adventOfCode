fileName = "adventofcode.com_2022_day_6_input.txt"
countLetters = 0
with open(file=fileName, mode="r") as f:
    found = False
    for line in f:
        currentWindow = []
        for letter in line:
            countLetters += 1
            currentWindow.append(letter)
            if len(currentWindow) > 4:
                del currentWindow[0]
            if len(set(currentWindow)) >= 4:
                found = True
                break
        if found:
            break
print(countLetters)
countLetters = 0
with open(file=fileName, mode="r") as f:
    found = False
    for line in f:
        currentWindow = []
        for letter in line:
            countLetters += 1
            currentWindow.append(letter)
            if len(currentWindow) > 14:
                del currentWindow[0]
            if len(set(currentWindow)) >= 14:
                found = True
                break
        if found:
            break
print(countLetters)
