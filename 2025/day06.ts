const TEST = false;

type Operation = {
  operator: "+" | "*";
  operands: number[];
};

const rotateText = (input: string[]): string[] => {
  const maxLength = input.reduce(
    (max, line) => (line.length > max ? line.length : max),
    0,
  );
  let output: string[] = [];
  for (let column = 0; column < maxLength; column++) {
    let line = "";
    for (let row = 0; row < input.length; row++) {
      line += input[row]![column] ?? "";
    }
    line = line.trim();
    output.push(line);
  }
  output = output.join(" ").split("  ");
  return output;
};

const parseWorksheet = async (
  direction: "ltr" | "ttb" = "ltr",
): Promise<Operation[]> => {
  const file = Bun.file(`./${TEST ? "examples" : "inputs"}/day06.txt`);
  const text = await file.text();
  const lines = text.trimEnd().split("\n");
  const operators = lines
    .at(-1)!
    .split(" ")
    .filter((s: string) => s === "+" || s === "*");
  const operations: Operation[] = operators.map((operator) => ({
    operator,
    operands: [],
  }));
  if (direction === "ltr") {
    const numbers = lines.slice(0, -1)!.flatMap((row: string) =>
      row
        .split(" ")
        .filter((s: string) => s !== "")
        .map((s: string) => Number.parseInt(s)),
    );
    numbers.forEach((number, i) => {
      operations[i % operations.length]?.operands.push(number);
    });
  }
  if (direction === "ttb") {
    const rotated = rotateText(lines.slice(0, -1)!);
    const numbers = rotated.map((row: string) =>
      row
        .split(" ")
        .filter((s: string) => s !== "")
        .map((s: string) => Number.parseInt(s)),
    );
    operations.forEach((operation, i) => {
      operation.operands = numbers[i]!;
    });
  }

  return operations;
};

const part1 = async (): Promise<number> => {
  const operations = await parseWorksheet();
  const grandTotal = operations.reduce((sum, op) => {
    if (op.operator === "+") {
      sum += op.operands.reduce((sum, num) => (sum += num));
    }
    if (op.operator === "*") {
      sum += op.operands.reduce((product, num) => (product *= num));
    }
    return sum;
  }, 0);
  return grandTotal;
};

const part2 = async () => {
  const operations = await parseWorksheet("ttb");
  const grandTotal = operations.reduce((sum, op) => {
    if (op.operator === "+") {
      sum += op.operands.reduce((sum, num) => (sum += num));
    }
    if (op.operator === "*") {
      sum += op.operands.reduce((product, num) => (product *= num));
    }
    return sum;
  }, 0);
  return grandTotal;
};

console.log(`using ${TEST ? "example" : "puzzle"} input`);
console.log("part 1:", await part1());
console.log("part 2:", await part2());
