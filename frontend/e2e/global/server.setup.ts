import { test as setup } from "@playwright/test";
import { spawn } from "child_process";

import { ServerConfig, config } from "./config";

setup("run Rust servers", async () => {
  await Promise.all(config.map((server) => runServer(server)));
});

const runServer = async (config: ServerConfig) => {
  const subprocess = spawn(
    "make",
    [
      "reset-db",
      `DB_FILE=${config.databaseFile}`,
      `SEED_FILES=${config.seeds.join(",")}`,
    ],
    {
      cwd: "./backend",
      env: {
        ...process.env,
        SQLX_OFFLINE: "true",
      },
    },
  );

  await new Promise((resolve) => {
    subprocess.on("close", resolve);
  });

  const serverSubprocess = spawn("cargo", ["run", "--bin", "server"], {
    cwd: "./backend",
    env: {
      ...process.env,
      AUTH__DATABASE__URL: "sqlite://./database/" + config.databaseFile,
      AUTH__SERVER__PORT: config.port.toString(),
      DATABASE_URL: "sqlite://./database/" + config.databaseFile,
    },
  });

  serverSubprocess.on("error", (error) => {
    console.error("Error running Rust server", error);
  });

  while (true) {
    try {
      const healthCheck = await fetch(
        `http://localhost:${config.port}/api/v1/health`,
      );
      if (healthCheck.ok) {
        break;
      }
    } catch (e) {}
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
};
