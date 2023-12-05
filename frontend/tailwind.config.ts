import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.ts",
    "./src/components/**/*.ts",
    "./src/app/**/*.ts",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
export default config;
