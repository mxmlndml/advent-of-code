const file = Bun.file(`${import.meta.dir}/test.txt`);
const input = await file.text();
const calories = [];
const rows = input.split("\n");
// rows.push("");

let currentCalories = 0;
let mostCaloriesPerElf = 0;
for (const row of rows) {
    if (row === "") {
        calories.push(currentCalories);

        if (mostCaloriesPerElf < currentCalories) {
            mostCaloriesPerElf = currentCalories;
        }

        currentCalories = 0;
        continue;
    }

    currentCalories += Number(row);
}

const [mostCalories, secondMostCalories, thirdMostCalories] = calories.sort((a, b) => b - a);
const topThreeCalories = mostCalories + secondMostCalories + thirdMostCalories;

console.log(`Part one: ${mostCalories}`);
console.log(`Part two: ${topThreeCalories}`);
