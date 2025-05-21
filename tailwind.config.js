/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/@skeletonlabs/skeleton/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {}
  },

  plugins: [
    require('@tailwindcss/forms'),
    require('@skeletonlabs/skeleton/tailwind/skeleton.cjs')
  ],

  // This enables the data-mode attribute for dark mode
  darkMode: ['class', '[data-mode="dark"]']
}; 