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

const rightIncrements = [1, 3, 5, 7, 1];
const downIncrements  = [1, 1, 1, 1, 2];
const scenarioCount   = rightIncrements.length;

result = 1;

for (let i = 0; i < scenarioCount; i++){
    column = 0;
    let resultFactor = 0;
    for (let line = 0; line < nLines; line+=downIncrements[i]){
        if (isThereTreeAt(line, column, inputLines)) {
            resultFactor += 1;
        }
        column += rightIncrements[i];
    }
    result *= resultFactor;
}

console.log(`Result part 2: ${result}`);

