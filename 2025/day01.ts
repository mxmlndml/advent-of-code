const TEST = false;

type Rotation = {
  direction: "left" | "right";
  distance: number;
};

const parseRotations = async (): Promise<Rotation[]> => {
  const file = Bun.file(`./${TEST ? "examples" : "inputs"}/day01.txt`);
  const text = await file.text();
  const rotations = text
    .trim()
    .split("\n")
    .map((row: string) => {
      let direction: "left" | "right";
      switch (row[0]) {
        case "L":
          direction = "left";
          break;
        case "R":
          direction = "right";
          break;
        default:
          throw new RangeError(
            `expected direction flag of either 'L' or 'R' but received '${row[0]}'`,
          );
      }

      const distance = Number.parseInt(row.slice(1, row.length));

      return {
        direction,
        distance,
      };
    });

  return rotations;
};

const part1 = async (): Promise<number> => {
  const rotations = await parseRotations();

  let password = 0;
  let dial = 50;
  for (const rotation of rotations) {
    switch (rotation.direction) {
      case "left":
        dial -= rotation.distance;
        break;

      case "right":
        dial += rotation.distance;
        break;
    }
    dial += 100;
    dial %= 100;

    if (dial === 0) {
      password++;
    }
  }

  return password;
};

const part2 = async (): Promise<number> => {
  const rotations = await parseRotations();
  let password = 0;
  let dial = 50;

  for (const rotation of rotations) {
    password += Math.floor(rotation.distance / 100);
    rotation.distance %= 100;

    switch (rotation.direction) {
      case "left":
        if (dial <= rotation.distance && dial !== 0) {
          password++;
        }

        dial -= rotation.distance;
        break;

      case "right":
        if (dial + rotation.distance >= 100) {
          password++;
        }

        dial += rotation.distance;
        break;
    }
    dial += 100;
    dial %= 100;
  }

  return password;
};

console.log(`using ${TEST ? "example" : "puzzle"} input`);
console.log("part 1:", await part1());
console.log("part 2:", await part2());
