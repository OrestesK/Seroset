import type { Config } from "tailwindcss";

const config: Config = {
  mode: "jit",
  content: ["./src/app/**/*.{ts,tsx}"],
  purge: ["./src/app/**/*.{ts,tsx}"],
  theme: {
    exptend: {
      colors: {
        primary: "#262626",
        secondary: "#505050",
      },
    },
  },
};
export default config;
