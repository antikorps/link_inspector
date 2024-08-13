import { defineConfig } from "astro/config";

import tailwind from "@astrojs/tailwind";
import viteConfig from "./vite.config.js";

// https://astro.build/config
export default defineConfig({
  integrations: [tailwind()],
  i18n: {
    defaultLocale: "es",
    locales: ["es", "en", "fr"],
  },
  vite: viteConfig,
});
