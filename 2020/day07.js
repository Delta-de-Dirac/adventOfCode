const inputFileName = "input/day07.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

let result = 0;

const bags = new Map();

const checkContainShinyGoldSet = new Set();


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
    checkContainShinyGoldSet.add(bag);
}

const containShinyGold = new Set();

function verifyBag(bag, bagMap, checkSet, containShinyGoldSet) {
  if (!checkSet.has(bag)) {
      return false;
  }
  
  checkSet.delete(bag);
  
  for (const bagInside of bagMap.get(bag).keys()){
      if (bagInside === 'shiny gold') {
          containShinyGoldSet.add(bag);
          return true;
      } else if (containShinyGoldSet.has(bagInside)) {
          containShinyGoldSet.add(bag);
          return true;
      } else {
          if (verifyBag(bagInside, bagMap, checkSet, containShinyGoldSet)) {
              containShinyGoldSet.add(bag);
              return true;
          } else {
              continue;
          }
      }
  }
  return false;
}

while (checkContainShinyGoldSet.size !== 0) {
    const iterator = checkContainShinyGoldSet.values()
    const nextVal = iterator.next().value
    verifyBag(nextVal, bags, checkContainShinyGoldSet, containShinyGold);
}

result = containShinyGold.size;

console.log(`Result part 1: ${result}`);

// Part 2

const countingMemo = new Map();

function countInside(bag, bagMap) {
    if (countingMemo.has(bag)) {
        return countingMemo.get(bag);
    } else {
        let count = 0;
        for (const [bagInside, bagInsideCount] of bagMap.get(bag)) {
            count += bagInsideCount*(countInside(bagInside, bagMap) + 1);
        }
        countingMemo.set(bag, count);
        return count;
    }
}


result = countInside("shiny gold", bags);


console.log(`Result part 2: ${result}`);

