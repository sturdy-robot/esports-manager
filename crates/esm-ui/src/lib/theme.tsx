import { useEffect, type ReactNode } from "react";
import { ThemeContext, type Theme } from "./theme-context";

export function ThemeProvider({ children }: { children: ReactNode }) {
  const theme: Theme = "dark";

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("esm-theme", theme);
  }, [theme]);

  const toggleTheme = () => {
    return;
  };

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}
