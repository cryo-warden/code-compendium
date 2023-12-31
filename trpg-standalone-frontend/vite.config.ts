import { defineConfig } from "vite";
import preact from "@preact/preset-vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [preact()],
  resolve: {
    alias: {
      components: "/src/components",
      demo: "/src/demo",
      GameEngine: "/src/GameEngine",
      general: "/src/general",
      images: "/src/images",
    },
  },
});
