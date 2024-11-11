const inputFileName = "input/day04.txt";
const file          = Bun.file(inputFileName);
const inputContent  = await file.text();
const inputLines    = inputContent.trim().split("\n");
const nLines        = inputLines.length;

// Part 1

const requiredFields = new Set([
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid"
])

const fields = new Set();

let result = 0;

for (let line of inputLines){
    if (line.trim() === "") {
        if (requiredFields.isSubsetOf(fields)) {
            result += 1;
        }
        fields.clear();
    } else {
        for (let word of line.split(" ")) {
            fields.add(word.split(":").at(0));
        }
    }
}

if (requiredFields.isSubsetOf(fields)) {
    result += 1;
}

fields.clear();

console.log(`Result part 1: ${result}`);

// Part 2

result = 0;

for (let line of inputLines){
    if (line.trim() === "") {
        if (requiredFields.isSubsetOf(fields)) {
            result += 1;
        }
        fields.clear();
    } else {
        for (let word of line.split(" ")) {

            const fieldKey = word.split(":").at(0).trim();
            const fieldVal = word.split(":").at(1).trim();
            switch (fieldKey) {
                case "byr":
                    const byr = Number(fieldVal);
                    if (byr >= 1920 && byr <= 2002){
                        fields.add(fieldKey);
                    }
                    break;
                case "iyr":
                    const iyr = Number(fieldVal);
                    if (iyr >= 2010 && iyr <= 2020){
                        fields.add(fieldKey);
                    }
                    break;
                case "eyr":
                    const eyr = Number(fieldVal);
                    if (eyr >= 2020 && eyr <= 2030){
                        fields.add(fieldKey);
                    }
                    break;
                case "hgt":
                    const hgtNum  = Number(fieldVal.slice(0, -2));
                    const hgtUnit = fieldVal.slice(-2);
                    if (hgtUnit === "cm") {
                        if (hgtNum >= 150 && hgtNum <= 193){
                            fields.add(fieldKey);
                        }
                    } else if (hgtUnit === "in") {
                        if (hgtNum >= 59 && hgtNum <= 76){
                            fields.add(fieldKey);
                        }
                    }
                    break;
                case "hcl":
                    if (fieldVal.at(0) !== '#'){
                        break;
                    }

                    const validDigits = new Set([
                        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                        'a', 'b', 'c', 'd', 'e', 'f'
                    ]);

                    for (let i = 1; i < 7; i++) {
                        if (!validDigits.has(fieldVal.at(i))){
                            break;
                        }
                    }
                    fields.add(fieldKey);
                    break;
                case "ecl":
                    const validEcl = new Set([
                        "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
                    ]);
                    if (validEcl.has(fieldVal)) {
                        fields.add(fieldKey);
                    }
                    break;
                case "pid":
                    const validDigitsPid = new Set([
                        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
                    ]);
                    if (fieldVal.length !== 9){
                        break;
                    }
                    for (let i = 0; i < fieldVal.length; i++) {
                        if (!validDigitsPid.has(fieldVal.at(i))){
                            break;
                        }
                    }
                    fields.add(fieldKey);
                    break;
                case "cid":
                    //pass
                    break;
                default:
                    console.log("unknown field: " + fieldKey);
                    break;
            }
        }
    }
}

if (requiredFields.isSubsetOf(fields)) {
    result += 1;
}

fields.clear();

console.log(`Result part 2: ${result}`);

