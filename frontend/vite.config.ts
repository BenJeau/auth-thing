import path from "path";
import react from "@vitejs/plugin-react-swc";
import { TanStackRouterVite } from "@tanstack/router-plugin/vite";
import { defineConfig } from "vite";
// @ts-expect-error - no TS types for beta test
import reactCompiler from "babel-plugin-react-compiler";

export default defineConfig({
  plugins: [
    [reactCompiler],
    react(),
    TanStackRouterVite({
      quoteStyle: "double",
      semicolons: true,
      routesDirectory: "./src/routes",
      generatedRouteTree: "./src/navigation/routeTree.gen.ts",
    }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ["react", "react-dom"],
          tanstack: [
            "@tanstack/react-query",
            "@tanstack/react-router",
            "@tanstack/react-table",
            "@tanstack/react-form",
          ],
        },
        chunkFileNames: "js/[name]-[hash].js",
        entryFileNames: "js/[name]-[hash].js",
        assetFileNames: "[ext]/[name]-[hash].[ext]",
      },
    },
  },
});
