class Directory:
    def __init__(self, name = "", files = [], directories = [], parentDirectory = None):
        self.name = name
        self.files = files
        self.directories = directories
        self.parentDirectory = parentDirectory
    def getSize(self):
        sizeOfFileInDiretory = 0
        sizeOfdirectories = 0
        for file in self.files:
            sizeOfFileInDiretory += file[0]
        for directory in self.directories:
            sizeOfdirectories += directory.getSize()
        return sizeOfdirectories + sizeOfFileInDiretory


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
            currentDirectory.directories.append(Directory(name=line.split(" ")[1][:-1], parentDirectory=currentDirectory))
            continue

        newFileSize = int(line.split(" ")[0])
        newFileName = line.split(" ")[1][:-1]
        currentDirectory.files.append([newFileSize, newFileName])
print(root.getSize())
