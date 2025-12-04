const TEST = false;

const parseMap = async (): Promise<string[][]> => {
  const file = Bun.file(`./${TEST ? "examples" : "inputs"}/day04.txt`);
  const text = await file.text();
  const map = text
    .trim()
    .split("\n")
    .map((row: string) => row.split(""));
  return map;
};

const getAccessibleRolls = (map: string[][], autoremove = false): number => {
  let accessibleRolls = 0;
  for (let y = 0; y < map.length; y++) {
    const row = map[y]!;
    for (let x = 0; x < row.length; x++) {
      const position = row[x]!;
      if (position !== "@") {
        continue;
      }

      let adjacentRolls = 0;
      const isTopRow = y === 0;
      const isBottomRow = y === map.length - 1;
      const isLeftColumn = x === 0;
      const isRightColumn = x === row.length - 1;
      // top left
      if (!isTopRow && !isLeftColumn && map[y - 1]![x - 1] === "@") {
        adjacentRolls++;
      }
      // top
      if (!isTopRow && map[y - 1]![x] === "@") {
        adjacentRolls++;
      }
      // top right
      if (!isTopRow && !isRightColumn && map[y - 1]![x + 1] === "@") {
        adjacentRolls++;
      }
      // left
      if (!isLeftColumn && map[y]![x - 1] === "@") {
        adjacentRolls++;
      }
      // right
      if (!isRightColumn && map[y]![x + 1] === "@") {
        adjacentRolls++;
      }
      // bottom left
      if (!isBottomRow && !isLeftColumn && map[y + 1]![x - 1] === "@") {
        adjacentRolls++;
      }
      // bottom
      if (!isBottomRow && map[y + 1]![x] === "@") {
        adjacentRolls++;
      }
      // bottom right
      if (!isBottomRow && !isRightColumn && map[y + 1]![x + 1] === "@") {
        adjacentRolls++;
      }

      if (adjacentRolls < 4) {
        accessibleRolls++;
        if (autoremove) {
          map[y]![x] = "x";
        }
      }
    }
  }

  return accessibleRolls;
};

const part1 = async (): Promise<number> => {
  const map = await parseMap();
  const accessibleRolls = getAccessibleRolls(map);
  return accessibleRolls;
};

const part2 = async (): Promise<number> => {
  const map = await parseMap();

  let previousAccessibleRolls = -1;
  let accessibleRolls = 0;
  while (previousAccessibleRolls !== accessibleRolls) {
    previousAccessibleRolls = accessibleRolls;
    accessibleRolls += getAccessibleRolls(map, true);
  }

  return accessibleRolls;
};

console.log(`using ${TEST ? "example" : "puzzle"} input`);
console.log("part 1:", await part1());
console.log("part 2:", await part2());
