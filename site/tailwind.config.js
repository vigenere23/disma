const defaultTheme = require("tailwindcss/defaultTheme")

/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: ["./src/**/*.{tsx,css}", "./index.html"],
  theme: {
    extend: {
      colors: {
        dark: {
          1: "#272C35",
          2: "#323843",
          3: "#3D4452",
          4: "#485061",
        },
        light: {
          1: "#F9F8FB",
          2: "#EAECF0",
          3: "#DCDFE5",
          4: "#CDD1DA",
        },
      },
      fontFamily: {
        heading: ["Kanit", ...defaultTheme.fontFamily.sans],
        sans: ["Inter", ...defaultTheme.fontFamily.sans],
        mono: ["'Fira Code'", ...defaultTheme.fontFamily.mono],
      },
    },
  },
  plugins: [],
}
