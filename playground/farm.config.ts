import { defineConfig } from "@farmfe/core";
import vue from "@vitejs/plugin-vue";
import wasm from "@farmfe/plugin-wasm";

export default defineConfig({
  plugins: [wasm()],
  vitePlugins: [vue()],
  compilation: {
    presetEnv: false,
  }
});
