import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import path from "node:path";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), wasm(), topLevelAwait()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
});
