const TEST = false;

type Database = {
  freshIngredientIDRanges: Range[];
  availableIngredientIDs: number[];
};
type Range = {
  min: number;
  max: number;
};

const parseDatabase = async (): Promise<Database> => {
  const file = Bun.file(`./${TEST ? "examples" : "inputs"}/day05.txt`);
  const text = await file.text();
  const [freshIDRanges, availableIDs] = text.trim().split("\n\n");

  return {
    freshIngredientIDRanges: freshIDRanges!
      .split("\n")
      .map((rawRange: string) => {
        const [min, max] = rawRange
          .split("-")
          .map((s: string) => Number.parseInt(s));
        return {
          min: min!,
          max: max!,
        };
      }),
    availableIngredientIDs: availableIDs!
      .split("\n")
      .map((s: string) => Number.parseInt(s)),
  };
};

const mergeRanges = (ranges: Range[]): Range[] => {
  const mergedRanges: Range[] = [];
  for (const newRange of ranges) {
    let merged = false;
    for (let i = 0; i < mergedRanges.length; i++) {
      const oldRange = mergedRanges[i]!;

      // [=====] old
      //  [===]  new
      if (
        oldRange.min <= newRange.min &&
        newRange.min <= newRange.max &&
        newRange.max <= oldRange.max
      ) {
        merged = true;
        break;
      }
      //  [===]  old
      // [=====] new
      if (
        newRange.min <= oldRange.min &&
        oldRange.min <= oldRange.max &&
        oldRange.max <= newRange.max
      ) {
        oldRange.min = newRange.min;
        oldRange.max = newRange.max;
        merged = true;
        break;
      }
      // [===]   old
      //   [===] new
      if (
        oldRange.min <= newRange.min &&
        newRange.min <= oldRange.max &&
        oldRange.max <= newRange.max
      ) {
        oldRange.max = newRange.max;
        merged = true;
        break;
      }
      //   [===] old
      // [===]   new
      if (
        newRange.min <= oldRange.min &&
        oldRange.min <= newRange.max &&
        newRange.max <= oldRange.max
      ) {
        oldRange.min = newRange.min;
        merged = true;
        break;
      }
    }
    if (!merged) {
      mergedRanges.push(newRange);
    }
  }
  return mergedRanges;
};

const part1 = async (): Promise<number> => {
  const db = await parseDatabase();

  let freshIngredients = 0;
  for (const ingredient of db.availableIngredientIDs) {
    for (const range of db.freshIngredientIDRanges) {
      if (ingredient >= range.min && ingredient <= range.max) {
        freshIngredients++;
        break;
      }
    }
  }
  return freshIngredients;
};

const part2 = async (): Promise<number> => {
  const db = await parseDatabase();

  let oldRangesLength = -1;
  let mergedRanges = db.freshIngredientIDRanges;
  while (oldRangesLength !== mergedRanges.length) {
    oldRangesLength = mergedRanges.length;
    mergedRanges = mergeRanges(mergedRanges);
  }

  const freshIngredientIDs = mergedRanges.reduce(
    (acc, range) => (acc += range.max - range.min + 1),
    0,
  );
  return freshIngredientIDs;
};

console.log(`using ${TEST ? "example" : "puzzle"} input`);
console.log("part 1:", await part1());
console.log("part 2:", await part2());
