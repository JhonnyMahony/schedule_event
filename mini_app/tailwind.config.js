/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    screens: {
      sm: '576px',
      md: '768px',
      lg: '992px',
      xl: '1200px',
    },
    container: {
      center: true,
      padding: '1rem',
    },
    extend: {
      zIndex: {
        '100': '100',
      },
      colors: {
        primary: '#0275d8'
      },
      fontFamily:{
        poppins:  "'Poppins', sans-serif",
        roboto:  "'Roboto', sans-serif",
      }
    },
  },
  variants: {
    extend: {
      visibility: ['group-hover'],
      display: ['group-hover']
    },
  },
  darkMode: "class",
  plugins: [
    require('@tailwindcss/forms')
  ],
}

