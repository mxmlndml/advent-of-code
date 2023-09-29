import { exit } from "process";

const day = Bun.argv[2]?.padStart(2, "0");

if (day === undefined) {
    console.log("Enter the day you want to execute, eg: bun run . 1");
    exit(1);
}

const proc = Bun.spawn(["bun", "run", "index.ts"], {
    cwd: `./${day}/`,
});


const decoder = new TextDecoder();
for await (const chunk of proc.stdout) {
    process.stdout.write(decoder.decode(chunk));
}
