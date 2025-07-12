import { defineConfig } from "vite"
import vue from "@vitejs/plugin-vue"
import AutoImport from "unplugin-auto-import/vite"
import Components from "unplugin-vue-components/vite"
import { ElementPlusResolver } from "unplugin-vue-components/resolvers"

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  build: {
    target: "esnext",
  },
  plugins: [
    vue(),
    AutoImport({
      imports: ["vue", "@vueuse/core"],
      dirs: ["./src/script/util.ts"],
      resolvers: [ElementPlusResolver()],
    }),
    Components({
      resolvers: [ElementPlusResolver()],
    }),
  ],
  resolve: {
    alias: {
      "@": "/src",
    },
  },
  css: {},
  envPrefix: ["TMUS_"],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**", "**/tmus-engine/**"],
    },
  },
}))
