export interface ServerConfig {
  frontendPort: number;
  port: number;
  seeds: string[];
  databaseFile: string;
}

export const config = [
  {
    frontendPort: 3000,
    port: 3456,
    seeds: ["data.sql"],
    databaseFile: "e2e_normal.db",
  },
  {
    frontendPort: 3001,
    port: 3457,
    seeds: ["e2e/no_signup.sql"],
    databaseFile: "e2e_no_signup.db",
  },
] as const satisfies ServerConfig[];
