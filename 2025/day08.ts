const TEST = false;

interface JunctionBox {
  id: number;
  x: number;
  y: number;
  z: number;
  circuit: number;
  isSingle(): boolean;
}
type Connection = {
  start: JunctionBox;
  end: JunctionBox;
  distance: number;
};

const parsePositions = async (): Promise<JunctionBox[]> => {
  const file = Bun.file(`./${TEST ? "examples" : "inputs"}/day08.txt`);
  const text = await file.text();
  const lines = text.trim().split("\n");
  return lines.map((line, i) => {
    const [x, y, z] = line.split(",").map((s) => Number.parseInt(s));
    return {
      id: i,
      x: x!,
      y: y!,
      z: z!,
      circuit: -1,
      isSingle() {
        return this.circuit === -1;
      },
    };
  });
};

const getDistance = (a: JunctionBox, b: JunctionBox): number =>
  Math.sqrt((a.x - b.x) ** 2 + (a.y - b.y) ** 2 + (a.z - b.z) ** 2);

const getConnections = (jbs: JunctionBox[]): Connection[] => {
  const connections: Connection[] = [];
  for (let i = 0; i < jbs.length - 1; i++) {
    const start = jbs[i]!;
    for (let j = i + 1; j < jbs.length; j++) {
      const end = jbs[j]!;

      connections.push({
        start,
        end,
        distance: getDistance(start, end),
      });
    }
  }
  return connections.sort((a, b) => a.distance - b.distance);
};

const part1 = async (): Promise<number> => {
  const junctionBoxes = await parsePositions();
  const connections = getConnections(junctionBoxes);
  const circuits: JunctionBox[][] = [];

  let limit = TEST ? 10 : 1000;
  for (let n = 0; n < limit; n++) {
    const { start, end } = connections[n]!;
    // both single -> new circuit with both
    if (start.isSingle() && end.isSingle()) {
      start.circuit = end.circuit = circuits.push([start, end]) - 1;
      continue;
    }
    // start single, end in circuit -> add start to end's circuit
    if (start.isSingle() && !end.isSingle()) {
      start.circuit = end.circuit;
      circuits[start.circuit]!.push(start);
      continue;
    }
    // start in circuit, end single -> add end to start's circuit
    if (!start.isSingle() && end.isSingle()) {
      end.circuit = start.circuit;
      circuits[start.circuit]!.push(end);
      continue;
    }
    // both start and end in circuit -> merge end's circuit into start's circuit and empty end's circuit
    if (start.circuit === end.circuit) {
      continue;
    }
    circuits[start.circuit]! = [
      ...circuits[start.circuit]!,
      ...circuits[end.circuit]!,
    ];
    circuits[end.circuit]! = [];
    circuits[start.circuit]!.forEach((jb) => (jb.circuit = start.circuit));
  }

  const [a, b, c] = circuits.map((c) => c.length).sort((a, b) => b - a);
  return a! * b! * c!;
};

const part2 = async (): Promise<number> => {
  const junctionBoxes = await parsePositions();
  const connections = getConnections(junctionBoxes);
  const circuits: JunctionBox[][] = [];

  let n = 0;
  let { start, end } = connections[n]!;
  while (
    junctionBoxes.some((jb) => jb.circuit === -1) ||
    !junctionBoxes.every((jb) => jb.circuit === junctionBoxes[0]!.circuit)
  ) {
    ({ start, end } = connections[n]!);
    // both single -> new circuit with both
    if (start.isSingle() && end.isSingle()) {
      start.circuit = end.circuit = circuits.push([start, end]) - 1;
      n++;
      continue;
    }
    // start single, end in circuit -> add start to end's circuit
    if (start.isSingle() && !end.isSingle()) {
      start.circuit = end.circuit;
      circuits[start.circuit]!.push(start);
      n++;
      continue;
    }
    // start in circuit, end single -> add end to start's circuit
    if (!start.isSingle() && end.isSingle()) {
      end.circuit = start.circuit;
      circuits[start.circuit]!.push(end);
      n++;
      continue;
    }
    // both start and end in circuit -> merge end's circuit into start's circuit and empty end's circuit
    if (start.circuit === end.circuit) {
      n++;
      continue;
    }
    circuits[start.circuit]! = [
      ...circuits[start.circuit]!,
      ...circuits[end.circuit]!,
    ];
    circuits[end.circuit]! = [];
    circuits[start.circuit]!.forEach((jb) => (jb.circuit = start.circuit));
    n++;
  }

  return start.x * end.x;
};

console.log(`using ${TEST ? "example" : "puzzle"} input`);
console.log("part 1:", await part1());
console.log("part 2:", await part2());
