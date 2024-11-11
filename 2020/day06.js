const inputFileName = "input/day06.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

let result = 0;

let currentSet = new Set();

for (const line of inputLines) {
    if (line.trim() === "") {
        result += currentSet.size;
        currentSet.clear();
    } else {
        for(const char of line) {
            currentSet.add(char);
        }
    }
}
result += currentSet.size;
currentSet.clear();

console.log(`Result part 1: ${result}`);

// Part 2

result = 0;

let currentSetArray = new Array();

for (const line of inputLines) {
    if (line.trim() === "") {
        let intersectionSet = currentSetArray.at(0);
        
        for (let i = 1; i < currentSetArray.length; i++){
            intersectionSet = intersectionSet.intersection(
                currentSetArray.at(i)
            );
        }
        
        result += intersectionSet.size;
        
        currentSetArray = new Array();
    } else {
        let currentSet = new Set();
        for(const char of line) {
            currentSet.add(char);
        }
        currentSetArray.push(currentSet);
    }
}
let intersectionSet = currentSetArray.at(0);
for (let i = 1; i < currentSetArray.length; i++){
    intersectionSet = intersectionSet.intersection(
        currentSetArray.at(i)
    );
}
result += intersectionSet.size;
currentSetArray = new Array();

console.log(`Result part 2: ${result}`);

