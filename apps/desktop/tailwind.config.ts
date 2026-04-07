import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{vue,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        app: {
          bg: "var(--color-bg)",
          panel: "var(--color-panel)",
          line: "var(--color-line)",
          text: "var(--color-text)",
          muted: "var(--color-muted)",
          accent: "var(--color-accent)",
          "accent-soft": "var(--color-accent-soft)",
          danger: "var(--color-danger)",
        },
      },
      fontFamily: {
        sans: [
          "SF Pro Text",
          "SF Pro Display",
          "Segoe UI",
          "system-ui",
          "sans-serif",
        ],
      },
    },
  },
  plugins: [],
} satisfies Config;
