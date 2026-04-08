import "vuetify/styles";
import "@mdi/font/css/materialdesignicons.css";
import { createVuetify } from "vuetify";

export const vuetify = createVuetify({
  icons: {
    defaultSet: "mdi",
  },
  theme: {
    defaultTheme: "light",
    themes: {
      light: {
        dark: false,
        colors: {
          background: "#f3f5f7",
          surface: "#ffffff",
          primary: "#356bb8",
          secondary: "#5b6b7f",
          error: "#d32f2f",
        },
      },
      dark: {
        dark: true,
        colors: {
          background: "#111318",
          surface: "#1a1e25",
          primary: "#5f8fdb",
          secondary: "#93a1b4",
          error: "#ef5350",
        },
      },
    },
  },
});
