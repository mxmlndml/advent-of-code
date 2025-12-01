const getPassword = async (): Promise<number> => {
  const file = Bun.file("inputs/day01.txt");
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

console.log(await getPassword());
