const inputFileName = "input/day08.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

let visited = new Array(nLines);
visited.fill(false);
let pc  = 0;
let acc = 0;

while (!visited.at(pc)) {
    visited[pc] = true;
    const op  = inputLines.at(pc).split(" ").at(0);
    const arg = Number(inputLines.at(pc).split(" ").at(1));
    
    switch (op) {
      case 'jmp':
        pc  += arg;
        break;
      case 'acc':
        acc += arg;
        pc  += 1;
        break;
      case 'nop':
        pc  += 1;
        break;
      default:
        console.log("Unknown operation: " + op);
        break;
    }
}

console.log(`Result part 1: ${acc}`);

// Part 2

visited.fill(false);
pc           = 0;
acc          = 0;
let swapped      = false;
let unswapped    = false;
let pc_sv        = 0;
let acc_sv       = 0;
let visited_sv = Array.from(visited);

while (true) {
    if (pc === nLines) {
        break;
    }
    unswapped = false;
    if (visited[pc]) {
        //inf loop -> unswap
        unswapped = true;
        acc     = acc_sv;
        pc      = pc_sv;
        visited = Array.from(visited_sv);
        swapped = false;
    }
    visited[pc] = true;
    const op  = inputLines.at(pc).split(" ").at(0);
    const arg = Number(inputLines.at(pc).split(" ").at(1));
    switch (op) {
      case 'jmp':
        if (!swapped && !unswapped) {
            swapped    = true;
            pc_sv      = pc;
            acc_sv     = acc;
            visited_sv = Array.from(visited);
            
            pc  += 1;
        } else {
            pc  += arg;
        }
        break;
      case 'acc':
        acc += arg;
        pc  += 1;
        break;
      case 'nop':
        if (!swapped && !unswapped) {
            swapped    = true;
            pc_sv      = pc;
            acc_sv     = acc;
            visited_sv = Array.from(visited);
            
            pc  += arg;
        } else {
            pc  += 1;
        }
        break;
      default:
        console.log("Unknown operation: " + op);
        break;
    }
}

console.log(`Result part 2: ${acc}`);

