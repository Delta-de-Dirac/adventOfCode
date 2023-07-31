fileName = "input"

def compareLists(leftList, rightList):
    if len(leftList) == 0 and len(rightList) != 0:
        return True
    if len(leftList) != 0 and len(rightList) == 0:
        return False
    if len(leftList) == 0 and len(rightList) == 0:
        return 'advanceBoth'
    if type(leftList[0]) is int and type(rightList[0]) is int:
        if leftList[0] > rightList[0]:
            return False
        if leftList[0] < rightList[0]:
            return True
        if leftList[0] == rightList[0]:
            return compareLists(leftList[1:],rightList[1:])
    if type(leftList[0]) is int and type(rightList[0]) is list:
        return compareLists([leftList[0]],rightList[0])
    if type(leftList[0]) is list and type(rightList[0]) is int:
        return compareLists(leftList[0],[rightList[0]])
    if compareLists(leftList[0],rightList[0]) == 'advanceBoth':
        return compareLists(leftList[1:],rightList[1:])
    return compareLists(leftList[0],rightList[0])
        


index = 0
indexSum = 0
with open(file=fileName, mode="r") as f:
        leftList = ''
        rightList = ''
        blankLine = ''
        while True:
            try:
                leftList = eval(next(f)[:-1])
                rightList = eval(next(f)[:-1])
                index += 1
                if compareLists(leftList, rightList):
                    indexSum += index
                blankLine = next(f)[:-1]
            except StopIteration:
                break
print(indexSum)

# part 2


firstDividerIndex = 1
secondDividerIndex = 1
with open(file=fileName, mode="r") as f:
        leftList = ''
        rightList = ''
        blankLine = ''
        while True:
            try:
                leftList = eval(next(f)[:-1])
                rightList = eval(next(f)[:-1])
                if compareLists(leftList, [[2]]):
                    firstDividerIndex += 1
                if compareLists(rightList, [[2]]):
                    firstDividerIndex += 1
                if compareLists(leftList, [[6]]):
                    secondDividerIndex += 1
                if compareLists(rightList, [[6]]):
                    secondDividerIndex += 1
                blankLine = next(f)[:-1]
            except StopIteration:
                break
print(firstDividerIndex*(secondDividerIndex+1))                
    # If both values are integers, the lower integer should come first. 
        # If the left integer is lower than the right integer, the inputs are in the right order. 
        # If the left integer is higher than the right integer, the inputs are not in the right order. 
        # Otherwise, the inputs are the same integer; 
            # continue checking the next part of the input.
    # If both values are lists, compare the first value of each list, then the second value, and so on. 
        # If the left list runs out of items first, the inputs are in the right order. 
        # If the right list runs out of items first, the inputs are not in the right order. 
        # If the lists are the same length and no comparison makes a decision about the order, 
            # continue checking the next part of the input.
    # If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, 
    # then retry the comparison. 
    # For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); 
    # the result is then found by instead comparing [0,0,0] and [2].

            
    