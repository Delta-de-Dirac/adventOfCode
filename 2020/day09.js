const inputFileName = "input/day09.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

const preambleLen = 25;
const numberList = new Array();

let currentLine = 0;
let target = 0;

// load preamble 
while (currentLine < preambleLen) {
    numberList.push(Number(inputLines.at(currentLine)));
    currentLine += 1;
}

// LABEL0:
outer:
while (true) {
    // load next number 
    const nextNumber = Number(inputLines.at(currentLine));
    currentLine += 1;
    
    // test if number is ok
    for (let i = 0; i < preambleLen-1; i++){
        for (let j = i + 1; j < preambleLen; j++){
            const lNum = numberList.at(i);
            const rNum = numberList.at(j);
            if (lNum == rNum) {
                continue;
            } else if (lNum + rNum == nextNumber) {         
                // append number to list
                numberList.push(nextNumber);
                
                // remove list head
                numberList.shift();
                
                // GOTO LABEL0
                continue outer;
            }
        }
    }
    target = nextNumber;
    break;
}

console.log(`Result part 1: ${target}`);

// Part 2

const weaknessList = new Array();
let sum = 0;
currentLine = 0;

while (true) {
    // load next number
    const nextNumber = Number(inputLines.at(currentLine));
    currentLine += 1;
    
    // Add next number to list
    weaknessList.push(nextNumber);
    
    // Update sum
    sum += nextNumber;
    
    // Remove head until sum less or equal to target
    while (sum > target) {
            const removed = weaknessList.shift();
            sum -= removed;
    }
    
    // if equal finish
    if (sum === target) {
        break;
    }
    
    // else loop
}

const result = Math.min(...weaknessList) + Math.max(...weaknessList);

console.log(`Result part 2: ${result}`);

