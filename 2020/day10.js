const inputFileName = "input/day10.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

var skip1 = 0;
var skip3 = 0;

const inputNums = inputLines.map(Number).toSorted((a,b)=>(a-b));

if (inputNums[0] === 1) {
    skip1 +=1;
} else if (inputNums[0] === 2) {
} else if (inputNums[0] === 3) {
    skip3 +=1;
} else {
    console.log("Can't adapt correctly.");
    console.log(inputNums[0]);
    throw new Error();
}

for (var i = 0; i < inputNums.length; i++) {
    if (i === inputNums.length - 1) {
        skip3 += 1;
        continue;
    } else if (inputNums[i+1] === inputNums[i]) {
        continue;
    } else if (inputNums[i+1] === inputNums[i] + 1) {
        skip1 += 1;
        continue;
    } else if (inputNums[i+1] === inputNums[i] + 2) {
        continue;
    } else if (inputNums[i+1] === inputNums[i] + 3) {
        skip3 += 1;
        continue;
    } else {
        console.log("Can't adapt correctly.");
        console.log(inputNums);
        console.log(`Current adapter = ${inputNums[i]}`);
        console.log(`Next adapter = ${inputNums[i+1]}`);
        throw new Error();
    }
}

var result = skip1 * skip3;

console.log(`Result part 1: ${result}`);

// Part 2

const memo = {};

function countAdapters(theList, n) {
    if (memo.hasOwnProperty(n)) {
        return memo[n];
    }
    var ways = 0;
    if (n === 0 || n === 1 || n === 2){
        if (theList[n] <= 3) {
            ways += 1;
        }
    }
    if (n > 0) {
        if (theList[n-1] >= theList[n] - 3) {
            ways += countAdapters(theList, n-1);
        }
    }
    if (n > 1) {
        if (theList[n-2] >= theList[n] - 3) {
            ways += countAdapters(theList, n-2);
        }
    }
    if (n > 2) {
        if (theList[n-3] >= theList[n] - 3) {
            ways += countAdapters(theList, n-3);
        }
    }
    memo[n] = ways;
    return ways;
}

result = countAdapters(inputNums, inputNums.length - 1);

console.log(`Result part 2: ${result}`);

