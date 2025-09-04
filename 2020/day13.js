const inputFileName = "input/day13.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// part 1

const earliest = Number(inputLines[0])

const busList = inputLines[1]
                .split(',')
                .filter(x=>x!='x')
                .map(Number)

var leastWaitTimeID = busList[0]
var leastWaitTime   = leastWaitTimeID - earliest%leastWaitTimeID;
for (var i = 1; i < busList.length; i++) {
    const busID    = busList[i];
    const waitTime = busID - earliest%busID;
    if (waitTime < leastWaitTime) {
        leastWaitTimeID = busID;
        leastWaitTime   = waitTime;
    }
}
const result = leastWaitTimeID * leastWaitTime;
console.log(`Result part 1: ${result}`);

