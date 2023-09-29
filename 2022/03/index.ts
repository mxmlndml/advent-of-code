const file = Bun.file(`${import.meta.dir}/input.txt`);
const input = await file.text();
const rows = input.split("\n");

const getDuplicates = (...strings: string[]) => {
    let duplicates: Set<string> = new Set();
    let temp: Set<string> = new Set();

    for (let i = 0; i < strings.length; i++) {
        if (i === 0) {
            for (const char of strings[0]) {
                duplicates.add(char);
            }
            continue;
        }

        for (const char of strings[i]) {
            if (duplicates.has(char)) {
                temp.add(char);
            }
        }
        duplicates = new Set(temp);
        temp.clear();
    }

    return Array.from(duplicates);
}

const getCompartments = (rucksack: string) => {
    return [rucksack.slice(0, rucksack.length / 2), rucksack.slice(-1 * rucksack.length / 2)];
}

const getPriority = (item: string) => {
    const offset = item.toUpperCase() === item ? 38 : 96;
    return item.charCodeAt(0) - offset;
}

let sum = [0, 0];
let sharedItems: string[];
let group: string[] = [];

rows.forEach((row, index) => {
    if (row === "") {
        return;
    }

    // part 1
    sharedItems = getDuplicates(...getCompartments(row));
    sum[0] += sharedItems.reduce((sum, current) => sum + getPriority(current), 0);

    // part 2
    group.push(row);

    if (index % 3 === 2) {
        const badge = getDuplicates(...group);

        sum[1] += getPriority(badge[0]);
        group = [];
    }
});

console.log(`Part 1: ${sum[0]}`);
console.log(`Part 2: ${sum[1]}`);
