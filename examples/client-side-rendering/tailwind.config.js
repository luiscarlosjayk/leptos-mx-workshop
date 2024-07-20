export default {
  content: [
    "*.html",
    "./src/**/*.rs",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        mono: ['IBM Plex Mono', 'monospace'],
      },
      keyframes: {
        'grow-progress': {
          from: { transform: 'scaleX(0)' },
          to: { transform: 'scaleX(1)' },
        },
      },
      animation: {
        'grow-progress': 'grow-progress auto linear',
      },
      width: {
        svg: '30px',
      },
      height: {
        svg: 'auto',
      },
      dropShadow: {
        '3xl': '0 35px 35px rgba(0, 0, 0, 0.25)',
        '4xl': [
          '0 35px 35px rgba(0, 0, 0, 0.25)',
          '0 45px 65px rgba(0, 0, 0, 0.15)'
        ],
      },
      boxShadow: {
        '3xl': '0 35px 60px -15px rgba(0, 0, 0, 0.3)',
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'), // https://tailwindcss.com/docs/typography-plugin
    require('@tailwindcss/forms'), // https://github.com/tailwindlabs/tailwindcss-forms
    function ({ addUtilities }) { // Sets scrollbar width to zero
      const newUtilities = {
        '.scrollbar-zero-width': {
          /* Firefox */
          'scrollbar-width': 'none',
          /* Safari and Chrome */
          '&::-webkit-scrollbar': {
            width: '0px'
          },
        }
      }
      addUtilities(newUtilities);
    },
  ],
}