const TEST = false;

const parseTachyonManifold = async (): Promise<string[][]> => {
  const file = Bun.file(`./${TEST ? "examples" : "inputs"}/day07.txt`);
  const text = await file.text();
  const grid = text
    .trim()
    .split("\n")
    .map((line) => line.split(""));
  return grid;
};

const part1 = async (): Promise<number> => {
  const manifold = await parseTachyonManifold();
  let splits = 0;
  manifold.forEach((line, row) => {
    if (row === manifold.length - 1) {
      return;
    }
    line.forEach((char, column) => {
      if (char === "S" || char === "|") {
        if (manifold[row + 1]![column] === ".") {
          manifold[row + 1]![column] = "|";
        }
        if (manifold[row + 1]![column] === "^") {
          splits++;
          if (column > 0 && manifold[row + 1]![column - 1] === ".") {
            manifold[row + 1]![column - 1] = "|";
          }
          if (
            column < line.length - 1 &&
            manifold[row + 1]![column + 1] === "."
          ) {
            manifold[row + 1]![column + 1] = "|";
          }
        }
      }
    });
  });
  return splits;
};

const part2 = async (): Promise<number> => {
  const manifold = await parseTachyonManifold();
  const counts = Array.from({ length: manifold.length }, (_, i) =>
    Array.from({ length: manifold[i]!.length }, () => 0),
  );
  manifold.forEach((line, row) => {
    if (row === manifold.length - 1) {
      return;
    }
    line.forEach((char, column) => {
      if (char === "S" || char === "|") {
        if (char === "S") {
          counts[row]![column] = 1;
        }
        if (manifold[row + 1]![column] !== "^") {
          manifold[row + 1]![column] = "|";
          counts[row + 1]![column]! += counts[row]![column]!;
        }
        if (manifold[row + 1]![column] === "^") {
          if (column > 0 && manifold[row + 1]![column - 1] !== "^") {
            manifold[row + 1]![column - 1] = "|";
            counts[row + 1]![column - 1]! += counts[row]![column]!;
          }
          if (
            column < line.length - 1 &&
            manifold[row + 1]![column + 1] !== "^"
          ) {
            manifold[row + 1]![column + 1] = "|";
            counts[row + 1]![column + 1]! += counts[row]![column]!;
          }
        }
      }
    });
  });
  const timelines = counts.at(-1)!.reduce((acc, curr) => (acc += curr), 0);
  return timelines;
};

console.log(`using ${TEST ? "example" : "puzzle"} input`);
console.log("part 1:", await part1());
console.log("part 2:", await part2());
