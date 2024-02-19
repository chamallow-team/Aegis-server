const { rose, fuchsia, gray } = require('tailwindcss/colors.js');

module.exports = {
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: "class",
  theme: {
    extend: {},
    screens: {
      sm: '480px',
      md: '768px',
      lg: '976px',
      xl: '1440px',
    },
    colors: {
      blue: { 100: "#759aab", 50: "rgba(117,154,171,0.5)" },
      gray: gray,
      red: rose,
      pink: fuchsia,
    },
    fontFamily: {
      raleway: ["Raleway", 'sans-serif']
    }
  },
  variants: {
    extend: {},
  },
  plugins: [],
}