const inputFileName = "input/day11.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

function countTotalOccupied(map) {
    var count = 0;
    for (var l = 0; l < map.length; l++) {
        for (var c = 0; c < map[l].length; c++) {
            if (getMap(map, l, c) === '#') {
                count += 1;
            }
        }
    }
    return count;
}

function mapCmp(map1, map2) {
    for (var l = 0; l < map1.length; l++) {
        for (var c = 0; c < map1[l].length; c++) {
            if (map1[l][c] !== map2[l][c]) {
                return false;
            }
        }
    }
    return true;
}

function copyMap(map) {
    var newMap = [];
    for (var l = 0; l < map.length; l++) {
        newMap.push([]);
        for (var c = 0; c < map[l].length; c++) {
            newMap[l].push(map[l][c]);
        }
    }
    return newMap;
}

function linesToMap(lines) {
    var map = [];
    for (var l = 0; l < lines.length; l++) {
        map.push([]);
        for (var c = 0; c < lines[l].length; c++) {
            map[l].push(lines[l][c]);
        }
    }
    return map;
}

function getMap(map, l, c) {
    if (l < 0) {
        return '.';
    }
    if (l >= map.length) {
        return '.'
    }
    if (c < 0) {
        return '.';
    }
    if (c >= map[0].length) {
        return '.';
    }
    return map[l][c];
}

function countOccupiedNeighbors(map, l, c){
    var count = 0;
    for (var i = -1; i < 2; i++) {
        for (var j = -1; j < 2; j++){
            if (i === 0 && j === 0) {
                continue;
            }
            if (getMap(map, l+i, c+j) === '#'){
                count += 1;
            }
        }
    }
    return count;
}

function nextState(map) {
    var newMap = [];
    for (var l = 0; l < map.length; l++){
        newMap.push([]);
        for (var c = 0; c < map[0].length; c++){
            newMap[l].push('.');
        }
    }

    for (var l = 0; l < map.length; l++){
        for (var c = 0; c < map[0].length; c++){
            if (map[l][c] === 'L' && countOccupiedNeighbors(map, l, c) === 0) {
                newMap[l][c] = '#';
            }
            else if (map[l][c] === '#' && countOccupiedNeighbors(map, l, c) >= 4) {
                newMap[l][c] = 'L';
            }
            else {
                newMap[l][c] = map[l][c]
            }
        }
    }
    return newMap;
}

var map = linesToMap(inputLines);

while (true) {
    const oldMap = copyMap(map);
    map = nextState(map);
    if (mapCmp(map, oldMap)) {
        break;
    }
}

var result = countTotalOccupied(map);

console.log(`Result part 1: ${result}`);

// Part 2

function countOccupiedNeighbors2(map, l, c){
    var count = 0;
    for (var i = -1; i < 2; i++) {
        for (var j = -1; j < 2; j++){
            if (i === 0 && j === 0) {
                continue;
            }
            var sightl = l+i;
            var sightc = c+j;
            outer:
            while (true) {
                if (sightl >= map.length    ||
                    sightl <  0             ||
                    sightc >= map[0].length ||
                    sightc <  0             ) {
                        break outer;
                } else if (getMap(map, sightl, sightc) === "."){
                    sightl += i;
                    sightc += j;
                    continue outer
                } else if (getMap(map, sightl, sightc) === "#") {
                    count += 1;
                    break outer;
                } else if (getMap(map, sightl, sightc) === "L") {
                    break outer;
                }
            }
        }
    }
    return count;
}

function nextState2(map) {
    var newMap = [];
    for (var l = 0; l < map.length; l++){
        newMap.push([]);
        for (var c = 0; c < map[0].length; c++){
            newMap[l].push('.');
        }
    }

    for (var l = 0; l < map.length; l++){
        for (var c = 0; c < map[0].length; c++){
            if (map[l][c] === 'L' && countOccupiedNeighbors2(map, l, c) === 0) {
                newMap[l][c] = '#';
            }
            else if (map[l][c] === '#' && countOccupiedNeighbors2(map, l, c) >= 5) {
                newMap[l][c] = 'L';
            }
            else {
                newMap[l][c] = map[l][c]
            }
        }
    }
    return newMap;
}

var map = linesToMap(inputLines);

while (true) {
    const oldMap = copyMap(map);
    map = nextState2(map);
    if (mapCmp(map, oldMap)) {
        break;
    }
}

var result = countTotalOccupied(map);

console.log(`Result part 2: ${result}`);

