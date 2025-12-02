const TEST = false;

type IDRange = {
  lower: number;
  upper: number;
};

const parseIDRanges = async (): Promise<IDRange[]> => {
  const file = Bun.file(`./${TEST ? "examples" : "inputs"}/day02.txt`);
  const text = await file.text();
  const idRanges = text
    .trim()
    .split(",")
    .map((idRange: string) => {
      const [lower, upper] = idRange
        .split("-")
        .map((id: string) => Number.parseInt(id));
      return {
        lower: lower!,
        upper: upper!,
      };
    });

  return idRanges;
};

const part1 = async (): Promise<number> => {
  const idRanges = await parseIDRanges();

  let sum = 0;
  for (const idRange of idRanges) {
    for (let n = idRange.lower; n <= idRange.upper; n++) {
      const s = n.toString();
      if (s.length % 2 === 1) {
        continue;
      }
      const firstHalf = s.substring(0, s.length / 2);
      const secondHalf = s.substring(s.length / 2);
      if (firstHalf === secondHalf) {
        sum += n;
      }
    }
  }
  return sum;
};

const part2 = async (): Promise<number> => {
  const idRanges = await parseIDRanges();

  let sum = 0;
  for (const idRange of idRanges) {
    for (let n = idRange.lower; n <= idRange.upper; n++) {
      const s = n.toString();

      for (
        let sequenceLength = 1;
        sequenceLength <= s.length / 2;
        sequenceLength++
      ) {
        if (s.length % sequenceLength !== 0) {
          continue;
        }

        const sequenced = s
          .substring(0, sequenceLength)
          .repeat(s.length / sequenceLength);
        if (s === sequenced) {
          sum += n;
          break;
        }
      }
    }
  }
  return sum;
};

console.log(`using ${TEST ? "example" : "puzzle"} input`);
console.log("part 1:", await part1());
console.log("part 2:", await part2());
