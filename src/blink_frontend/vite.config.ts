import { fileURLToPath, URL } from "node:url";
import environment from "vite-plugin-environment";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

import dotenv from "dotenv";

dotenv.config({ path: "../../.env" });

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    environment('all', { prefix: 'CANISTER_' }),
    environment('all', { prefix: 'DFX_' }),
  ],
  resolve: {
    alias: {
      "@declarations": fileURLToPath(new URL('../declarations/blink_backend', import.meta.url)),
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  server: {
    host: "0.0.0.0",
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:4943',
        changeOrigin: true,
      },
    },
  },
});
