const inputFileName = "input/day07.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

let result = 0;

const bags = new Map();

for (const line of inputLines) {
    const bag     = line.split("bags contain").at(0).trim();
    const content = line.split("bags contain").at(1).trim();
    const bagMap  = new Map();
    
    if (content === "no other bags.") {
        //empty bag
    } else {
        const contentWords = content.split(" ");
        for (const [wordI, word] of contentWords.entries()) {
            if (!isNaN(word)) { //word is num
                const color = contentWords.at(wordI+1) + " " + contentWords.at(wordI+2);
                bagMap.set(color, Number(word));
            }
        }
    }
    bags.set(bag, bagMap);
    console.log(bags.get(bag));
}

console.log(`Result part 1: ${result}`);

// Part 2

result = 0;


console.log(`Result part 2: ${result}`);

