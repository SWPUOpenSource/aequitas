import { defineConfig } from "@farmfe/core";
import vue from "@vitejs/plugin-vue";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  vitePlugins: [vue(), wasm(), topLevelAwait()],
});
