from functools import lru_cache 
class Directory:
    def __init__(self, name = "", files = None, directories = None, parentDirectory = None):
        if files == None:
            files = list()
        if directories == None:
            directories = list()
        self.name = name
        self.files = files
        self.directories = directories
        self.parentDirectory = parentDirectory
    @lru_cache(maxsize=None)
    def getSize(self):
        sizeOfFileInDiretory = 0
        sizeOfdirectories = 0
        for file in self.files:
            sizeOfFileInDiretory += file[0]
        for directory in self.directories:
            sizeOfdirectories += directory.getSize()
        return sizeOfdirectories + sizeOfFileInDiretory
    def getAllDirectoriesSizeAtMost(self, n):
        result = []
        for directory in self.directories:
            if directory.getSize() <= n:
                result.append(directory)
            result += directory.getAllDirectoriesSizeAtMost(n)
        return result
    def getAllDirectoriesSizeAtLeast(self, n):
        result = []
        for directory in self.directories:
            if directory.getSize() >= n:
                result.append(directory)
            result += directory.getAllDirectoriesSizeAtLeast(n)
        return result
fileName = "adventofcode.com_2022_day_7_input.txt"
root = Directory(name="/")
currentDirectory = None
with open(file=fileName, mode="r") as f:
    for line in f:
        if line == "$ cd /\n":
            currentDirectory = root
            continue
        if line == "$ ls\n":
            continue
        if line == "$ cd ..\n":
            if currentDirectory == root:
                continue
            currentDirectory = currentDirectory.parentDirectory
            continue       
        if "$ cd" in line:
            newDirectoryName = line.split(" ")[2][:-1]
            for directory in currentDirectory.directories:
                if directory.name == newDirectoryName:
                    currentDirectory = directory
                    break
            continue
        if "dir " in line:
            if any(map(lambda x: x.name == line.split(" ")[1][:-1],currentDirectory.directories)):
                continue
            currentDirectory.directories.append(Directory(name=line.split(" ")[1][:-1], parentDirectory=currentDirectory))
            continue
        newFileSize = int(line.split(" ")[0])
        newFileName = line.split(" ")[1][:-1]
        currentDirectory.files.append([newFileSize, newFileName])
sumDesiredSize = 0
for directory in root.getAllDirectoriesSizeAtMost(100000):
    sumDesiredSize += directory.getSize()
print(sumDesiredSize)
spaceNeeded = 30000000
filesystemSize = 70000000
spaceUsed = root.getSize()
freeSpace = filesystemSize - spaceUsed
spaceToFree = spaceNeeded - freeSpace
smallestFound = filesystemSize
for directory in root.getAllDirectoriesSizeAtLeast(spaceToFree):
    smallestFound = min(directory.getSize(),smallestFound)
print(smallestFound)
