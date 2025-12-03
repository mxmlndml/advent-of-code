const TEST = false;

type Battery = {
  joltage: number;
  index: number;
};

const parseBanks = async (): Promise<string[]> => {
  const file = Bun.file(`./${TEST ? "examples" : "inputs"}/day03.txt`);
  const text = await file.text();
  const banks = text.trim().split("\n");
  return banks;
};

const getLargestBattery = (joltages: number[]): Battery => {
  if (joltages.length < 1) {
    throw new RangeError("joltages must not be empty");
  }

  const battery = {
    joltage: -1,
    index: -1,
  };

  for (let index = 0; index < joltages.length; index++) {
    const joltage = joltages[index]!;
    if (joltage === 9) {
      return {
        joltage,
        index,
      };
    }
    if (joltage > battery.joltage) {
      battery.joltage = joltage;
      battery.index = index;
    }
  }
  return battery;
};

const part1 = async (): Promise<number> => {
  const banks = await parseBanks();

  let totalJoltage = 0;
  for (const bank of banks) {
    const batteries = bank.split("").map((s: string) => Number.parseInt(s));

    const leftBatteries = batteries.slice(0, bank.length - 1);
    const leftBattery = getLargestBattery(leftBatteries);

    const rightBatteries = batteries.slice(leftBattery.index + 1);
    const rightBattery = getLargestBattery(rightBatteries);

    const joltage = leftBattery.joltage * 10 + rightBattery.joltage;
    totalJoltage += joltage;
  }

  return totalJoltage;
};

const part2 = async (): Promise<number> => {
  const banks = await parseBanks();

  let totalJoltage = 0;
  for (const bank of banks) {
    let batteries = bank.split("").map((s: string) => Number.parseInt(s));

    for (let i = 11; i >= 0; i--) {
      const possibleBatteries = batteries.slice(0, batteries.length - i);
      const largestBattery = getLargestBattery(possibleBatteries);
      totalJoltage += largestBattery.joltage * 10 ** i;
      batteries = batteries.slice(largestBattery.index + 1);
    }
  }

  return totalJoltage;
};

console.log(`using ${TEST ? "example" : "puzzle"} input`);
console.log("part 1:", await part1());
console.log("part 2:", await part2());
