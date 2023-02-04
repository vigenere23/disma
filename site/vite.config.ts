import { defineConfig } from "vite"
import solidPlugin from "vite-plugin-solid"
import { prismjsPlugin } from "vite-plugin-prismjs"

export default defineConfig({
  plugins: [
    solidPlugin(),
    prismjsPlugin({
      languages: ["rust", "handlebars", "bash", "yaml"],
    }),
  ],
  server: {
    port: 3000,
  },
  build: {
    target: "esnext",
  },
})
