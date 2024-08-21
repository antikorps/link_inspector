import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";
import viteConfig from "./vite.config.js";
import react from "@astrojs/react";
import compressor from "astro-compressor";

import alpinejs from "@astrojs/alpinejs";

// https://astro.build/config
export default defineConfig({
  integrations: [tailwind(), react(), compressor({
    gzip: true,
    brotli: false
  }), alpinejs()],
  i18n: {
    defaultLocale: "es",
    locales: ["es", "en", "fr"]
  },
  vite: viteConfig
});