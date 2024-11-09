const inputFileName = "input/day03.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

function isThereTreeAt(l, c, treeMap) {
    return (treeMap.at(l).at(c%treeMap.at(l).length) === '#');
}

// Part 1

let result = 0;

let column = 0;

for (let line = 0; line < nLines; line++){
    if (isThereTreeAt(line, column, inputLines)) {
        result += 1;
    }
    column += 3;
}

console.log(`Result part 1: ${result}`);

// Part 2
