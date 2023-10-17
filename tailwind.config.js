/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: [
      "*.html",
      "./src/**/*.rs",
      "./partials/**/*.rs",
      "./components/**/*.rs",
    ],
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
