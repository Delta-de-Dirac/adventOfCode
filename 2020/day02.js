const inputFileName = "input/day02.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1
let validCount = 0;

for (let i = 0; i < nLines; i++) {
    const password        = inputLines[i].split(': ').at(1);
    const lowerBound      = Number(inputLines[i].split('-').at(0));
    const upperBound      = Number(inputLines[i].split('-').at(1).split(' ').at(0));
    const policyChar      = inputLines[i].split(' ').at(1).at(0);
    const policyCharCount = [...password].reduce(
        (acc, x) => {
            return acc + (x == policyChar ? 1 : 0);
        },
        0
    );
    if (policyCharCount >= lowerBound && policyCharCount <= upperBound) {
        validCount += 1;
    }
}

console.log(`Result part 1: ${validCount}`)

// Part 2

validCount = 0;

for (let i = 0; i < nLines; i++) {
    const password        = inputLines[i].split(': ').at(1);
    const lowerIndex      = Number(inputLines[i].split('-').at(0));
    const upperIndex      = Number(inputLines[i].split('-').at(1).split(' ').at(0));
    const policyChar      = inputLines[i].split(' ').at(1).at(0);
    if (
        password.at(lowerIndex-1) === policyChar && password.at(upperIndex-1) !== policyChar ||
        password.at(lowerIndex-1) !== policyChar && password.at(upperIndex-1) === policyChar ) 
    {
        validCount += 1;
    }
}

console.log(`Result part 2: ${validCount}`)
