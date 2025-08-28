const inputFileName = "input/day12.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

let rotRight = ([x, y]) => [y, -x];
let rotLeft  = ([x, y]) => [-y, x];

var pos = [0, 0];
var dir = [1, 0];

for (var i = 0; i < nLines; i++){
    const command = inputLines[i].substring(0,1)
    const number  = Number(inputLines[i].substring(1))
    const turnCount = Math.floor((number % 360)/90)

    switch (command) {
        case "E":
            pos[0] += number;
            break;
        case "S":
            pos[1] -= number;
            break;
        case "W":
            pos[0] -= number;
            break;
        case "N":
            pos[1] += number;
            break;
        case "L":
            for (var j = 0; j < turnCount; j++){
                dir = rotLeft(dir);
            }
            break;
        case "R":
            for (var j = 0; j < turnCount; j++){
                dir = rotRight(dir);
            }
            break;
        case "F":
            pos[0] += dir[0] * number
            pos[1] += dir[1] * number
            break;
        default:
            console.log(`Command ${command} Unknown.`);
    }
}

const result = Math.abs(pos[0]) + Math.abs(pos[1])

console.log(`Result part 1: ${result}`);

// Part 2

var ship_pos = [0,  0]
var wp_pos   = [10, 1]

for (var i = 0; i < nLines; i++){
    const command = inputLines[i].substring(0,1)
    const number  = Number(inputLines[i].substring(1))
    const turnCount = Math.floor((number % 360)/90)

    switch (command) {
        case "E":
            wp_pos[0] += number;
            break;
        case "S":
            wp_pos[1] -= number;
            break;
        case "W":
            wp_pos[0] -= number;
            break;
        case "N":
            wp_pos[1] += number;
            break;
        case "L":
            for (var j = 0; j < turnCount; j++){
                wp_pos = rotLeft(wp_pos);
            }
            break;
        case "R":
            for (var j = 0; j < turnCount; j++){
                wp_pos = rotRight(wp_pos);
            }
            break;
        case "F":
            ship_pos[0] += wp_pos[0] * number
            ship_pos[1] += wp_pos[1] * number
            break;
        default:
            console.log(`Command ${command} Unknown.`);
    }
}

const result2 = Math.abs(ship_pos[0]) + Math.abs(ship_pos[1])

console.log(`Result part 2: ${result2}`);

