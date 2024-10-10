/** @type {import('tailwindcss').Config} */
import daisyui from "daisyui";
import typography from "@tailwindcss/typography";

module.exports = {
    content: ["./components/**/*.{html,hbs}"],
    theme: {
        extend: {},
    },
    plugins: [typography, daisyui],
}

