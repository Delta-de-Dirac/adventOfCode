const inputFileName = "input/day01.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;


// Part 1
outer:
for (let l = 0; l < nLines-1; l++) {
    const lNumber = Number(inputLines.at(l));
    for (let r = l+1; r < nLines; r++) {
        const rNumber = Number(inputLines.at(r));
        if (lNumber + rNumber == 2020) {
            const result = lNumber * rNumber;
            console.log(`Result part 1: ${result}`)
            break outer;
        }
    }
} 

// Part 2
outer:
for (let l = 0; l < nLines-2; l++) {
    const lNumber = Number(inputLines.at(l));
    for (let m = l+1; m < nLines-1; m++) {
        const mNumber = Number(inputLines.at(m));
        if (lNumber + mNumber > 2020) {
            continue;
        }
        for (let r = m+1; r < nLines; r++) {
            const rNumber = Number(inputLines.at(r));
            if (lNumber + mNumber + rNumber == 2020) {
                const result = lNumber * mNumber * rNumber;
                console.log(`Result part 2: ${result}`)
                break outer;
            }
        }
    }
} 
