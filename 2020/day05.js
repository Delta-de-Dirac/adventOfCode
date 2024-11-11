const inputFileName = "input/day05.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

let result = 0;

for (const line of inputLines) {
    let id = 0;
    for (const charI in line) {
        const char = line[charI];
        if (char === "B" || char === "R"){
            id += 0b1 << (9-charI);
        }
    }
    result = id > result ? id : result;
}

console.log(`Result part 1: ${result}`);

// Part 2

result = 0;

const seats = new Set();

for (const line of inputLines) {
    let id = 0;
    for (const charI in line) {
        const char = line[charI];
        if (char === "B" || char === "R"){
            id += 0b1 << (9-charI);
        }
    }
    seats.add(id);
}

for (let i = 0; i < 1023; i++){
    if (seats.has(i-1) && seats.has(i+1) && !seats.has(i)){
        result = i;
        break;
    }
}

console.log(`Result part 2: ${result}`);

