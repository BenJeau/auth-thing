import { test as teardown } from "@playwright/test";
import { spawn } from "child_process";

import { ServerConfig, config } from "./config";

teardown("stop Rust servers", async () => {
  await Promise.all(config.map((server) => stopServer(server)));
});

const stopServer = async (config: ServerConfig) => {
  const pid: string = await new Promise((resolve) => {
    const child = spawn("lsof", ["-t", `-i:${config.port}`]);
    let output = "";
    child.stdout.on("data", (data) => (output += data));
    child.on("close", () => resolve(output.trim()));
  });
  pid.split(/\s/).forEach((p) => spawn("kill", ["-9", p.trim()]));
  spawn("make", ["delete-db", `DB_FILE=${config.databaseFile}`], {
    cwd: "./backend",
  });
};
