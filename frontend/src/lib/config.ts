declare global {
  interface Window {
    _env_?: ImportMetaEnv;
  }
}

const development = {
  rest_server_base_url:
    import.meta.env.VITE_REST_SERVER_BASE_URL ?? "http://localhost:3456/api/v1",
  admin_email: import.meta.env.VITE_ADMIN_EMAIL ?? "admin@localhost",
  sentry_dsn: import.meta.env.VITE_SENTRY_DSN ?? undefined,
  sentry_tunnel_path: import.meta.env.VITE_SENTRY_TUNNEL_PATH ?? "/sentry",
  commit_sha: "dev",
  version: "v0.0.0",
  domain: "testing.dev",
  name: "Auth Thing",
};

const production = {
  rest_server_base_url: window._env_?.VITE_REST_SERVER_BASE_URL ?? "/api/v1",
  admin_email: window._env_?.VITE_ADMIN_EMAIL ?? "",
  sentry_dsn: window._env_?.VITE_SENTRY_DSN ?? undefined,
  sentry_tunnel_path: window._env_?.VITE_SENTRY_TUNNEL_PATH ?? undefined,
  commit_sha: import.meta.env.VITE_COMMIT_SHA,
  version: import.meta.env.VITE_VERSION,
  domain: import.meta.env.VITE_DOMAIN,
  name: development.name,
};

export default (() => (import.meta.env.PROD ? production : development))();
