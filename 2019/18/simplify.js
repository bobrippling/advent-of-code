const fs = require('fs');
const input = fs.readFileSync('./input', 'utf8').split('\n').map(x => x.split(""));

let modifiedInput = [...input];

while (true) {
    let counter = 0;

    for (let y = 0; y < modifiedInput.length; y++) {
        for (let x = 0; x < modifiedInput[y].length; x++) {
            if (modifiedInput[y][x] === '.' || modifiedInput[y][x].match(/[A-Z]/)) {
                const surroundings = [
                    modifiedInput[y + 1][x],
                    modifiedInput[y - 1][x],
                    modifiedInput[y][x + 1],
                    modifiedInput[y][x - 1],
                ];

                if (surroundings.filter(x => x === '#').length === 3) {
                    modifiedInput[y][x] = '#';
                    counter++;
                }
            }
        }
    }

    if (counter === 0) {
        break;
    }
}

fs.writeFile(
    'testout.txt',
    modifiedInput.map(x => x.join('')).join('\n'),
    'utf8',
    err => {
        if (err) {
            console.error(err);
            process.exit(1);
        }
    }
)
