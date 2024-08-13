import { defineConfig } from "vite";

export default defineConfig({
  server: {
    proxy: {
      "/upload": {
        target: "http://localhost:3000",
        changeOrigin: true,
      },
    },
  },
});
