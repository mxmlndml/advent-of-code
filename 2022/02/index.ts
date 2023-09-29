const file = Bun.file(`${import.meta.dir}/input.txt`);
const input = await file.text();
const rows = input.split("\n");

const scores = [0, 0];

for (const row of rows) {
    if (row === "") {
        break;
    }

    const opponentsShape = row.charCodeAt(0) - 64;
    const secondInput = row.charCodeAt(2) - 87;

    // part 1:
    // second input is my shape
    // X: rock, Y: paper, Z: scissors

    // shape score
    // X - 87 = 88 - 87 = 1
    // Y - 87 = 89 - 87 = 2
    // Z - 87 = 90 - 87 = 3
    scores[0] += secondInput;

    // outcome score
    // ... 3: Scissors | 1: Rock - 2: Paper - 3: Scissors | 1: Rock ...
    // all shapes are beaten by their right neighbor and beat their left neighbor
    scores[0] += (secondInput - opponentsShape + 1 + 3) % 3 * 3;


    // part 2:
    // second input is outcome
    // X: lose, Y: draw, Z: lose

    // outcome score
    // (X - 87 - 1) * 3 = (88 - 87 - 1) * 3 = (1 - 1) * 3 = 0 * 3 = 0
    // (Y - 87 - 1) * 3 = (89 - 87 - 1) * 3 = (2 - 1) * 3 = 1 * 3 = 3
    // (Z - 87 - 1) * 3 = (90 - 87 - 1) * 3 = (3 - 1) * 3 = 2 * 3 = 6
    scores[1] += (secondInput - 1) * 3;

    // shape score
    // opponentsShape - 1 ->  0..2
    // secondInput - 2    -> -1..1
    // + 3) % 3
    scores[1] += (opponentsShape - 1 + secondInput - 2 + 3) % 3 + 1;
}

console.log(`Part one: ${scores[0]}`);
console.log(`Part two: ${scores[1]}`);
